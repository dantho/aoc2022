/// https://adventofcode.com/2022/day/24
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754
///
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax
// extern crate regex;
// use self::regex::{Captures, Regex};

use std::{collections::HashSet, iter::once};

#[derive(Clone ,Debug)]
pub struct Valley {
    north: Vec<(usize, usize)>,
    east:  Vec<(usize, usize)>,
    south: Vec<(usize, usize)>,
    west:  Vec<(usize, usize)>,
    ymax: usize,
    xmax: usize,
    start: (usize, usize),
    end: (usize, usize)
}

impl Valley {
    fn new(map_with_blizzards: &[Vec<char>]) -> Self {
        let mut north = Vec::new();
        let mut east = Vec::new();
        let mut south = Vec::new();
        let mut west = Vec::new();
        for y in 0..map_with_blizzards.len() {
            for x in 0..map_with_blizzards[0].len() {
                match map_with_blizzards[y][x] {
                    '^' => north.push((y,x)),
                    '>' => east.push((y,x)),
                    'v' => south.push((y,x)),
                    '<' => west.push((y,x)),
                    _ => (),
                }
            }
        }
        let ymax = map_with_blizzards.len()-2;
        let xmax = map_with_blizzards[0].len()-2;
    
        Valley { north, east, south, west, ymax, xmax, start: (0,1), end: (ymax+1, xmax)}

    }

    fn time_step(&mut self) {
        self.north = self.north.iter()
            .map(|(y,x)| (if (y-1) == 0 {self.ymax} else {y-1}, *x))
            .collect();
        self.west = self.west.iter()
            .map(|(y,x)| (*y, if (x-1) == 0 {self.xmax} else {x-1}))
            .collect();
        self.south = self.south.iter()
            .map(|(y,x)| (if (y+1) <= self.ymax {y+1} else {1}, *x))
            .collect();
        self.east = self.east.iter()
            .map(|(y,x)| (*y, if (x+1) <= self.xmax {x+1} else {1}))
            .collect();
    }

    fn blizzards(&self) -> HashSet<(usize, usize)> {
        self.north.iter()
        .chain(self.south.iter())
        .chain(self.east.iter())
        .chain(self.west.iter())
        .map(|p|*p)
        .collect()
    }

}

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day24)]
pub fn gen1(input: &str) -> Valley {
    let map: Vec<Vec<char>> = input.lines()
        .map(|line|line.chars().collect()).collect();
    Valley::new(&map)
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day24, part1)]
pub fn part1(input: &Valley) -> u32 {
    // after every minute, consider every position,
    // Eliminate positions with blizzards on them
    // Eliminate positions not-adjacent or coincident with a prior position
    // All remaining positions are valid
    // Continue until an available position is END position.
    let mut valley = input.clone();
    let mut explore = HashSet::new();
    let mut minutes = 0;

    while !explore.contains(&valley.end) {
        minutes += 1;
        valley.time_step();
        // move explore list to newly available and adjacent positions
        explore = explore.into_iter().map(|(y,x)| [(y,x),(y-1,x),(y+1,x),(y,x-1),(y,x+1)].to_vec().into_iter())
        .flatten().chain(once((valley.start.0+1,valley.start.1)))
        .filter_map(|p| match p {
            e if e == valley.end => Some(e),
            (toobig,_) if toobig == valley.ymax+1 => None,
            (_,toobig) if toobig == valley.xmax+1 => None,
            (0,_) => None,
            (_,0) => None,
            other => Some(other),
        })
        .filter(|p| !valley.blizzards().contains(p)).collect();
    }
    minutes
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 18);
    }

    // #[test]
    // fn test_ex1_part2() {
    //     assert_eq!(part2(&gen1(EX1)), 45000);
    // }

    const EX1: &'static str =
r"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
}
