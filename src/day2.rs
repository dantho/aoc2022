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
#[derive(Debug)]
pub enum RockPaperScissors {
    Rock,
    Paper,
    Scissors
}

impl RockPaperScissors {
    fn ToU32(self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3
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

#[aoc_generator(day2)]
pub fn gen1(input: &str) -> Vec<(RockPaperScissors,RockPaperScissors)> {
    input.lines()
    .map(|line| {
        let mut play = line.split(' ').map(|p|p.parse::<RockPaperScissors>().unwrap());
        (play.next().unwrap(), play.next().unwrap())
    }).collect()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day2, part1)]
pub fn part1(input: &[(RockPaperScissors,RockPaperScissors)]) -> u32 {
    0
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        let gen = gen1(EX1);
        println!("{:?}", gen);
        assert_eq!(part1(&gen1(EX1)), 24000);
    }

const EX1: &'static str =
r"A Y
B X
C Z";

}