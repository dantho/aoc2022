/// https://adventofcode.com/2021/day/12
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754


use std::collections::{HashMap, HashSet};
type Path = Vec<String>;

pub struct Cave {
    name: String,
    connections: HashSet<String>,
}

impl Cave {
    pub fn is_big(&self) -> bool {
        self.name.chars().nth(0).unwrap().is_uppercase()
    }
}

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day12)]

pub fn gen1(input: &str) -> HashMap<String, Cave> {
    let mut cave_system = HashMap::new();
    for line in input.lines() {
        let mut pair = line.split("-").to_owned();
        let name0 = pair.next().unwrap();
        let name1 = pair.next().unwrap();
        let cave0 = cave_system.entry(name0.to_string()).or_insert(Cave {
            name: name0.to_string(),
            connections: HashSet::new(),
        });
        cave0.connections.insert(name1.to_string());
        let cave1 = cave_system.entry(name1.to_string()).or_insert(Cave {
            name: name1.to_string(),
            connections: HashSet::new(),
        });
        cave1.connections.insert(name0.to_string());
    }
    cave_system
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day12, part1)]
pub fn part1(cave_system: &HashMap<String, Cave>) -> usize {
    // validate that no BIG cave is adjacent to another BIG cave.
    // This assumption is important to prevent infinite paths,
    // since BIG caves can be revisited in a path
    for (_, cave) in cave_system {
        if cave.is_big() {
            assert!(cave.connections.iter().fold(true, |all_small, adjacent| {
                all_small && !cave_system.get(adjacent).unwrap().is_big()
            }))
        }
    }
    let starting_cave = cave_system.get("start").unwrap();
    let paths = find_paths(cave_system, starting_cave, Vec::new(), None);
    paths.len()
}

#[aoc(day12, part2)]
pub fn part2(cave_system: &HashMap<String, Cave>) -> usize {
    let starting_cave = cave_system.get("start").unwrap();
    let paths = cave_system.iter()
    .filter(|(_, cave)| !cave.is_big() && cave.name != "start" && cave.name != "end")
    .map(|(special, _)|
        find_paths(cave_system, starting_cave, Vec::new(), Some(special))
    ).flatten().collect::<HashSet<_>>();
    if cfg!(test) {
        println!("Paths:");
        for p in &paths {
            println!("{:?}", p);
        }
    }
    paths.len()
}

fn find_paths(
    cave_system: &HashMap<String, Cave>,
    this_cave: &Cave,
    path_to_get_here: Path,
    special_small_cave_name: Option<&str>,
) -> Vec<Path> {
    if this_cave.name == "end" {
        let mut finished_path = path_to_get_here.clone();
        finished_path.push(this_cave.name.clone());
        return vec![finished_path];
    }
    this_cave.connections.iter()
        // Eliminate repeating small caves (critical for avoiding infinite loops)
        // Unless that small cave is special, then visit no more than twice
        .filter(|&conn|{
            let conn_cave = cave_system.get(conn).unwrap();
            if conn_cave.is_big() {
                true
            } else {
                let small_cave_exception = special_small_cave_name == Some(conn) && {
                    let count_of_special = path_to_get_here.iter().filter(|&name|name==conn).count();
                    count_of_special < 2
                };
                !path_to_get_here.contains(conn) || small_cave_exception
            }
        })
        .map(|conn| {
            let mut new_path = path_to_get_here.clone();
            new_path.push(this_cave.name.clone());
            find_paths(
                cave_system,
                cave_system.get(conn).unwrap(),
                new_path,
                special_small_cave_name,
            )})
        .flatten()
        .collect()
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        let g = gen1(EX1);
        assert_eq!(g.len(), 6);
        let g = gen1(EX2);
        assert_eq!(g.len(), 7);
        let g = gen1(EX3);
        assert_eq!(g.len(), 10);
        let most_connections = g
            .iter()
            .map(|(_, cave)| cave.connections.len())
            .max()
            .unwrap();
        assert_eq!(most_connections, 6);
    }

    #[test]
    fn test_part1_ex1() {
        let g = gen1(EX1);
        let p1 = part1(&g);
        assert_eq!(p1, 10);
    }

    #[test]
    fn test_part1_ex2() {
        let g = gen1(EX2);
        let p1 = part1(&g);
        assert_eq!(p1, 19);
    }

    #[test]
    fn test_part1_ex3() {
        let g = gen1(EX3);
        let p1 = part1(&g);
        assert_eq!(p1, 226);
    }

    #[test]
    fn test_part2_ex1() {
        let g = gen1(EX1);
        let p1 = part2(&g);
        assert_eq!(p1, 36);
    }

    #[test]
    fn test_part2_ex2() {
        let g = gen1(EX2);
        let p1 = part2(&g);
        assert_eq!(p1, 103);
    }

    #[test]
    fn test_part2_ex3() {
        let g = gen1(EX3);
        let p1 = part2(&g);
        assert_eq!(p1, 3509);
    }

    const EX1: &'static str = r"start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const EX2: &'static str = r"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const EX3: &'static str = r"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
}
