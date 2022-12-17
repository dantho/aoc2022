/// https://adventofcode.com/2022/day/17
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754
///
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax
// extern crate regex;
// use self::regex::{Captures, Regex};

const ROCKS: [&'static str; 5] = 
[r"####",
r".#.
###
.#.",
r"..#
..#
###",
r"#
#
#
#",
r"##
##"];

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day17)]
pub fn gen1(input: &str) -> String {
    input.to_string()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day17, part1)]
pub fn part1(input: &str) -> u32 {
    const ROCK_CNT: usize = 2022;

    999
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 3068);
    }

    // #[test]
    // fn test_ex1_part2() {
    //     assert_eq!(part2(&gen1(EX1)), 999);
    // }

    const EX1: &'static str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
}
