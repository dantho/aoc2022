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
    .filter(|elves| {
        let elf0 = elves.0;
        let elf1 = elves.1;
        if elf0.0 >= elf1.0 {
            elf0.1 <= elf1.1
        } else {
            elf0.1 >= elf1.1
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
        assert_eq!(part1(&gen1(EX1)), 157);
    }

    #[test]
    fn test_ex1_part2() {
        assert_eq!(part2(&gen1(EX1)), 70);
    }

const EX1: &'static str =
r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
}