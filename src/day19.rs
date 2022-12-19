/// https://adventofcode.com/2022/day/19
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
#[aoc_generator(day19)]
pub fn gen1(input: &str) -> Vec<Vec<isize>> {
    input.lines()
        .map(|line| parse_numbers(line))
        .collect::<Vec<_>>()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day19, part1)]
pub fn part1(input: &[Vec<isize>]) -> usize {
    input.len() * input[0].len()
}

// #[aoc(day19, part2)]
// pub fn part2(input: &Vec<Vec<u32>>) -> u32 {
//     999
// }

fn parse_numbers(s: &str) -> Vec<isize> {
    let just_nums: String = s.chars()
    .map(|c| if c.is_numeric() || c == '-' {c} else {' '}).collect();
    just_nums.trim().split(' ')
        .filter(|s|!s.is_empty())
        .map(|s|s.parse().unwrap()).collect()
}
// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers() {
        assert_eq!(parse_numbers("xyz123abc456def"),[123,456]);
        assert_eq!(parse_numbers("xyz123abc-456def"),[123,-456]);
        assert_eq!(parse_numbers("123 456"),[123,456]);
        assert_eq!(parse_numbers("&123:-456#"),[123,-456]);

        assert_eq!(parse_numbers("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian."),
            [1,4,2,3,14,2,7]);

        assert_eq!(parse_numbers("humanoid robot"),[]);
    }

    #[test]
    #[should_panic]
    fn test_parse_numbers_solitarydash() {
        // Not yet implemented
        assert_eq!(parse_numbers("humanoid-robot"),[]);
    }
    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 24000);
    }

    // #[test]
    // fn test_ex1_part2() {
    //     assert_eq!(part2(&gen1(EX1)), 45000);
    // }

    const EX1: &'static str = 
r"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian. 
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

}
