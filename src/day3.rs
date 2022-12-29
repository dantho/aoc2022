/// https://adventofcode.com/2022/day/3
/// DAN AoC: https://adventofcode.com/2022/leaderboard/private/view/380786
/// HLOTYAK: https://adventofcode.com/2022/leaderboard/private/view/951754

use std::collections::HashMap;

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day3)]
pub fn gen1(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_owned()).collect()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day3, part1)]
pub fn part1(input: &[String]) -> u32 {
    let priority = ('a'..='z')
        .chain('A'..='Z')
        .zip(1..)
        .collect::<HashMap<char, u32>>();
    input
        .iter()
        .map(|line| {
            let len = line.len();
            let sac1 = &line[..len / 2];
            let sac2 = &line[len / 2..];
            assert_eq!(sac1.len(), sac2.len());
            let mut found: char = '-';
            for c in sac1.chars() {
                if sac2.contains(c) {
                    if found != '-' && found != c {
                        panic!("Found '{}' after '{}'", c, found);
                    }
                    found = c;
                }
            }
            assert_ne!(found, '-');
            // println!("{} ({})", priority[&found], found);
            priority[&found]
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &[String]) -> u32 {
    let priority = ('a'..='z')
        .chain('A'..='Z')
        .zip(1..)
        .collect::<HashMap<char, u32>>();
    input
        .iter()
        .step_by(3)
        .zip(
            input
                .iter()
                .skip(1)
                .step_by(3)
                .zip(input.iter().skip(2).step_by(3)),
        )
        .map(|(e1, (e2, e3))| {
            let mut found: char = '-';
            let mut maybe = String::new();
            for c in e1.chars() {
                if e2.contains(c) {
                    maybe.push(c)
                };
            }
            for c in maybe.chars() {
                if e3.contains(c) {
                    if found != '-' && found != c {
                        panic!("Found '{}' after '{}'", c, found);
                    }
                    found = c;
                }
            }
            assert_ne!(found, '-');
            // println!("{} ({})", priority[&found], found);
            priority[&found]
        })
        .sum()
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 157);
    }

    #[test]
    fn test_ex1_part2() {
        assert_eq!(part2(&gen1(EX1)), 70);
    }

    const EX1: &'static str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
}
