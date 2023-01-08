use std::thread::panicking;

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
    const MINUTES_OF_OPERATION: usize = 24;
    let mut blueprints: Vec<Factory> = input.iter()
    .map(|params| Factory::new(params))
    .collect();

    blueprints.iter().map(|blueprint| {
        let mut factories: Vec<Factory> = vec![blueprint.clone()];
        for _minute in 0..MINUTES_OF_OPERATION {
            factories = factories.iter().map(|f| {
                let max_ore = f.max_builds_of(Ore(0));
                let max_clay = f.max_builds_of(Clay(0));
                let max_obs = f.max_builds_of(Obsidian(0));
                let max_geode = f.max_builds_of(Geode(0));
                (0..=max_ore).map(move |ore|
                    (0..=max_clay).map(move |clay|
                        (0..=max_obs).map(move |obs|
                            (0..=max_geode).map(move |geode|
                                [ore,clay,obs,geode]
                            )
                        ).flatten()
                    ).flatten()
                ).flatten()
                .filter(|robot_count| f.build_is_possible(*robot_count))
                .map(|robot_count| {
                    let mut ff = f.clone();
                    let [new_ore, new_clay, new_obs, new_geode] = ff.produce();
                    ff.build(robot_count);
                    let [ore, clay, obs, geode] = if let [Ore(ore), Clay(c), Obsidian(obs), Geode(g)] = ff.inventory {[ore,c,obs,g]} else {panic!("Error")};
                    ff.inventory = [
                        Ore(ore + new_ore),
                        Clay(clay + new_clay),
                        Obsidian(obs + new_obs),
                        Geode(geode + new_geode),
                    ];
                    ff
                })
            }).flatten()
            .collect();
        }
        blueprint.blueprint * factories.iter().map(|f| {
            let geode_cnt = if let Geode(g) = f.inventory[3] {g} else {panic!("Should be Geode()")};
            geode_cnt
        }).max().unwrap()
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
    Ore(usize),
    Clay(usize),
    Obsidian(usize),
    Geode(usize),
}

#[derive(Clone, Debug)]
pub struct Robot {
    #[allow(dead_code)]
    produces: Material,
    cost: [Material;3],
    count: usize,
}

#[derive(Clone, Debug)]
pub struct RobotList {
    geode: Robot,
    obsidian: Robot,
    clay: Robot,
    ore: Robot,
}

#[derive(Clone, Debug)]
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
        let cost = [Ore(input[1]), Clay(0), Obsidian(0)];
        let ore = Robot { produces, cost, count: 1 };
        // a robot
        let produces = Clay(1);
        let cost = [Ore(input[2]), Clay(0), Obsidian(0)];
        let clay = Robot { produces, cost, count: 0 };
        // a robot
        let produces = Obsidian(1);
        let cost = [Ore(input[3]), Clay(input[4]), Obsidian(0)];
        let obsidian = Robot { produces, cost, count: 0 };
        // a robot
        let produces = Geode(1);
        let cost = [Ore(input[5]), Clay(0), Obsidian(input[6])];
        let geode = Robot { produces, cost, count: 0 };

        let robots = RobotList {ore, clay, obsidian, geode};

        Factory { blueprint, inventory, robots}
    }

    // fn reset(self) -> Self {
    //     let blueprint = self.blueprint;
    //     let inventory = [
    //         Ore(0),
    //         Clay(0),
    //         Obsidian(0),
    //         Geode(0),
    //     ];
    //     let mut ore = self.robots.ore;
    //     ore.count = 1;
    //     let mut clay = self.robots.clay.clone();
    //     clay.count = 0;
    //     let mut obsidian = self.robots.obsidian.clone();
    //     obsidian.count = 0;
    //     let mut geode = self.robots.geode.clone();
    //     geode.count = 0;
    //     let robots = RobotList {ore, clay, obsidian, geode};

    //     Factory { blueprint, inventory, robots}
    // }

    fn produce(&self) -> [usize; 4] {
        // Tally to-be-produced resources
        let produced_this_minute = [
            if self.robots.ore.count > 0 {
                #[cfg(test)]
                println!("{} ore-collecting robot{} collect{} {} ore",
                    self.robots.ore.count,
                    if self.robots.ore.count == 1 {""} else {"s"},
                    if self.robots.ore.count == 1 {"s"} else {""},
                    self.robots.ore.count,
                );
                self.robots.ore.count
            } else {0},
            if self.robots.clay.count > 0 {
                #[cfg(test)]
                println!("{} clay-collecting robot{} collect{} {} clay.",
                    self.robots.clay.count,
                    if self.robots.clay.count == 1 {""} else {"s"},
                    if self.robots.clay.count == 1 {"s"} else {""},
                    self.robots.clay.count,
                );
                self.robots.clay.count
            } else {0},
            if self.robots.obsidian.count > 0 {
                #[cfg(test)]
                println!("{} obsidian-collecting robot{} collect{} {} obsidian.",
                    self.robots.obsidian.count,
                    if self.robots.obsidian.count == 1 {""} else {"s"},
                    if self.robots.obsidian.count == 1 {"s"} else {""},
                    self.robots.obsidian.count,
                );
                self.robots.obsidian.count
            } else {0},
            if self.robots.geode.count > 0 {
                #[cfg(test)]
                println!("{} geode-collecting robot{} collect{} {} geode.",
                    self.robots.geode.count,
                    if self.robots.geode.count == 1 {""} else {"s"},
                    if self.robots.geode.count == 1 {"s"} else {""},
                    self.robots.geode.count,
                );
                self.robots.geode.count
            } else {0},
        ];
        // Return new production (don't add to inventory yet)
        produced_this_minute
    }

    fn max_builds_of(&self, robot_type: Material) -> usize {
        // Get existing resources in inventory
        let [ore, clay, obs, geode] = if let [Ore(ore), Clay(c), Obsidian(obs), Geode(g)] = self.inventory {[ore,c,obs,g]} else {panic!("Error")};
        // And cost of selected robot type
        let [ore_cost, clay_cost, obs_cost] = if let [Ore(ore), Clay(c), Obsidian(obs)] = match robot_type {
            Ore(_) => self.robots.ore.cost,
            Clay(_) => self.robots.clay.cost,
            Obsidian(_) => self.robots.obsidian.cost,
            Geode(_) => self.robots.geode.cost,
        } {[ore,c,obs]} else {panic!("Error")};
        let robot_cnt = usize::MAX;
        let robot_cnt = if ore_cost > 0 {robot_cnt.min(ore / ore_cost)} else {robot_cnt};
        let robot_cnt = if clay_cost > 0 {robot_cnt.min(clay / clay_cost)} else {robot_cnt};
        let robot_cnt = if obs_cost > 0 {robot_cnt.min(obs / obs_cost)} else {robot_cnt};
        robot_cnt
    }

    fn build_is_possible(&self, robot_count: [usize; 4]) -> bool {
        // Get existing resources in inventory
        let [mut ore, mut clay, mut obs, _geode] = if let [Ore(ore), Clay(c), Obsidian(obs), Geode(g)] = self.inventory {[ore,c,obs,g]} else {panic!("Error")};
        // Now hypothetically "spend" that inventory building robots
        for (robot_type, count) in [Ore(0), Clay(0), Obsidian(0), Geode(0)].into_iter().zip(robot_count) {
            // Determine cost of this robot type
            let [ore_cost, clay_cost, obs_cost] = if let [Ore(ore), Clay(c), Obsidian(obs)] = match robot_type {
                Ore(_) => self.robots.ore.cost,
                Clay(_) => self.robots.clay.cost,
                Obsidian(_) => self.robots.obsidian.cost,
                Geode(_) => self.robots.geode.cost,
            } {[ore,c,obs]} else {panic!("Error")};
            if ore < ore_cost * count {return false};
            if clay < clay_cost * count {return false};
            if obs < obs_cost * count {return false};
            ore -= ore_cost * count;
            clay -= clay_cost * count;
            obs -= obs_cost * count;
        }
        true
    }

    fn build(&mut self, robot_count: [usize; 4]) {
        assert!(self.build_is_possible(robot_count));
        // Get existing resources in inventory
        let [mut ore, mut clay, mut obs, geode] = if let [Ore(ore), Clay(c), Obsidian(obs), Geode(g)] = self.inventory {[ore,c,obs,g]} else {panic!("Error")};
        // Spend inventory resources building robots
        for (robot_type, count) in [Ore(0), Clay(0), Obsidian(0), Geode(0)].into_iter().zip(robot_count) {
            // Determine cost of this robot type
            let [ore_cost, clay_cost, obs_cost] = if let [Ore(ore), Clay(c), Obsidian(obs)] = match robot_type {
                Ore(_) => self.robots.ore.cost,
                Clay(_) => self.robots.clay.cost,
                Obsidian(_) => self.robots.obsidian.cost,
                Geode(_) => self.robots.geode.cost,
            } {[ore,c,obs]} else {panic!("Error")};
            // Pay for the robot(s)
            ore -= ore_cost * count;
            clay -= clay_cost * count;
            obs -= obs_cost * count;
            // Build the robot(s)
            match robot_type {
                Ore(_) => self.robots.ore.count += count,
                Clay(_) => self.robots.clay.count += count,
                Obsidian(_) => self.robots.obsidian.count += count,
                Geode(_) => self.robots.geode.count += count,
            };
        }
        // Pay the total cost
        self.inventory = [
            Ore(ore),
            Clay(clay),
            Obsidian(obs),
            Geode(geode)
        ];
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
