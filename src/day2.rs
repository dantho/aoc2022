/// https://adventofcode.com/2022/day/2
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786 
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754 
/// 
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax 
// extern crate regex;
// use self::regex::{Captures, Regex};

use std::str::FromStr;

// ********************
// *** Generator(s) ***
// ********************/
#[derive(Clone, Copy, Debug)]
pub enum RockPaperScissors {
    Rock = 0,
    Paper,
    Scissors
}

impl RockPaperScissors {
    fn score(self, vs: RockPaperScissors) -> u32 {
        1 + self as u32 + match self {
            Self::Rock => match vs {
                Self::Rock => 3,
                Self::Paper => 0,
                Self::Scissors => 6
            }
            Self::Paper => match vs {
                Self::Rock => 6,
                Self::Paper => 3,
                Self::Scissors => 0
            }
            Self::Scissors => match vs {
                Self::Rock => 0,
                Self::Paper => 6,
                Self::Scissors => 3
            }
        }
    }
}
impl From<u32> for RockPaperScissors {
    fn from(v: u32) -> Self {
        match v {
            n if n == Self::Rock as u32 => Self::Rock,
            n if n == Self::Paper as u32 => Self::Paper,
            n if n == Self::Scissors as u32 => Self::Scissors,
            _ => panic!("Expecting value 0, 1, or 2")
        }
    }
}
impl ToString for RockPaperScissors {
    fn to_string(&self) -> String {
        match self {
            Self::Rock => "Rock",
            Self::Paper => "Paper",
            Self::Scissors => "Scissors"
        }.to_string()
    }
}

impl FromStr for RockPaperScissors {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            bad => Err(format!("Unexpected input play \"{}\"", bad))
        }
    }
}

#[aoc_generator(day2, part1)]
pub fn gen1(input: &str) -> Vec<(RockPaperScissors,RockPaperScissors)> {
    input.lines()
    .map(|line| {
        let mut play = line.split(' ').map(|p|p.parse::<RockPaperScissors>().unwrap());
        (play.next().unwrap(), play.next().unwrap())
    }).collect()
}

#[aoc_generator(day2, part2)]
pub fn gen2(input: &str) -> Vec<(RockPaperScissors,RockPaperScissors)> {
    input.lines()
    .map(|line| {
        let mut play = line.split(' ');
        let vs = play.next().unwrap().parse::<RockPaperScissors>().unwrap();
        let me = match play.next().unwrap() {
            // X means you need to lose,
            // Y means you need to end the round in a draw, and
            // Z means you need to win.
            // "+ 2" below is equivalent to -1 in mod 3 math
            "X" => RockPaperScissors::from((vs as u32 + 2) % 3),
            "Y" => vs,
            "Z" => RockPaperScissors::from((vs as u32 + 1) % 3),
            bad => panic!("Unexpected instructional play \"{}\"", bad)
        };
        (vs, me)
    }).collect()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day2, part1)]
pub fn part1(input: &[(RockPaperScissors,RockPaperScissors)]) -> u32 {
    let ans = input.iter().map(|pair|pair.1.score(pair.0)).sum();
    #[cfg(not(test))]
    assert!(ans > 11333);
    ans
}

#[aoc(day2, part2)]
pub fn part2(input: &[(RockPaperScissors,RockPaperScissors)]) -> u32 {
    let ans = input.iter().map(|pair|pair.1.score(pair.0)).sum();
    #[cfg(not(test))]
    assert!(ans > 11333);
    ans
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