/// https://adventofcode.com/2022/day/16
/// DAN AoC: https://adventofcode.com/2022/leaderboard/private/view/380786
/// HLOTYAK: https://adventofcode.com/2022/leaderboard/private/view/951754

use std::collections::HashMap;

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day16)]
pub fn gen1(input: &str) -> (HashMap<String, u32>, HashMap<(String, String), u32>) {
    let mut flow_rates = HashMap::new();
    let mut tunnels = HashMap::new();
    // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    for line in input.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        let name = parts[1].to_string();
        let flow_rate = parts[4][5..parts[4].len()-1].parse().unwrap();
        flow_rates.insert(name.to_string(), flow_rate);
        let tunnelv = parts[9..].to_vec();
        let tunnelv = tunnelv.into_iter().map(|t| {
            let vname = name.clone();
            let tname = t[0..2].to_string();
            (if vname < tname {
                (vname, tname)
            } else {
                (tname, vname)
            }, 1)            
        }).collect::<Vec<_>>();
        tunnels.extend(tunnelv.into_iter());
    }
    (flow_rates, tunnels)
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day16, part1)]
pub fn part1(input: &(HashMap<String, u32>, HashMap<(String, String), u32>)) -> u32 {
    let flow_rates: HashMap<String, u32> = input.0.iter()
        .filter(|(_, flow)| **flow > 0)
        .map(|(s,f)|(s.to_string(),*f))
        .collect();
    let mut tunnels = input.1.clone();

    // println!("Flow Rates: {:?}", input.0);
    println!("Tunnels: {:?}", tunnels);

    // Build up paths through maze
    for _i in 0..100  {
        for ((aa, bb), dist1) in tunnels.clone() {
            let connections = tunnels.iter().filter_map(|((a,b), dist2)| {
                match (a==&aa, b==&bb, a==&bb, b==&aa) {
                    (true, true, _, _) => None,
                    (_, _, true, true) => None,
                    (true, false, _, _) => Some(((bb.to_string(), b.to_string()), dist1+*dist2)),
                    (false, true, _, _) => Some(((aa.to_string(), a.to_string()), dist1+*dist2)),
                    (_, _, true, false) => Some(((aa.to_string(), b.to_string()), dist1+*dist2)),
                    (_, _, false, true) => Some(((bb.to_string(), a.to_string()), dist1+*dist2)),
                    _ => None
                }}).collect::<Vec<_>>();
            for ((a,b), dist) in connections {
                let link = if a < b {(a,b)} else {(b,a)};
                tunnels.entry(link).and_modify(|d| *d = u32::min(*d,dist)).or_insert(dist);
            }
        }
    }

    // Now maximize flow, somehow -- lets target max flow at all costs
    let mut minutes=30;
    let mut location="AA".to_string();

    while minutes > 0 {
        let max_valve_rate = flow_rates.iter().fold(("".to_string(), 0),|(max_valve, max_flow), (valve,&flow)| {
            if flow > max_flow {
                (valve.to_string(), flow)
            } else {
                (max_valve, max_flow)
            }
        });
        let dist_to_max = if max_valve_rate.0 < location {
            tunnels.get(&(max_valve_rate.0, location))
        } else {
            tunnels.get(&(location, max_valve_rate.0))
        };
        let cost = dist_to_max.unwrap()+1;
        // not yet finished:
        if cost <= minutes {
            minutes -= cost;
            location = max_valve_rate.0; // todo
        }
        minutes -= cost.min(minutes);


    }
    
    println!("Tunnels: {:?}", tunnels);

    888
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 1651);
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
