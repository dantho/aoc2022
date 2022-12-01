/// https://adventofcode.com/2021/day/N
/// ADI: https://adventofcode.com/2021/leaderboard/private/view/380786 
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 
/// 
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax 
extern crate regex;
// use self::regex::{Captures, Regex};

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day1)]
pub fn gen1(input: &str) -> Vec<u16> {
    input.lines()
        .map(|line|line.parse().unwrap())
        .collect()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day1, part1)]
pub fn part1(input: &Vec<u16>) -> usize {
    input.iter().zip(input.iter().skip(1))
        .filter(|(a,b)| b > a)
        .count()
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<u16>) -> usize {
    let input = input.iter()
        .zip(input.iter().skip(1))
        .zip(input.iter().skip(2))
        .map(|((a,b),c)|a+b+c)
        .collect();
    part1(&input)
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