/// https://adventofcode.com/2022/day/16
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754
///
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax
// extern crate regex;
// use self::regex::{Captures, Regex};

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Valve {
    name: String,
    is_on: bool,
    flow_rate: u32,
}

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day16)]
pub fn gen1(input: &str) -> Vec<(Valve, Vec<String>)> {
    // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    input.lines()
    .map(|line| {
        let parts = line.split(' ').collect::<Vec<_>>();
        let name = parts[1].to_string();
        let flow_rate = parts[4][5..parts[4].len()-1].parse().unwrap();
        let tunnels = parts[9..].to_vec();
        let tunnels = tunnels.into_iter().map(|s|s.to_string()).collect::<Vec<_>>();
        (Valve{name, is_on: false, flow_rate}, tunnels)
    }).collect::<Vec<_>>()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day16, part1)]
pub fn part1(input: &[(Valve, Vec<String>)]) -> u32 {
    let valves = input
        .iter()
        .map(|(valve, paths)| (valve.name.to_string(), (valve.clone(), paths.clone())))
        .collect::<HashMap<String,(Valve,Vec<String>)>>();
    let valve_cnt = valves.len();

    // find shortest path length from each valve to ALL OTHER VALVES
    // This might come in handy
    let path_lengths: HashMap<String, HashMap<String, u32>> = valves.keys()
    .map(|valve_name| {
        let mut distances = valves.keys().map(|name|(name.to_string(), u32::MAX)).collect::<HashMap<String, u32>>();
        distances.entry(valve_name.to_string()).and_modify(|d| *d = 0); // distance to self is 0
        // This is too complicated.  Better work out a plan!
    }).collect();

    888
}

fn dykstra_1(starting_valve_name: &str, distances: &mut HashMap<String, u32>, valves: &HashMap<String, (Valve, Vec<String>)>) {
    // Ugh.
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 24000);
    }

    // #[test]
    // fn test_ex1_part2() {
    //     assert_eq!(part2(&gen1(EX1)), 45000);
    // }

    const EX1: &'static str = 
    r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    Valve BB has flow rate=13; tunnels lead to valves CC, AA
    Valve CC has flow rate=2; tunnels lead to valves DD, BB
    Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    Valve EE has flow rate=3; tunnels lead to valves FF, DD
    Valve FF has flow rate=0; tunnels lead to valves EE, GG
    Valve GG has flow rate=0; tunnels lead to valves FF, HH
    Valve HH has flow rate=22; tunnel leads to valve GG
    Valve II has flow rate=0; tunnels lead to valves AA, JJ
    Valve JJ has flow rate=21; tunnel leads to valve II";
}
