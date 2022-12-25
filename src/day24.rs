/// https://adventofcode.com/2022/day/24
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754
///
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax
// extern crate regex;
// use self::regex::{Captures, Regex};

pub struct Valley {
    map: Vec<Vec<char>>,
    north: Vec<(usize, usize)>,
    east:  Vec<(usize, usize)>,
    south: Vec<(usize, usize)>,
    west:  Vec<(usize, usize)>,
}
// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day24)]
pub fn gen1(input: &str) -> Valley {
    let mut north = Vec::new();
    let mut east = Vec::new();
    let mut south = Vec::new();
    let mut west = Vec::new();
    let map: Vec<Vec<char>> = input.lines()
        .map(|line|line.chars().collect()).collect();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            match map[y][x] {
                '^' => north.push((y,x)),
                '>' => east.push((y,x)),
                'v' => south.push((y,x)),
                '<' => west.push((y,x)),
                _ => (),
            }
        }
    }
    let map = map.iter().map(|row|row.iter().map(|c| match c {
        '^' | '>' | 'v' | '<' => '.',
        other => *other,
    }).collect()).collect();

    Valley { map, north, east, south, west }

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
    // Continue until one available position is END position.
    let mut valley: Valley = *input;
    let valley.north = valley.north;
    999
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
