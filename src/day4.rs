/// https://adventofcode.com/2022/day/4
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
#[aoc_generator(day4)]
pub fn gen1(input: &str) -> Vec<((u32,u32),(u32,u32))> {
    let output = input.lines()
    .map(|line|line.split(',')
        .map(|range|range.split('-')
            .map(|v|v.parse().unwrap()).collect::<Vec<_>>()
        ).collect::<Vec<_>>()
    ).collect::<Vec<_>>();
    // Tranform pairs of pairs from Vec of Vec to Tuple of Tuples
    output.into_iter()
    .map(|pair_of_pairs| pair_of_pairs.into_iter()
        .map(|pair| vecpair2tuple(pair))
        .collect())
    .map(|pair| vecpair2tuple(pair))
    .collect()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day4, part1)]
pub fn part1(input: &[((u32,u32),(u32,u32))]) -> usize {
    input.iter()
    .filter(|elf| {
        elf.0.0 == elf.1.0 ||
        if elf.0.0 < elf.1.0 {
            // does elf.0 contain elf.1?
            elf.0.1 >= elf.1.1
        } else {
            // does elf.1 contain elf.0?
            elf.0.1 <= elf.1.1
        }
    }).count()
}

#[aoc(day4, part2)]
pub fn part2(input: &[((u32,u32),(u32,u32))]) -> usize {
    input.iter()
    .filter(|elf| {
        elf.0.0 == elf.1.0 ||
        if elf.0.0 < elf.1.0 {
            // does elf.0's range end on or after elf.1's start?
            elf.0.1 >= elf.1.0
        } else {
            // does elf.1's range end on or after elf.0's start?
            elf.1.1 >= elf.0.0
        }
    }).count()
}

fn vecpair2tuple<T: Copy>(pair_as_vec: Vec<T>) -> (T,T) {
    (pair_as_vec[0], pair_as_vec[1])
}
// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 2);
    }

    #[test]
    fn test_ex1_part2() {
        assert_eq!(part2(&gen1(EX1)), 4);
    }

const EX1: &'static str =
r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
}