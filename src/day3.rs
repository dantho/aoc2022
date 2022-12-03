/// https://adventofcode.com/2022/day/2
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786 
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754 
/// 
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax 
// extern crate regex;
// use self::regex::{Captures, Regex};

use std::{str::FromStr, collections::HashMap};

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day3, part1)]
pub fn gen1(input: &str) -> Vec<String> {
    input.lines().map(|s|s.to_owned()).collect()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day3, part1)]
pub fn part1(input: &[String]) -> u32 {
    let priority = ('a'..='z').chain('A'..='Z').zip(1..).collect::<HashMap<char,u32>>();
    input.iter()
    .map(|line|{
        let len = line.len();
        let sac1 = &line[..len/2];
        let sac2 = &line[len/2..];
        assert_eq!(sac1.len(), sac2.len());
        let mut found: char = '-';
        for c in sac1.chars() {
            if sac2.contains(c) {
                found = c;
                break;
            }
        }
        assert_ne!(found, '-');
        priority[&found]
    }).sum()
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 15);
    }

    #[test]
    fn test_ex1_part2() {
        assert_eq!(part2(&gen2(EX1)), 12);
    }

const EX1: &'static str =
r"A Y
B X
C Z";

}