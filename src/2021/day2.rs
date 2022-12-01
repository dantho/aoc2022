/// https://adventofcode.com/2021/day/N
/// ADI: https://adventofcode.com/2021/leaderboard/private/view/380786 
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 
/// 
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax 
extern crate regex;
// use self::regex::{Captures, Regex};
use std::convert::From;
use self::Movement::*;

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day2)]
pub fn gen1(input: &str) -> Vec<Movement> {
    input.lines()
        .map(|line|Movement::from(line))
        .collect()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day2, part1)]
pub fn part1(input: &Vec<Movement>) -> i32 {
    let finalpos = input.into_iter()
        .fold((0,0),|(xpos,depth), mv| match mv {
            Forward(n) => (xpos+*n, depth),
            Up(n) => (xpos, if depth >= *n {depth-*n} else {0}),
            Down(n) => (xpos, depth+*n)
        });
    finalpos.0*finalpos.1
}


#[aoc(day2, part2)]
pub fn part2(input: &Vec<Movement>) -> i32 {
    let finalpos = input.into_iter()
    .fold((0,0,0i32),|(xpos,depth,aim), mv| match mv {
        Forward(n) => (xpos+*n, depth+aim**n, aim),
        Up(n) => (xpos, depth, aim-*n),
        Down(n) => (xpos, depth, aim+*n)
    });
    finalpos.0*finalpos.1
}

pub enum Movement {
    Forward(i32),
    Up(i32),
    Down(i32)
}

impl From<&str> for Movement {
    fn from(s: &str) -> Self {
        let mut half = s.split(" ");
        let cmd = half.next().unwrap();
        let dist = half.next().unwrap().parse().unwrap();
        match cmd {
            "forward" => Forward(dist),
            "up" => Up(dist),
            "down" => Down(dist),
            _ => panic!("Parse error on Movement")
        }
    }
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