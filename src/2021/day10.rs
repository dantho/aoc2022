/// https://adventofcode.com/2021/day/10
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 
/// 
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax 
// extern crate regex;
// use self::regex::{Captures, Regex};

use std::str::FromStr;
use self::Delim::*;

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub enum Delim {
    BrackOpen,
    BrackClose,
    ParenOpen,
    ParenClose,
    CurlyOpen,
    CurlyClose,
    CompareOpen,
    CompareClose,
}
impl Delim {
    fn from_char(c: char) -> Result<Self, ()> {
        match c {
            '[' => Ok(BrackOpen),
            ']' => Ok(BrackClose),
            '(' => Ok(ParenOpen),
            ')' => Ok(ParenClose),
            '{' => Ok(CurlyOpen),
            '}' => Ok(CurlyClose),
            '<' => Ok(CompareOpen),
            '>' => Ok(CompareClose),
            _ => Err(()),
        }
    }
    fn is_open(&self) -> bool {
        match *self {
            BrackOpen => true,
            ParenOpen => true,
            CurlyOpen => true,
            CompareOpen => true,
            _ => false,
        }
    }
    fn is_close(&self) -> bool {
        !Self::is_open(self)
    }
    fn matching(&self) -> Self {
        match *self {
            BrackOpen => BrackClose,
            ParenOpen => ParenClose,
            CurlyOpen => CurlyClose,
            CompareOpen => CompareClose,
            BrackClose => BrackOpen,
            ParenClose => ParenOpen,
            CurlyClose => CurlyOpen,
            CompareClose => CompareOpen,
        }
    }
}

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day10)]
pub fn gen1(input: &str) -> Vec<Vec<Delim>> {
    input.lines()
    .map(|l|l.chars().map(|c|Delim::from_char(c)).collect::<Result<Vec<_>,_>>()).collect::<Result<Vec<_>,_>>().unwrap()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day10, part1)]
pub fn part1(input: &[Vec<Delim>]) -> usize {
    let validate_lines = input.into_iter().map(|line| validate_line(line))
        .collect::<Vec<Result<Vec<Delim>,Delim>>>();
    validate_lines.into_iter().filter(|res|res.is_err()).map(|err_d| {
        if let Err(d) = err_d {
            match d {
                ParenClose => 3,
                BrackClose => 57,
                CurlyClose => 1197,
                CompareClose => 25137,
                _ => panic!("Unexpected delim found"),
            }
        } else {
            panic!("Broken filter")
        }
    }).sum()
}

#[aoc(day10, part2)]
pub fn part2(input: &[Vec<Delim>]) -> usize {
    let validate_lines = input.into_iter().map(|line| validate_line(line))
        .collect::<Vec<Result<Vec<Delim>,Delim>>>();
    let valid_lines= validate_lines.into_iter().filter(|res|res.is_ok()).collect::<Result<Vec<Vec<Delim>>,_>>().unwrap();
    let mut scores: Vec<usize> = valid_lines.into_iter().map(|v| v.into_iter().rev().fold(0,|score,d| {
        score * 5 +
        match d.matching() {
            ParenClose => 1,
            BrackClose => 2,
            CurlyClose => 3,
            CompareClose => 4,
            _ => panic!("Unexpected delim found"),
        }
    })).collect();
    scores.sort();
    scores[scores.len() / 2] // return middle score, len() should be odd
}

fn validate_line(line: &[Delim]) -> Result<Vec<Delim>,Delim> {
    let result_stack = line.iter().fold(Vec::new(),|mut stack, &d| {
        if d.is_open() {
            stack.push(Ok(d));
        } else {
            match stack.pop() {
                Some(Ok(popped_delim)) => {
                    if popped_delim.matching() != d {
                        stack.push(Err(d));
                    }
                },
                Some(prior_err) => {
                    stack.push(prior_err)
                },
                None => {
                    stack.push(Err(d));
                },
            }
        }
        stack
    });
    result_stack.into_iter().collect::<Result<Vec<Delim>,Delim>>()
}
// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        let g = gen1(EX1);
        assert_eq!(g.len(), 10);
        assert_eq!(g[0].len(), 24);
    }

    #[test]
    fn test_ex1_part1() {
        let g = gen1(EX1);
        let p1 = part1(&g);
        assert_eq!(p1, 26397);
    }

    #[test]
    fn test_ex1_part2() {
        let g = gen1(EX1);
        let p1 = part2(&g);
        assert_eq!(p1, 288957);
    }

const EX1: &'static str =
r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

}