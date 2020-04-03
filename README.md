# Swappy (experimental)

Programmatically produce anagrams of a given input phrase using a given dictionary.

## Design Goals

- Because a phrase can have millions of anagrams, be able to produce the most interesting subset of them first and stop early.
- Possibly allow resuming later - serialize all data structures for rehydration
- Produced anagrams should be deterministic - given same list, if we say "find the first 10", we should get the same list every time.
- "Most interesting" = "containing words listed earlier in the user-specified dictionary". So the dictionary should be sorted most-to-least interesting. This could be simple (eg, longest to shortest) or geared toward certain results (eg, funniest words at the top).
- Be as efficient and concurrent as possible

## Observations

- If a dictionary word's alphagram can't be found in the entire input phrase, it will definitely never be found in any subset of it
- If we find word `N` in the input phrase, we should only check for words `> N` in the remainder of the phrase, so that we avoid finding (eg) both `[2, 3]` and `[3, 2`] (and the longer the chain, the more duplicate variations there could be)

## Rough plan

Data sources / structures:

- The input dictionary, unfiltered, streaming from a file
- An indexed "candidate words" list of dictionary words which are subsets of the input phrase
- A priority queue of "candidate anagrams", both partial and just-completed
  - Priority criteria:
    - `[1]` is higher than `[2]`, and `[1,2]` is higher than `[1,3]`, etc.
    - We want to approximate "depth first" (but concurrent), so `[1,2]` is higher than `[1]`, and `[1]` is higher than `[]`
- A list of finalized anagrams
- A `BATCH_SIZE` integer
- A `THREAD_COUNT` integer

A "candidate anagram" looks like: `CandidateAnagram{found_words: Vec<String>, remaining_chars: Vec<Char>, priority: CandidatePriority{i: Vec<i32>}, next_candidate_word: u32}`

Steps:

- Convert the input phrase to an alphagram.
- Create the first candidate anagram and drop it in the priority queue. It looks like this:
  - `priority` is `[]`
  - `found_words` is an empty vec
  - `remaining_chars` has the full input alphagram
  - `next_candidate_word` is `0`
- main loop: 
  - Check length of finalized anagrams. If we have enough, break and return them
  - Pull a `CandidateAnagram` from the priority queue
    - If there isn't anything, break and return the list of finalized anagrams (we couldn't find enough)
    - If it's a completed anagram, add it to the list of finalized anagrams and restart the main loop
    - If it's a partial anagram, look at its `next_candidate_word`. Suppose it's `0`. Suppose `THREAD_COUNT` is `3` and `BATCH_SIZE` is `10`. That means we'd like to divvy up candidate words `next_candidate_word + (THREAD_COUNT * BATCH_SIZE)` among our threads.
    - If index `next_candidate_word + (THREAD_COUNT * BATCH_SIZE)` doesn't exist in `candidate_words`, pull from the dictionary file and add to `candidate_words` until it does or until we reach the end of the dictionary file. If we reach the end of the dictionary file first, set `DICTIONARY_EXHAUSTED=true`
    - spin up `THREAD_COUNT` threads and let each take a batch of `BATCH_SIZE`, based on the `next_candidate_word`. Eg, if `next_candidate_word` is `0`, `BATCH_SIZE` is 10 and `THREAD_COUNT` is `3`, one thread would try out `candidate_words` `0-9`, the next would try `10-19`, and the next `20-29`. (NOTE: if the max `candidate_words` index is `18`, only spin up 2 threads because we only have 2 batches left to do.)
    - For each candidate word in its batch, the thread will do as follows:
      - If the word can't be taken from `remaining_chars`, it will be ignored
      - If the word can be taken from `remaining_chars`, the thread will create a new `CandidateAnagram` with that word added to the `found_words`, its alphagram removed from `remaining_chars`, and `next_candidate_word` set to `(candidate word's index) + 0` (in case we find apple apple)
    - Wait for all threads to finish.
    - Add all the new `CandidateAnagrams` to the priority queue
    - if `DICTIONARY_EXHAUSTED`, discard the `CandidateAnagram` we pulled initially; otherwise, update it to have `next_candidate_word=30` and put it back in the priority queue.
