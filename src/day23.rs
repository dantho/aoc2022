use std::collections::{HashSet, HashMap};

/// https://adventofcode.com/2022/day/23
/// DAN AoC: https://adventofcode.com/2022/leaderboard/private/view/380786
/// HLOTYAK: https://adventofcode.com/2022/leaderboard/private/view/951754

use crate::day23::Dirs::*;

pub enum Dirs {
    North,
    South,
    West,
    East
}

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day23)]
pub fn gen1(input: &str) -> Vec<(usize,usize)> {
    const BORDER: usize = 100;
    let mut elves = Vec::new();
    let lines = input.lines().collect::<Vec<_>>();
    for y in 0..lines.len() {
        let chars = lines[y].chars().collect::<Vec<_>>();
        for x in 0..lines[y].len() {
            if '#' == chars[x] {
                elves.push((BORDER + y, BORDER + x));
            }
        }
    }
    elves
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day23, part1)]
pub fn part1(input: &[(usize,usize)]) -> usize {
    let look_dirs = [North, South, West, East];
    let mut elves = input.iter().cloned().collect::<HashSet<_>>();
    let mut move_count = 0;
    #[cfg(test)]
    print_elves(&elves);
    let mut crowded = elves.iter()
        .filter_map(|&(y,x)| {
            if [(y+1,x-1),(y+1,x),(y+1,x+1),(y-1,x-1),(y-1,x),(y-1,x+1),(y,x-1),(y,x+1)].iter()
            .fold(false, |nearby_detected, pos| nearby_detected || elves.contains(pos)) {
                Some((y,x))
            } else {
                None
            }}).collect::<HashSet<_>>();
    while !crowded.is_empty() {
        let mut proposed: HashMap<(usize,usize), Option<(usize,usize)>> = HashMap::new();
        for &(y,x) in &crowded {
            for look in move_count..move_count+4 {
                let search = match look_dirs[look % 4] {
                    North => [(y-1,x-1),(y-1,x),(y-1,x+1)],
                    South => [(y+1,x-1),(y+1,x),(y+1,x+1)],
                    West =>  [(y-1,x-1),(y,x-1),(y+1,x-1)],
                    East =>  [(y-1,x+1),(y,x+1),(y+1,x+1)],
                };
                let available = !search.iter().fold(false, |occupied, pos| occupied || elves.contains(pos));
                if available {
                    // None indicates conflict
                    proposed.entry(search[1]).and_modify(|v| *v = None).or_insert(Some((y,x)));
                    break;
                }
            }
        }
        proposed.into_iter().for_each(|(to, maybe_from)|
            if let Some(from) = maybe_from {
                // println!("Moving from {:?} to {:?}",from,to);
                elves.remove(&from);
                elves.insert(to);
            }
        );
        crowded = elves.iter().filter_map(|&(y,x)| {
            if [(y+1,x-1),(y+1,x),(y+1,x+1),(y-1,x-1),(y-1,x),(y-1,x+1),(y,x-1),(y,x+1)].iter()
            .fold(false, |nearby_detected, pos| nearby_detected || elves.contains(pos)) {
                Some((y,x))
            } else {
                None
            }}).collect::<HashSet<_>>();

            #[cfg(test)]
            print_elves(&elves);

            move_count += 1;
        // Part 1 only
        // if move_count == 10 {
        //     crowded = HashSet::new(); // terminate loop
        // }
    }
    // Part 1
    // let (ymin,ymax,xmin,xmax) = elven_boundaries(&elves);
    // (ymax-ymin+1)*(xmax-xmin+1)-elves.len()

    // Part 2
    move_count + 1

}

fn elven_boundaries(elves: &HashSet<(usize,usize)>) -> (usize, usize, usize, usize) {
    elves.iter()
    .fold((usize::MAX,0,usize::MAX,0), |(ymin,ymax,xmin,xmax), &(y,x) | (
        y.min(ymin),
        y.max(ymax),
        x.min(xmin),
        x.max(xmax)))
}

fn print_elves(elves: &HashSet<(usize,usize)>) {
    let (ymin,ymax,xmin,xmax) = elven_boundaries(&elves);
    for y in ymin..=ymax {
        for x in xmin..=xmax {
            print!("{}", if elves.contains(&(y,x)) {'#'} else {'.'});
        }
        println!();
    }
}
// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    // Part 2, actually
    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 20);
    }

    // #[test]
    // fn test_ex2_part1() {
    //     assert_eq!(part1(&gen1(EX2)), 25);
    // }

    // #[test]
    // fn test_ex1_part2() {
    //     assert_eq!(part2(&gen1(EX1)), 45000);
    // }

    const EX1: &'static str =
r"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

const EX2: &'static str =
r".....
..##.
..#..
.....
..##.
.....";

}
