/// https://adventofcode.com/2022/day/1
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786 
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754 
/// 
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax 
// extern crate regex;
// use self::regex::{Captures, Regex};

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day1)]
pub fn gen1(input: &str) -> Vec<Vec<u16>> {
    let mut elf = Vec::<u16>::new();
    let mut elves = Vec::new();
    for line in input.lines() {
        if line == "" {
            elves.push(elf);
            elf = Vec::new();
        } else {
            elf.push(line.parse().unwrap());
        }
    }
    elves
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day1, part1)]
pub fn part1(input: &Vec<Vec<u16>>) -> u16 {
    input.iter()
        .map(|elf|elf.iter()
            .fold(0,|sum,item|sum+item)).max().unwrap()
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(888, 999);
    }

    #[test]
    fn test_ex2_part2() {
        assert_eq!(888, 999);
    }

const EX1: &'static str =
r"
";

const EX2: &'static str =
r"
";

}