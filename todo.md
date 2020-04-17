From profiling, we see that the PQ is much slower than 'without'. This means that using the PQ to support making 'without' concurrent is silly.

What we want to do is be multi-threaded without PQ. We can try "depth first" search (like we do now, finding everything possible with the first / funniest word) or "breadth first" (prioritizing anagrams with few words).

We also see that cloning a hash is slow. We think that we could speed that up by instead using a vector of numbers, eg 1 = a, 2 = b, etc, for all characters we want to support / are found in our words.


To profile (from https://carol-nichols.com/2017/04/20/rust-profiling-with-dtrace-on-osx/):

- `cargo build --release`
- `sudo dtrace -c './target/release/swappy nathanhaslicemyfr' -o out.stacks -n 'profile-997 /execname == "swappy"/ { @[ustack(100)] = count(); }'`
- `./stackcollapse.pl ~/code/mine/rust_experiments/swappy/out.stacks | ./flamegraph.pl > pretty-graph2.svg`
