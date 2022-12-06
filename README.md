# Advent of Code 2021
[adventofcode.com/2022](https://adventofcode.com/2022)

[My Rust solutions](https://github.com/dantho/aoc2022) (This repo)

## Issues/Gotchas/Hints by day:

### Day 4:

Parsing input data _could have been done_ in about 10 minutes in VS Code.  Took more than 1 hour in code.  :(

### Day 5:

Parsing two types of data was a pain, took much more time that algo did.  Crates section of input had to be transposed so columns were vectors of crates whereas reading input line by line tends to make rows as vectors of crates.  Used pop/push for part 1. Used temporary storage and vec.append() for part 2.  Didn't visualize.  :(