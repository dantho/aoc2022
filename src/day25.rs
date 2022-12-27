/// https://adventofcode.com/2022/day/25
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754
///
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax
// extern crate regex;
// use self::regex::{Captures, Regex};

use std::fmt::Display;
use crate::day25::Snafu::*;

#[derive(Debug, Clone)]
pub struct SnafuNum {
    snafs: Vec<Snafu>,
}

impl Display for SnafuNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut num = String::new();
        for dig in &self.snafs {
            num.push(dig.to_char());
        }
        write!(f,"{}",num)?;
        Ok(())
    }
}

impl From<&str> for SnafuNum {
    fn from(s: &str) -> Self {
        let mut num = Vec::new();
        for c in s.chars() {
            num.push(Snafu::from(c));
        }
        SnafuNum { snafs: num }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Snafu {
    Two = 2,
    One = 1,
    Zero = 0,
    MinusOne = -1,
    MinusTwo = -2
}

impl Snafu {
    fn to_char(&self) -> char {
        match self {
            Two => '2',
            One => '1',
            Zero => '0',
            MinusOne => '-',
            MinusTwo => '=',
        }
    }
}

impl From<char> for Snafu {
    fn from(c: char) -> Self {
        match c {
            '2' => Two,
            '1' => One,
            '0' => Zero,
            '-' => MinusOne,
            '=' => MinusTwo,
            bad => panic!("Bad Snafu char: '{}'", bad)
        }
    }
}

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day25)]
pub fn gen1(input: &str) -> Vec<Vec<u32>> {
    let mut elf = Vec::<u32>::new();
    let mut elves = Vec::new();
    for line in input.lines() {
        if line == "" {
            elves.push(elf);
            elf = Vec::new();
        } else {
            elf.push(line.parse().unwrap());
        }
    }
    elves.push(elf);
    elves
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day25, part1)]
pub fn part1(input: &Vec<Vec<u32>>) -> u32 {
    input
        .iter()
        .map(|elf| elf.iter().fold(0, |sum, item| sum + item))
        .max()
        .unwrap()
}

#[aoc(day25, part2)]
pub fn part2(input: &Vec<Vec<u32>>) -> u32 {
    let mut calories: Vec<u32> = input
        .iter()
        .map(|elf| elf.iter().fold(0, |sum, item| sum + item))
        .collect();
    calories.sort();
    calories.reverse();
    calories.iter().take(3).sum()
}

// *************
// *** Tests ***
// *************
#[test]
fn test_SNAFU() {
    //         Decimal     SNAFU
    assert_eq!(1        .  SnafuNum::from("1").to_decimal);
    assert_eq!(2        .  SnafuNum::from("2").to_decimal);
    assert_eq!(3        .  SnafuNum::from("1=").to_decimal);
    assert_eq!(4        .  SnafuNum::from("1-").to_decimal);
    assert_eq!(5        .  SnafuNum::from("10").to_decimal);
    assert_eq!(6        .  SnafuNum::from("11").to_decimal);
    assert_eq!(7        .  SnafuNum::from("12").to_decimal);
    assert_eq!(8        .  SnafuNum::from("2=").to_decimal);
    assert_eq!(9        .  SnafuNum::from("2-").to_decimal);
    assert_eq!(10       .  SnafuNum::from("20").to_decimal);
    assert_eq!(15       .  SnafuNum::from("1=0").to_decimal);
    assert_eq!(20       .  SnafuNum::from("1-0").to_decimal);
    assert_eq!(2022     .  SnafuNum::from("1=11-2").to_decimal);
    assert_eq!(12345    .  SnafuNum::from("1-0---0").to_decimal);
    assert_eq!(314159265.  SnafuNum::from("1121-1110-1=0").to_decimal);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 24000);
    }

    #[test]
    fn test_ex1_part2() {
        assert_eq!(part2(&gen1(EX1)), 45000);
    }

    const EX1: &'static str = r"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
}
