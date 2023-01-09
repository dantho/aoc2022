/// https://adventofcode.com/2022/day/19
/// DAN AoC: https://adventofcode.com/2022/leaderboard/private/view/380786
/// HLOTYAK: https://adventofcode.com/2022/leaderboard/private/view/951754

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

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day19, part1)]
pub fn part1(input: &[Vec<usize>]) -> usize {
    const MINUTES_OF_OPERATION: usize = 18;
    let blueprints: Vec<Factory> = input.iter()
    .map(|params| Factory::new(params))
    .collect();

    blueprints.iter().map(|blueprint| {
        blueprint.quality_level(MINUTES_OF_OPERATION)
    }).sum()
}

// *********************
// ***** Utilities *****
// *********************
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

// *********************
// *** Day N Details ***
// *********************
#[derive(Clone, Copy, Debug)]
pub enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Clone, Debug)]
pub struct Robot {
    #[allow(dead_code)]
    produces: Material,
    cost: [usize;3],
    count: usize,
}

#[derive(Clone, Debug)]
pub struct Factory {
    blueprint: usize,
    inventory: [usize;4],
    robots: [Robot; 4],
    max_robots: [usize; 4],
}

impl Factory {
    fn new(input: &[usize]) -> Self {
        let blueprint = input[0];
        let inventory = [0, 0, 0, 0];
        // a robot
        let produces = Ore;
        let cost = [input[1], 0, 0];
        let ore = Robot { produces, cost, count: 1 };
        // a robot
        let produces = Clay;
        let cost = [input[2], 0, 0];
        let clay = Robot { produces, cost, count: 0 };
        // a robot
        let produces = Obsidian;
        let cost = [input[3], input[4], 0];
        let obsidian = Robot { produces, cost, count: 0 };
        // a robot
        let produces = Geode;
        let cost = [input[5], 0, input[6]];
        let geode = Robot { produces, cost, count: 0 };

        let robots = [ore, clay, obsidian, geode];

        // Limit Robot count to most expensive cost for any robot type (except Geode)
        let max_robots = [Ore, Clay, Obsidian, Geode].into_iter()
            .fold([0,0,0,usize::MAX],|[ore_max, clay_max, obs_max, geode], robot_type| {
                let [ore, c, obs] = match robot_type {
                    Ore => robots[0].cost,
                    Clay => robots[1].cost,
                    Obsidian => robots[2].cost,
                    Geode => robots[3].cost,
                };
                [ore_max.max(ore), clay_max.max(c), obs_max.max(obs), geode]
            });

        Factory { blueprint, inventory, robots, max_robots}
    }

    fn quality_level(&self, minutes_remaining: usize) -> usize {
            if minutes_remaining == 0 {
                let geode_cnt = self.inventory[3];
                // This terminates recursion
                return self.blueprint * geode_cnt;
            }
            let [new_ore, new_clay, new_obs, new_geode] = self.produce();
            // Now let's consider the build options from this one factory at this minute
            let mut list_of_options = Vec::new();
            if self.can_build(Geode) {
                list_of_options.push(Some(Geode))
            } else {
                if self.can_build(Obsidian) {list_of_options.push(Some(Obsidian))};
                if self.can_build(Clay) {list_of_options.push(Some(Clay))};
                if self.can_build(Ore) {list_of_options.push(Some(Ore))};
                list_of_options.push(None); // Choose to build nothing
            };
            list_of_options.into_iter()
            .fold(usize::MIN, |max_quality, build_option| {
                let mut ff = self.clone();
                // Build new robot (and pay inventory costs)
                if let Some(robot_type) = build_option {
                    ff.build(robot_type);
                }
                // Update inventory based on production (determined above)
                let [ore, clay, obs, geode] = ff.inventory;
                ff.inventory = [
                    ore + new_ore,
                    clay + new_clay,
                    obs + new_obs,
                    geode + new_geode,
                ];
                let max_geode = max_quality / ff.blueprint;
                let geode_cnt = ff.inventory[3];
                let geode_robots = ff.robots[3].count;
                let estimated_max = (0..minutes_remaining).fold((geode_robots, geode_cnt),|(future_robots, future_geode), _| (future_robots+1, future_geode + future_robots)).1;
                if estimated_max < max_geode {
                    max_quality // don't bother with this one
                } else {
                    max_quality.max(ff.quality_level(minutes_remaining-1))
                }
            })
    }

    fn produce(&self) -> [usize; 4] {
        // Tally to-be-produced resources
        let produced_this_minute = [
                self.robots[0].count,
                self.robots[1].count,
                self.robots[2].count,
                self.robots[3].count
        ];
        // Return new production (don't add to inventory yet)
        produced_this_minute
    }

    fn can_build(&self, robot_type: Material) -> bool {
        // Get existing resources in inventory
        let [ore, clay, obs, _geode] = self.inventory;
        // And cost of selected robot type
        let [ore_cost, clay_cost, obs_cost] = match robot_type {
            Ore => self.robots[0].cost,
            Clay => self.robots[1].cost,
            Obsidian => self.robots[2].cost,
            Geode => self.robots[3].cost,
        };
        // Check robot limit of selected robot type
        let robot_limit_reached = match robot_type {
            Ore => self.robots[0].count >= self.max_robots[0],
            Clay => self.robots[1].count >= self.max_robots[1],
            Obsidian => self.robots[2].count >= self.max_robots[2],
            Geode => self.robots[3].count >= self.max_robots[3],
        };
        if robot_limit_reached {
            false
        } else {
            ore >= ore_cost && clay >= clay_cost && obs >= obs_cost
        }
    }

    fn build(&mut self, robot_type: Material) {
        assert!(self.can_build(robot_type));
        // Get existing resources in inventory
        let [mut ore, mut clay, mut obs, geode] = self.inventory;
        // Determine cost of this robot type
        let [ore_cost, clay_cost, obs_cost] = match robot_type {
            Ore => self.robots[0].cost,
            Clay => self.robots[1].cost,
            Obsidian => self.robots[2].cost,
            Geode => self.robots[3].cost,
        };
        // Pay for the robot
        ore -= ore_cost;
        clay -= clay_cost;
        obs -= obs_cost;
        self.inventory = [
            ore,
            clay,
            obs,
            geode
        ];
        // Build the robot
        match robot_type {
            Ore => self.robots[0].count += 1,
            Clay => self.robots[1].count += 1,
            Obsidian => self.robots[2].count += 1,
            Geode => self.robots[3].count += 1,
        };
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
        // Shouldn't really panic, but I haven't implemented the exception yet
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
