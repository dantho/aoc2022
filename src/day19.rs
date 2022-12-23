/// https://adventofcode.com/2022/day/19
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754
///
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax
// extern crate regex;
// use self::regex::{Captures, Regex};
use crate::day19::Material::*;

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

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day19)]
pub fn gen1(input: &str) -> Vec<Vec<usize>> {
    input.lines()
        .map(|line| parse_numbers_usize(line))
        .collect::<Vec<_>>()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day19, part1)]
pub fn part1(input: &[Vec<usize>]) -> usize {
    let factories: Vec<Factory> = input.iter()
    .map(|inp| Factory::new(inp))
    .collect();

    factories.into_iter()
        .map(|mut f| {
            f.run(24);
            println!("{:?}",f);
            if let Geode(geode_cnt) = f.inventory[3] {
                geode_cnt * f.blueprint
            } else {
                panic!("Should be Geode(?)")
            }
        }).sum()
}

// *********************
// *** Detailed stuf ***
// *********************
#[derive(Clone, Copy, Debug)]
pub enum Material {
    Clay(usize),
    Geode(usize),
    Obsidian(usize),
    Ore(usize),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Inventory {
    ore: Material,
    clay: Material,
    obsidian: Material,
    geode: Material,
}

#[derive(Debug)]
pub struct RobotList {
    geode: Robot,
    obsidian: Robot,
    clay: Robot,
    ore: Robot,
}

#[derive(Debug)]
pub struct Factory {
    blueprint: usize,
    inventory: [Material;4],
    robots: RobotList,
}

impl Factory {
    fn new(input: &[usize]) -> Self {
        let blueprint = input[0];
        let inventory = [
            Ore(0),
            Clay(0),
            Obsidian(0),
            Geode(0),
        ];
        // a robot
        let produces = Ore(1);
        let cost = vec![Ore(input[1])];
        let ore = Robot { produces, cost, count: 1 };
        // a robot
        let produces = Clay(1);
        let cost = vec![Ore(input[2])];
        let clay = Robot { produces, cost, count: 0 };
        // a robot
        let produces = Obsidian(1);
        let cost = vec![Ore(input[3]), Clay(input[4])];
        let obsidian = Robot { produces, cost, count: 0 };
        // a robot
        let produces = Geode(1);
        let cost = vec![Ore(input[5]), Obsidian(input[6])];
        let geode = Robot { produces, cost, count: 0 };

        let robots = RobotList {ore, clay, obsidian, geode};

        Factory { blueprint, inventory, robots }
    }

    fn run(&mut self, minutes: usize) {
        for _m in 0..minutes {
            // Anticipate to-be-produced resource
            let produced_this_minute = [
                self.robots.ore.count,
                self.robots.clay.count,
                self.robots.obsidian.count,
                self.robots.geode.count
            ];
            // Restructure data format of previously produced resources
            let available_inventory = if let [
                Ore(a),
                Clay(b),
                Obsidian(c),
                Geode(d),
            ] = self.inventory {
                [a,b,c,d]
            } else {
                panic!("Inventory structure mismatch")
            };
            // Spend resources in inventory to build robots, starting with most expensive first
            let [mut ore, mut clay, mut obs, mut geode] = available_inventory;
            // Geode Robot
            if let (Ore(ore_cost),Obsidian(obsidian_cost)) = 
                    (self.robots.geode.cost[0], self.robots.geode.cost[1]) {
                let robot_cnt = ore / ore_cost;
                let robot_cnt = robot_cnt.min(obs / obsidian_cost);
                if robot_cnt > 0 {
                    ore -= robot_cnt * ore_cost;
                    obs -= robot_cnt * obsidian_cost;
                    geode += robot_cnt;
                }
            }
            // Obsidian Robot
            if let (Ore(ore_cost), Clay(clay_cost)) = 
                    (self.robots.obsidian.cost[0], self.robots.obsidian.cost[1]) {
                let robot_cnt = ore / ore_cost;
                let robot_cnt = robot_cnt.min(clay / clay_cost);
                if robot_cnt > 0 {
                    ore -= robot_cnt * ore_cost;
                    clay -= robot_cnt * clay_cost;
                    obs += robot_cnt;
                }
            }
            // Clay Robot
            if let Ore(ore_cost) = self.robots.clay.cost[0] {
                let robot_cnt = ore / ore_cost;
                if robot_cnt > 0 {
                    ore -= robot_cnt * ore_cost;
                    clay += robot_cnt;
                }
            }
            // Ore Robot
            if let Ore(ore_cost) = self.robots.ore.cost[0] {
                let robot_cnt = ore / ore_cost;
                if robot_cnt > 0 {
                    ore -= robot_cnt * ore_cost;
                    ore += robot_cnt;
                }
            }
            // Add new production to remaining inventory
            ore += produced_this_minute[0];
            clay += produced_this_minute[1];
            obs += produced_this_minute[2];
            geode += produced_this_minute[3];

            self.inventory = [
                Ore(ore),
                Clay(clay),
                Obsidian(obs),
                Geode(geode),
            ];
            println!("Inventory: {:?}", self.inventory);
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
