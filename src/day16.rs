/// https://adventofcode.com/2022/day/16
/// DAN AoC: https://adventofcode.com/2022/leaderboard/private/view/380786
/// HLOTYAK: https://adventofcode.com/2022/leaderboard/private/view/951754

use std::collections::HashMap;
type Vname = [char; 2]; // Two-char arrays are COPY, Strings are not

fn sorted(a: [char; 2], b: [char; 2]) -> ([char; 2], [char; 2]) {
    if a < b { (a,b) }
    else { (b,a) }
}
// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day16)]
pub fn gen1(input: &str) -> (HashMap<Vname, u32>, HashMap<(Vname, Vname), u32>) {
    let mut flow_rates = HashMap::new();
    let mut tunnels = HashMap::new();
    // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    for line in input.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        let mut n = parts[1].chars();
        let name = [n.next().unwrap(), n.next().unwrap()] as Vname;
        let flow_rate = parts[4][5..parts[4].len()-1].parse().unwrap();
        flow_rates.insert(name, flow_rate);
        let tunnelv = parts[9..].to_vec();
        let tunnelv = tunnelv.into_iter().map(|t| {
            let vname = name;
            let tname = [t.chars().next().unwrap(), t.chars().next().unwrap()] as Vname;
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
pub fn part1(input: &(HashMap<Vname, u32>, HashMap<(Vname, Vname), u32>)) -> u32 {
    let mut flow_rates: HashMap<Vname, u32> = input.0.iter()
        .filter(|(_, flow)| **flow > 0)
        .map(|(s,f)|(*s,*f))
        .collect();
    let mut tunnels = input.1.clone();

    // println!("Flow Rates: {:?}", input.0);
    let mut sorted_conn = tunnels.iter().collect::<Vec<_>>();
    sorted_conn.sort();
    println!("Direct connections: {:?}", sorted_conn);

    // Build up paths through maze
    for _i in 0..100  {
        for ((aa, bb), dist1) in tunnels.clone() {
            let connections = tunnels.iter().filter_map(|((a,b), dist2)| {
                match (a==&aa, b==&bb, a==&bb, b==&aa) {
                    (true, true, _, _) => None,
                    (_, _, true, true) => None,
                    (true, false, _, _) => Some(((bb, *b), dist1+*dist2)),
                    (false, true, _, _) => Some(((aa, *a), dist1+*dist2)),
                    (_, _, true, false) => Some(((aa, *b), dist1+*dist2)),
                    (_, _, false, true) => Some(((bb, *a), dist1+*dist2)),
                    _ => None
                }}).collect::<Vec<_>>();
            for ((a,b), dist) in connections {
                let link = if a < b {(a,b)} else {(b,a)};
                tunnels.entry(link).and_modify(|d| *d = u32::min(*d,dist)).or_insert(dist);
            }
        }
    }
    let mut sorted_conn = tunnels.iter().collect::<Vec<_>>();
    sorted_conn.sort();
    println!("All connections: {:?}", sorted_conn);

    // Now maximize flow, somehow -- lets target max next flow at all costs -- didn't work
    // Let's try max total flow after considering minutes with valve open -- didn't work
    // Let's enumerate all orders starting with the sorted order of max flow rate first
    let mut minutes=30;
    let mut location=['A','A']; // Starting location is "AA"
    let mut total_flow = 0;
    while minutes > 0 && !flow_rates.is_empty() {
        let tunnels2 = tunnels.clone();

        // Greedy for max flow
        // let max_valve_rate = flow_rates.iter().fold((['_','_'], 0),|(max_valve, max_flow), (&vname, &flow)| {
        //     if flow > max_flow {
        //         (vname, flow)
        //     } else {
        //         (max_valve, max_flow)
        //     }
        // });

        // Greedy for max payback
        let (mut max_valve, mut max_flow) = flow_rates.iter()
            .fold((['_','_'], 0),|(max_valve, max_flow), (&vname, &flow)| {
                let distance_to_valve = tunnels[&sorted(location, vname)];
                let new_flow = if minutes < distance_to_valve + 1 {0} else {
                    (minutes - (distance_to_valve + 1)) * flow
                };
                if new_flow > max_flow {
                    (vname, new_flow)
                } else {
                    (max_valve, max_flow)
                }
            });

        // consider nearer valves "along the way"
        let mut dist_to_max = tunnels[&sorted(location, max_valve)];
        let mut max_rate = flow_rates[&max_valve];
        for &near_valve in flow_rates.keys()
            .filter(move |&&other_valve| {
                if other_valve == location || other_valve == max_valve {
                    false
                } else {
                    let dist_to_other = tunnels2[&sorted(location, other_valve)];
                    dist_to_other < dist_to_max
                }
            }) {
                let dist_to_near = tunnels[&sorted(location, near_valve)];
                let dist_from_near_to_max = tunnels[&sorted(near_valve, max_valve)];
                let near_rate = flow_rates[&near_valve];
                let spent_time = dist_to_near+dist_from_near_to_max+1-dist_to_max;
                let opportunity_cost = max_flow*spent_time;
                let near_benefit = near_rate*dist_from_near_to_max;
                if near_benefit > opportunity_cost {
                    dist_to_max = dist_to_near;
                    max_valve = near_valve;
                    max_rate = near_rate;
                    max_flow = max_rate*(minutes-(dist_to_max+1));
                } 
            }
        
        total_flow += max_flow;
        flow_rates.remove(&max_valve);

        minutes -= dist_to_max+1;
        location = max_valve;
        println!("{:?} with total flow of {} at minute {}", location, max_flow, minutes);
    }
    
    total_flow

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
