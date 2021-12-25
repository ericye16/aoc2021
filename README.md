# AoC 2021 Solutions in Rust
(c) Eric Ye, also (c) Google LLC

Most solutions are decently fast but there are some optimization opportunities:
- Day 7: The solution space for part 2 could be reduced thanks to some math. 
  However, it already runs so fast I don't think it's worth it.
- Day 19: This uses a sort-of brute-force solution. I could reduce the search
  and make it faster by a significant amount if I e.g. sorted by pairwise
  distance between beacons. Currently runs in about 11.8 seconds on my MacBook
  Pro.
- Day 22: This is the slowest of the bunch. This compresses the space by only
  using grid indices that show up in the inputs, but that still takes ~40
  seconds and ~15GB of memory on my MacBook Pro. There should be a solution
  using the [inclusion-exclusion principle](https://en.wikipedia.org/wiki/Inclusion–exclusion_principle)
  which would solve this in a much shorter amount of time and with less memory.
- Day 23: The solution takes 27 seconds for parts 1 and 2 combined and takes
  ~10 GB of memory. I'm not sure how much I could improve this, actually.
  I know others have must faster solutions though.
- ~~Day 25: I could make a visualization.~~
