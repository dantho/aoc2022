/// https://adventofcode.com/2022/day/6
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
#[aoc_generator(day6)]
pub fn gen1(input: &str) -> String {
    input.to_string()
}

// *********************
// *** Part1 & Part2 ***
// *********************

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    find_no_dups(4, input).unwrap()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    find_no_dups(14, input).unwrap()
}

fn find_no_dups(window_size: usize, input: &str) -> Option<usize> {
    let array: Vec<_> = input.chars().collect();
    array
        .windows(window_size)
        .enumerate()
        .fold(None, |found, (ndx, chars)| match found {
            Some(f) => Some(f),
            None => {
                let mut found = Some(ndx + window_size); // Optimistic default
                for c in chars {
                    if chars.iter().filter(|c2| &c == c2).count() > 1 {
                        found = None;
                        break;
                    }
                }
                found
            }
        })
}
// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        assert_eq!(part1(&gen1(EX1)), 7);
    }
    #[test]
    fn test_part1_ex2() {
        assert_eq!(part1(&gen1(EX2)), 5);
    }
    #[test]
    fn test_part1_ex3() {
        assert_eq!(part1(&gen1(EX3)), 6);
    }
    #[test]
    fn test_part1_ex4() {
        assert_eq!(part1(&gen1(EX4)), 10);
    }
    #[test]
    fn test_part1_ex5() {
        assert_eq!(part1(&gen1(EX5)), 11);
    }

    #[test]
    fn test_part2_ex1() {
        assert_eq!(part2(&gen1(EX1)), 19);
    }
    #[test]
    fn test_part2_ex2() {
        assert_eq!(part2(&gen1(EX2)), 23);
    }
    #[test]
    fn test_part2_ex3() {
        assert_eq!(part2(&gen1(EX3)), 23);
    }
    #[test]
    fn test_part2_ex4() {
        assert_eq!(part2(&gen1(EX4)), 29);
    }
    #[test]
    fn test_part2_ex5() {
        assert_eq!(part2(&gen1(EX5)), 26);
    }

    const EX1: &'static str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const EX2: &'static str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EX3: &'static str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EX4: &'static str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EX5: &'static str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
}
