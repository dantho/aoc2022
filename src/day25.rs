/// https://adventofcode.com/2022/day/25
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

    const EX1: &'static str = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
}
