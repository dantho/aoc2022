# Advent of Code 2022

![](pix/aoc.png)

Teradyne private [Leaderboard](https://adventofcode.com/2022/leaderboard/private/view/380786) for [adventofcode.com/2022](https://adventofcode.com/2022)

| Day | Stars | Comments (**SPOILER ALERT!**) |
|---|:-:|-|
| [1: Calorie Counting](https://adventofcode.com/2022/day/1) |⭐⭐| Sum Calories by elf, find max elf.  Add sorting to find sum of top 3 elves. |
| [2: Rock Paper Scissors](https://adventofcode.com/2022/day/2) |⭐⭐| Circular buffer for game logic |
| [3: Rucksack Reorganization](https://adventofcode.com/2022/day/3) |⭐⭐| Find dups, then find dups in groups |
| [4: Camp Cleanup](https://adventofcode.com/2022/day/4) |⭐⭐| Parsing input data _could have been done_ in about 10 minutes in VS Code.  Took more than 1 hour in code.  :( |
| [5: Supply Stacks](https://adventofcode.com/2022/day/5) |⭐⭐| Parsing two types of data was a pain, took much more time than algo did.  Crates section of input had to be transposed so _columns_ were vectors of crates instead of rows.  Used pop/push for part 1. Used temporary storage and vec.append() for part 2.  Didn't visualize.  :( |
| [6: Tuning Trouble](https://adventofcode.com/2022/day/6) |⭐⭐| Protocol-aware puzzle -- write start-of-packet marker detection routine |
| [7: No Space Left On Device](https://adventofcode.com/2022/day/7) |⭐⭐| Terminal output parsing for directory structure/navigation.  Tried/failed to use Trees crate.  Tracked directory sizes in HashMap with full path as key. |
| [8: Treetop Tree House](https://adventofcode.com/2022/day/8) |⭐⭐| Simple elevation gradient algos in 4 directions |
| [9: Rope Bridge](https://adventofcode.com/2022/day/9) |⭐⭐| Weird rope knot dynamics -- tried/failed to visualize with animation via CrossTerm::Cursor::MoveTo.  :( |
| [10: Cathode-Ray Tube](https://adventofcode.com/2022/day/10) |⭐⭐| *Very* simple ALU with 2 op codes. Part 2 prints pixels for the human to read. |
| [11: Monkey in the Middle](https://adventofcode.com/2022/day/11) |⭐⭐| Difficult puzzle for 3 reasons: 1) Input didn't lend itself to parsing.  Instead, I maniputlated input text into data structure syntax; 2) One piece of data was a _function_ -- a lambda function; 3) Had to figure out how to "limit" intermediates, without further instructions, just "figure it out". |
| [12: Hill Climbing Algorithm](https://adventofcode.com/2022/day/12) |⭐⭐|  |
| [13: Distress Signal](https://adventofcode.com/2022/day/13) |⭐⭐|  |
| [14: Regolith Reservoir](https://adventofcode.com/2022/day/14) |⭐⭐| Filling Fast Fun!  With sand!  References 2018's Day 17: Reservoir Research, but I rewrote from scratch.  Faster?  Animation opportunity. |
| [15: Beacon Exclusion Zone](https://adventofcode.com/2022/day/15) |⭐⭐| Brute force barely works in part 1. Part 2 was kinda brutal. |
| [16: Proboscidea Volcanium](https://adventofcode.com/2022/day/17) |  | HARD! Valves, flow rates, paths. Weighted Graph Algo, Maximum/Minimum Spanning Trees, greedy algorithms, all paths? Ugh. Skipping for now. |
| [17: Pyroclastic Flow](https://adventofcode.com/2022/day/17) |⭐⭐| Tetris with volcanic rocks! Part 2 is an annoying repetition detection thing. A spreadsheet helps.  Also helps to realize repetition implies, THE WHOLE OF THE DATASET will repeat, so ANY part of it also repeats. |
| [18: Boiling Boulders](https://adventofcode.com/2022/day/18) |⭐⭐| Finding air in 3-space. Part 2 is finding open air only. |
| [19: Not Enough Minerals](https://adventofcode.com/2022/day/19) || Robot factory processing Clay, Ore, Obsidian, Geode. |
| [20: Grove Positioning System](https://adventofcode.com/2022/day/20) || Seemingly simple unscramble. Anything but!! |
| [21: Monkey Math](https://adventofcode.com/2022/day/21) |  |  |
| [22: Monkey Map](https://adventofcode.com/2022/day/22) |⭐| Part 1 is weird with space on map.  Part 2 is brutal with the extra complexity folded in (pun intended). |
| [23: Unstable Diffusion](https://adventofcode.com/2022/day/23) ||  |
| [24: Blizzard Basin](https://adventofcode.com/2022/day/24) ||  |
| [25: ](https://adventofcode.com/2022/day/25) |  |  |