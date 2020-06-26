# Swappy

Anagrams are useless but funny. Let's make some!

- "rust programming" => "grim gun port arms" or "asymmetry foe" or "tarring prom mugs"
- "rust anagrams library" => "triangular brass army" or "snag arbitrary murals"
- "software engineer" => "we are fingers o net" or "sea of wintergreen"
- "memory safety" => "me, my artsy foe" or "ye soft yammer" or "format my eyes"

Swappy produces anagrams of a given input phrase using a given word list file.

It prioritizes words that occur earlier in the word list, and lets you limit how many results are produced.
This lets you tailor what kinds of results you want (eg, by putting words you think are funny at the top, or sorting the list longest to shortest for impressive finds).
By default, it looks for a word list file at `~/.swappy_wordlist`, but you can set the `WORDLIST` env var to another file.
Swappy's repo contains [a wordlist file](https://gitlab.com/nathanl/swappy.rs/-/raw/master/test_support/smallish_list.txt) sorted longest to shortest, and with many short, uncommon words (like "oe" and "mu") removed for increased efficiency.

Usage:
   - `cargo build --release`
   - `target/release/swappy 'my phrase'`
   - `LIMIT=20 WORDLIST=/some/file.txt target/release/swappy 'my phrase'`

## Basic Strategy

Swappy performs a depth-first search of the tree of possible anagrams which use our word list and phrase.
Our tree could look like this, where a node is "found words / remaining letters".

- racecar /
  - race / car
      - race a / cr  [failed leaf]
      - race car /  [successful leaf]
  - craec / ar
      - racec a / r  [failed leaf]

Before deciding if a phrase contains a word, it converts them both to "alphagrams", which are order-agnostic; "bat" and "tab" have the same alphagram.
We represent alphagrams internally as a hashmap listing the count of each character.
This makes comparison and subtraction efficient.

Swappy is single-threaded, but performs well.
I considered a multi-threaded design, but could not find a good way to use multiple threads and still guarantee that results are produced in the order given in the word list.
I experimented with using a priority queue for this, but the queue was a significant performance bottleneck.
