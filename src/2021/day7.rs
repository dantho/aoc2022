/// https://adventofcode.com/2021/day/7
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 
/// 
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax 
// extern crate regex;
// use self::regex::{Captures, Regex};

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day7)]
pub fn gen1(input: &str) -> Vec<usize> {
    input.split(",").map(|numstr|numstr.parse().unwrap()).collect()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day7, part1)]
pub fn part1(input: &[usize]) -> usize {
    let max_pos = *input.iter().max().unwrap();
    let align_energy: Vec<usize> = (0..=max_pos)
        .map(|target_pos|{
            input.iter().map(|&p|(if p>target_pos {p-target_pos} else {target_pos-p})).sum()
        }).collect();
        align_energy.into_iter().min().unwrap()
    }

#[aoc(day7, part2)]
pub fn part2(input: &[usize]) -> usize {
    let max_pos = *input.iter().max().unwrap();
    let align_energy: Vec<usize> = (0..=max_pos)
        .map(|target_pos|{
            input.iter().map(|&p|
                (geom(dist(p,target_pos)))).sum()
        }).collect();
    align_energy.into_iter().min().unwrap()
}
fn dist(p1:usize, p2:usize) -> usize {
    if p1>p2 {p1-p2} else {p2-p1}
}
fn geom(dist:usize)->usize {
    (1..=dist).sum()
}
// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen1() {
        let g = gen1(EX1);
        assert_eq!(g.len(), 10);
    }

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 37);
    }

    #[test]
    fn test_ex1_part2() {
        assert_eq!(part2(&gen1(EX1)), 168);
    }

const EX1: &'static str =
r"16,1,2,0,4,2,7,1,2,14";

const EX2: &'static str =
r"
";

}