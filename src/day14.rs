/// https://adventofcode.com/2022/day/14
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754
///
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax
// extern crate regex;
// use self::regex::{Captures, Regex};

use std::fmt::Display;


#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub enum Cave {
    Start,
    Air,
    Rock,
    Sand,
}

pub struct CaveSystem {
    map: Vec<Vec<Cave>>,
    minx: usize,
}

impl From<Cave> for char {
    fn from(cave: Cave) -> Self {
        match cave {
            Cave::Start => '+',
            Cave::Air =>   '.',
            Cave::Rock =>  '#',
            Cave::Sand =>  'o',
        }
    }
}
impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cave::Start => write!(f,"+"),
            Cave::Air => write!(f,"."),
            Cave::Rock => write!(f,"#"),
            Cave::Sand => write!(f,"o"),
        }
    }
}


impl Display for CaveSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.map.len() {
            let mut line = String::new();
            let left = self.minx.max(1)-1;
            for x in left..self.map[0].len() {
                line.push(char::from(self.map[y][x]));
            }
            writeln!(f,"{}",line)?;
        }
        Ok(())
    }
}

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day14)]
pub fn gen1(input: &str) -> CaveSystem {
    const STARTX: usize = 500;
    let terrain_coords = input.lines()
        .map(|line|line.split(" -> ")
            .map(|coord| {
                coord.split(',').map(|s|s.parse().unwrap())
                .zip(coord.split(',').map(|s|s.parse::<usize>().unwrap()).skip(1))
                .nth(0).unwrap()
            }).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (minx, maxx, maxy) = terrain_coords.iter()
        .map(|v|v.iter())
        .flatten()
        .fold((usize::MAX,0,0), |(mx,xx,yy),(x,y)| (mx.min(*x), xx.max(*x), yy.max(*y)));
    assert!(maxx >= STARTX);
    let blank_row = vec![Cave::Air;maxx+2];
    let mut cave_system = Vec::new();
    for _y in 0..maxy+2 {
        cave_system.push(blank_row.clone());
    }
    dbg!(&terrain_coords);
    let segments = terrain_coords.iter()
        .map(|sequence| sequence.iter()
            .zip(sequence.iter().skip(1))
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for v in segments {
        for ((xx,yy),(xxx,yyy)) in v {
            for y in *yy.min(yyy)..=*yy.max(yyy) {
                for x in *xx.min(xxx)..=*xx.max(xxx) {
                    cave_system[y][x] = Cave::Rock;
                }
            }
        }
    }
    cave_system[0][STARTX] = Cave::Start;
    CaveSystem { map: cave_system, minx}
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day14, part1)]
pub fn part1(input: &CaveSystem) -> u32 {
    println!("{}",input);
    0
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 999);
    }

    const EX1: &'static str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
}
