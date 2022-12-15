/// https://adventofcode.com/2022/day/14
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754
///
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax
// extern crate regex;
// use self::regex::{Captures, Regex};

use std::fmt::Display;
use crate::day14::Cave::*;

#[derive(Copy,Clone,Eq,PartialEq,Debug)]
pub enum Cave {
    Start,
    Air,
    Rock,
    Sand,
}

#[derive(Clone)]
pub struct CaveSystem {
    map: Vec<Vec<Cave>>,
    start: (usize,usize), 
    xmin: usize,
}

impl From<Cave> for char {
    fn from(cave: Cave) -> Self {
        match cave {
            Start => '+',
            Air =>   '.',
            Rock =>  '#',
            Sand =>  'o',
        }
    }
}
impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Start => write!(f,"+"),
            Air => write!(f,"."),
            Rock => write!(f,"#"),
            Sand => write!(f,"o"),
        }
    }
}


impl Display for CaveSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.map.len() {
            let mut line = String::new();
            let left = self.xmin.max(1)-1;
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
    let mut ymax = terrain_coords.iter()
        .map(|v|v.iter())
        .flatten()
        .fold(0, |yy,(_,y)| yy.max(*y));
    ymax += 1; // single blank row at bottom
    let (xmin, xmax) = terrain_coords.iter()
        .map(|v|v.iter())
        .flatten()
        .fold((usize::MAX,0), |(mx,xx),(x,y)| (mx.min(*x), xx.max(*x+*y)));
    assert!(xmax >= STARTX);
        // make horizonal room for a diagonal pile
    let blank_row = vec![Air;xmax+1];
    let mut cave_system = Vec::new();
    for _y in 0..=ymax {
        cave_system.push(blank_row.clone());
    }
    let segments = terrain_coords.iter()
        .map(|sequence| sequence.iter()
            .zip(sequence.iter().skip(1))
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for v in segments {
        for ((xx,yy),(xxx,yyy)) in v {
            for y in *yy.min(yyy)..=*yy.max(yyy) {
                for x in *xx.min(xxx)..=*xx.max(xxx) {
                    cave_system[y][x] = Rock;
                }
            }
        }
    }
    cave_system[0][STARTX] = Start;
    CaveSystem { map: cave_system, xmin, start: (STARTX, 0)}
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day14, part1)]
pub fn part1(caves: &CaveSystem) -> usize {
    println!("{}",caves);
    let caves = let_sand_fall(caves.clone());
    println!("{}",caves);
    caves.map.iter()
    .map(|v|v.iter()
        .filter(|item|item==&&Sand))
    .flatten()
    .count()
}

#[aoc(day14, part2)]
pub fn part2(caves: &CaveSystem) -> usize {
    println!("{}",caves);
    let mut caves = caves.clone();
    let row_of_rock = caves.map[0].iter().map(|_|Rock).collect::<Vec<_>>();
    caves.map.push(row_of_rock);
    let caves = let_sand_fall(caves);
    println!("{}",caves);
    caves.map.iter()
    .map(|v|v.iter()
        .filter(|item|item==&&Sand))
    .flatten()
    .count()
}

fn let_sand_fall(mut caves: CaveSystem) -> CaveSystem {
    let ymax = caves.map.len()-1;
    let _xmax = caves.map[0].len()-1;
    let mut grain = caves.start;

    #[cfg(test)]
    let mut timeout = 1_000;
    #[cfg(not (test))]
    let mut timeout = 10_000_000;
    let mut still_filling = true;
    while still_filling {
        let mut last_loc = (usize::MAX, usize::MAX);
        while grain != last_loc { // Not at rest?
            last_loc = grain;
            if grain.1+1 > ymax { // falling into the abyss?
                caves.map[grain.1][grain.0] = Air;
                still_filling = false;
                break; 
            }
            let down = (grain.0, grain.1+1);
            let leftd = (grain.0-1, grain.1+1);
            let rightd = (grain.0+1, grain.1+1);
            caves.map[grain.1][grain.0] = Air;
            grain = match [caves.map[down.1][down.0], caves.map[leftd.1][leftd.0], caves.map[rightd.1][rightd.0]] {
                [Air, _, _] => down,
                [_, Air, _] => leftd,
                [_, _, Air] => rightd,
                _ => grain, // at rest
            };
            caves.map[grain.1][grain.0] = Sand;
            timeout -= 1;
            if timeout <= 0 {break;}
        }
        grain = caves.start;
        if timeout <= 0 {break;}
    }
    caves
}
// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 24);
    }

    #[test]
    fn test_ex1_part2() {
        assert_eq!(part2(&gen1(EX1)), 93);
    }

    const EX1: &'static str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
}
