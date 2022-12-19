/// https://adventofcode.com/2022/day/19
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754
///
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax
// extern crate regex;
// use self::regex::{Captures, Regex};
use crate::day19::Material::*;

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day19)]
pub fn gen1(input: &str) -> Vec<Vec<usize>> {
    input.lines()
        .map(|line| parse_numbers_usize(line))
        .collect::<Vec<_>>()
}

#[derive(Clone)]
pub enum Material {
    Clay(usize),
    Geode(usize),
    Obsidian(usize),
    Ore(usize),
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day19, part1)]
pub fn part1(input: &[Vec<usize>]) -> usize {
    let factories: Vec<Factory> = input.iter()
    .map(|inp| Factory::new(inp))
    .collect();

    factories.len()
}

// *********************
// *** Detailed stuf ***
// *********************
pub struct Robot {
    produces: Material,
    cost: Vec<Material>,
    count: usize,
}

impl Robot {
    fn add(&mut self, n: usize) {
        self.count += n;
    }
}

pub struct Inventory {
    clay: Material,
    geode: Material,
    obsidian: Material,
    ore: Material,
}

pub struct robot_list {
    clay: Robot,
    geode: Robot,
    obsidian: Robot,
    ore: Robot,
}

pub struct Factory {
    blueprint: usize,
    inventory: [Material;4],
    robots: robot_list,
}

impl Factory {
    fn new(input: &[usize]) -> Self {
        let blueprint = input[0];
        let inventory = [
            Clay(0),
            Geode(0),
            Obsidian(0),
            Ore(0),
        ];
        // a robot
        let produces = Clay(1);
        let cost = vec![Ore(input[2])];
        let clay = Robot { produces, cost, count: 0 };
        // a robot
        let produces = Geode(1);
        let cost = vec![Ore(input[5]), Obsidian(input[6])];
        let geode = Robot { produces, cost, count: 0 };
        // a robot
        let produces = Obsidian(1);
        let cost = vec![Ore(input[3]), Clay(input[4])];
        let obsidian = Robot { produces, cost, count: 0 };
        // a robot
        let produces = Ore(1);
        let cost = vec![Ore(input[1])];
        let ore = Robot { produces, cost, count: 0 };

        let robots = robot_list {clay, geode, obsidian, ore};

        Factory { blueprint, inventory, robots }
    }
}

// #[aoc(day19, part2)]
// pub fn part2(input: &Vec<Vec<u32>>) -> u32 {
//     999
// }

fn parse_numbers(s: &str) -> Vec<isize> {
    let just_nums: String = s.chars()
    .map(|c| if c.is_numeric() || c == '-' {c} else {' '}).collect();
    just_nums.trim().split(' ')
        .filter(|s|!s.is_empty())
        .map(|s|s.parse().unwrap())
        .collect()
}

fn parse_numbers_usize(s: &str) -> Vec<usize> {
    parse_numbers(s).into_iter()
    .map(|i| i as usize)
    .collect()    
}
// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numbers() {
        assert_eq!(parse_numbers("xyz123abc456def"),[123,456]);
        assert_eq!(parse_numbers("xyz123abc-456def"),[123,-456]);
        assert_eq!(parse_numbers("123 456"),[123,456]);
        assert_eq!(parse_numbers("&123:-456#"),[123,-456]);

        assert_eq!(parse_numbers("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian."),
            [1,4,2,3,14,2,7]);

        assert_eq!(parse_numbers("humanoid robot"),[]);
    }

    #[test]
    #[should_panic]
    fn test_parse_numbers_solitarydash() {
        // Not yet implemented
        assert_eq!(parse_numbers("humanoid-robot"),[]);
    }
    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 33);
    }

    // #[test]
    // fn test_ex1_part2() {
    //     assert_eq!(part2(&gen1(EX1)), 45000);
    // }

    const EX1: &'static str = 
r"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian. 
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

}
