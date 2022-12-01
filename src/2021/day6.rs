/// https://adventofcode.com/2021/day/6
/// ADI: https://adventofcode.com/2021/leaderboard/private/view/380786 
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 
/// 
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax 
// extern crate regex;
// use self::regex::{Captures, Regex};

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day6, part1)]
pub fn gen1(input: &str) -> (Vec<u8>, u16) {
    let days = 80;
    (input.split(",").map(|x|x.parse().unwrap()).collect(), days)
}

#[aoc_generator(day6, part2)]
pub fn gen2(input: &str) -> (Vec<usize>, u16) {
    let days = 256;
    let lanternfish:Vec<usize> = input.split(",").map(|x|x.parse().unwrap()).collect();
    let mut tally_by_age = vec![0usize; 8+1];
    for age in lanternfish {
        tally_by_age[age] += 1;
    }
    (tally_by_age, days)
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day6, part1)]
pub fn part1(input: &(Vec<u8>, u16)) -> usize {
    let mut state = input.0.clone();
    let days = input.1;
    let newborn_age = 8;
    let reset_age = 6;
    for day in 1..=days {
        state = state.iter()
            .map(|&x|
                if 0 == x  {
                    vec![reset_age+1,newborn_age+1]
                } else {
                    vec![x]
                }).flatten()
            .map(|x|x-1)
            .collect();
    }

    if !cfg!(test) {assert_eq!(state.len(),377263)};
    state.len()
}

#[aoc(day6, part2)]
pub fn part2(input: &(Vec<usize>, u16)) -> usize {
    let mut tally_by_age = input.0.clone();
    let days = input.1;
    for day in 0..days {
        let age0 = tally_by_age[0];
        for age in 1..=8 {
            tally_by_age[age-1] = tally_by_age[age];
        }
        tally_by_age[6] += age0;
        tally_by_age[8] = age0;
    }
    tally_by_age.into_iter().sum()
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_gen1() {
        let initial_state = "3,4,3,1,2";
        let generated = gen1(initial_state);
        assert_eq!(generated, (vec![3,4,3,1,2], 80));
    }

    #[test]
    fn test_ex1_part1() {
        let initial_state = "3,4,3,1,2";
        let generated = gen1(initial_state);
        assert_eq!(part1(&(generated.0,18)), 26);
        let generated = gen1(initial_state);
        assert_eq!(part1(&(generated.0,80)), 5934);
    }
    #[test]
    fn test_ex1_part2() {
        let initial_state = "3,4,3,1,2";
        let generated = gen2(initial_state);
        assert_eq!(part2(&(generated.0,256)), 26984457539);
    }

}