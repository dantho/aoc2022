/// https://adventofcode.com/2022/day/9
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754
///
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax
// extern crate regex;
// use self::regex::{Captures, Regex};
use crate::day9::Dir::*;
use std::cmp::Ordering::*;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Dir {
    fn from(s: &str) -> Self {
        match s {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            bad => panic!("Bad input for Dir {}", bad),
        }
    }
}
// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day9)]
pub fn gen1(input: &str) -> Vec<(Dir, isize)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let dir: Dir = parts.next().unwrap().into();
            let dist = parts.next().unwrap().parse().unwrap();
            (dir, dist)
        })
        .collect::<Vec<(_, _)>>()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day9, part1)]
pub fn part1(input: &[(Dir, isize)]) -> usize {
    #[cfg(test)]
    println!("-- Starting Part 1 --");
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut tail_history: HashSet<(isize, isize)> = HashSet::new();
    tail_history.insert(tail);
    for (dir, dist) in input.into_iter() {
        for _i in 0..*dist {
            head = move_head(head, *dir);
            tail = follow(head, tail);
            tail_history.insert(tail);
            #[cfg(test)]
            println!("{:?} {:?}", head, tail);
        }
    }

    tail_history.len()
}

#[aoc(day9, part2)]
pub fn part2(input: &[(Dir, isize)]) -> usize {
    #[cfg(test)]
    println!("-- Starting Part 2 --");
    let mut head = (0, 0);
    let mut tails = [(0, 0); 9];
    let mut tail_history: HashSet<(isize, isize)> = HashSet::new();
    tail_history.insert(tails[8]);

    for (dir, dist) in input.into_iter() {
        for _i in 0..*dist {
            head = move_head(head, *dir);
            for t in 0..9 {
                tails[t] = follow(if t == 0 { head } else { tails[t - 1] }, tails[t]);
            }
            tail_history.insert(tails[8]);
        }
        #[cfg(test)]
        println!("{:?} {:?}", head, tails[8]);
    }

    tail_history.len()
}

fn move_head(head: (isize, isize), dir: Dir) -> (isize, isize) {
    match dir {
        Up => (head.0, head.1 + 1),
        Down => (head.0, head.1 - 1),
        Right => (head.0 + 1, head.1),
        Left => (head.0 - 1, head.1),
    }
}

fn follow(head: (isize, isize), tail: (isize, isize)) -> (isize, isize) {
    let abs_x = (head.0 - tail.0).abs();
    let abs_y = (head.1 - tail.1).abs();
    let abs = abs_x.max(abs_y);
    let cmp_x = head.0.cmp(&tail.0);
    let cmp_y = head.1.cmp(&tail.1);
    let new_x = if abs > 1 {
        match cmp_x {
            Greater => tail.0 + 1,
            Equal => tail.0,
            Less => tail.0 - 1,
        }
    } else {
        tail.0
    };
    let new_y = if abs > 1 {
        match cmp_y {
            Greater => tail.1 + 1,
            Equal => tail.1,
            Less => tail.1 - 1,
        }
    } else {
        tail.1
    };
    (new_x, new_y)
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 13);
    }

    #[test]
    fn test_ex1_part2() {
        assert_eq!(part2(&gen1(EX1)), 1);
    }

    #[test]
    fn test_ex2_part2() {
        assert_eq!(part2(&gen1(EX2)), 36);
    }

    const EX1: &'static str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const EX2: &'static str = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
}
