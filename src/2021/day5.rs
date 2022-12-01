/// https://adventofcode.com/2021/day/5
/// ADI: https://adventofcode.com/2021/leaderboard/private/view/380786 
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 
/// 
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax 
// extern crate regex;
// use self::regex::{Captures, Regex};
use std::{convert::From, collections::HashMap, hash::Hash, cmp::Ord, cmp::Ordering::*};

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day5)]
pub fn gen1(input: &str) -> Vec<(Point,Point)> {
    let points: Vec<Point> = input.lines()
        .map(|line|line.split(" -> "))
        .flatten()
        .map(|pt_str|Point::from(pt_str))
        .collect();    
    points.clone().into_iter().step_by(2).zip(points.into_iter().skip(1).step_by(2)).collect()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day5, part1)]
pub fn part1(input: &[(Point,Point)]) -> usize {
    let mut map: UnderwaterMap = HashMap::new();
    input.iter()
        .filter(|(p1,p2)|p1.x == p2.x || p1.y == p2.y)
        .for_each(|(p1,p2)|{
            if p1.x == p2.x {
                let x = p1.x;
                let y1 = p1.y.min(p2.y);
                let y2 = p1.y.max(p2.y);
                for y in y1..=y2 {
                    let val = map.entry((x,y)).or_insert(0);
                    *val = *val+1;
                }
            } else if p1.y == p2.y {
                let y = p1.y;
                let x1 = p1.x.min(p2.x);
                let x2 = p1.x.max(p2.x);
                for x in x1..=x2 {
                    let val = map.entry((x,y)).or_insert(0);
                    *val = *val+1;
                }
            }
        });
    if cfg!(test) {dump_map(&map)};
    let count_of_overlap = map.values().filter(|&&v|v>1).count();
    count_of_overlap
}

#[aoc(day5, part2)]
pub fn part2(input: &[(Point,Point)]) -> usize {
    let mut map: UnderwaterMap = HashMap::new();
    input.iter()
        .for_each(|(p1,p2)|{
            let dir_x = Ord::cmp(&p2.x,&p1.x);
            let dir_y = Ord::cmp(&p2.y,&p1.y);
            let mut x = p1.x;
            let mut y = p1.y;
            loop {
                let val = map.entry((x,y)).or_insert(0);
                *val = *val+1;
                // Test for done-ness
                if x==p2.x && y==p2.y {break}
                // Move to next point in grid
                x = match dir_x {
                    Greater => x + 1,
                    Equal => x,
                    Less => x - 1,
                };
                y = match dir_y {
                    Greater => y + 1,
                    Equal => y,
                    Less => y - 1,
                };
            }
        });
    if cfg!(test) {dump_map(&map)};
    let count_of_overlap = map.values().filter(|&&v|v>1).count();
    count_of_overlap
}

#[derive(Debug,PartialEq,Eq,Copy,Clone,Hash)]
pub struct Point{
    x: u32,
    y: u32
}
impl From<&str> for Point {
    fn from(s:&str) -> Self {
        let mut nums = s.split(",");
        let x = nums.next().unwrap();
        let x:u32 = x.parse().unwrap();
        let y = nums.next().unwrap();
        let y:u32 = y.parse().unwrap();
        Point{x,y}
    }
}
type UnderwaterMap = HashMap<(u32,u32),u32>;

fn dump_map(map: &UnderwaterMap) {
    let (max_x,max_y) = map.keys().fold((0,0),|(mx,my),&(x,y)|(mx.max(x),my.max(y)));
    for y in 0..=max_y {
        let mut line = String::new();
        for x in 0..=max_x {
            match map.get(&(x,y)) {
                None => line.push_str("."),
                Some(&n) => line.push_str(&n.to_string()),
            };
        }
        println!("{}", line);
    }
}
// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_gen1() {
        let g = gen1(EX1);
        assert_eq!(g.len(), 10);
    }

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 5);
    }

    #[test]
    fn test_ex1_part2() {
        assert_eq!(part2(&gen1(EX1)), 12);
    }


const EX1: &'static str =
r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

}