# AoC 2021 Solutions in Rust
(c) Eric Ye, also (c) Google LLC

Most solutions are decently fast but there are some optimization opportunities:
- Day 7: The solution space for part 2 could be reduced thanks to some math. 
  However, it already runs so fast I don't think it's worth it.
- Day 19: Rust solution v1 uses a sort-of brute-force solution. I could reduce
  the search and make it faster by a significant amount if I e.g. sorted by pairwise
  distance between beacons. Currently runs in about 11.8 seconds on my MacBook
  Pro. I'd like to do this in C++ but apparently getting Eigen to work with
  Bazel is non-trivial. I re-wrote it in Rust and used linear algebra routines
  to find a faster solution. Down from ~12 seconds to 250ms, still slower than
  I'd want but much better.
- Day 22: Rust solution is the slowest of the bunch. This compresses the space by only using grid indices that show up in the inputs, but that still takes ~40 seconds and ~15GB of memory on my MacBook Pro. There should be a solution using the [inclusion-exclusion principle](https://en.wikipedia.org/wiki/Inclusion–exclusion_principle) which would solve this in a much shorter amount of time and with less memory. Optimized in C++.
- Day 23: Rust solution takes 27 seconds for parts 1 and 2 combined and takes
  ~10 GB of memory. I'm not sure how much I could improve this, actually. C++ improves this significant to 250ms for both parts.
  I know others have must faster solutions though.
- Day 25: I could make a visualization. Done.

## Running

### Rust solutions
Install Cargo, then run `cargo run --release -p d<day of problem>`. For example, for day 25, run `cargo run --release -p d25`.

### C++ Solutions
Install Bazel, then run `bazel run -c opt //cpp:p<day of problem>`. For example, for day 23, run `bazel run -c opt //cpp:p23`. Note only P1, P22 and P23 are implemented in C++.
