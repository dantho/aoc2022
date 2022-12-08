use std::{fmt::Display, collections::HashMap};

/// https://adventofcode.com/2022/day/7
/// AoC: https://adventofcode.com/2022/leaderboard/private/view/380786
/// SEGCC: https://adventofcode.com/2022/leaderboard/private/view/951754
///
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax
// extern crate regex;
// use self::regex::{Captures, Regex};
use crate::day7::{FileSystemNode::*, Command::*, TerminalOutput::*};

// ********************
// *** Generator(s) ***
// ********************/
// struct FileSystem {
//     root: trees::Tree<FileSystemNode>,
// }

// impl FileSystem {
//     fn new() -> FileSystem {
//         let newfs: FileSystem = FileSystem { root: Tree::<FileSystemNode>::new(Dir("/".to_string())) };
//         newfs
//     }
// }

struct Path {
    path: Vec<String>
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.iter()
            .map(|s|s.chars().chain("_".chars()))
            .flatten()
            .collect::<String>())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FileSystemNode {
    Dir(String),
    File(String, u64)
}

#[derive(Clone, Debug, PartialEq)]
pub enum TerminalOutput {
    Command(Command),
    OutputLine(FileSystemNode)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Ls,
    Cd(String)
}

#[aoc_generator(day7)]
pub fn gen1(input: &str) -> Vec<TerminalOutput> {
    input.lines()
    .map(|line| {
        let mut parts = line.split(' ');
        match parts.next().unwrap() {
            "$" => match parts.next().unwrap() {
                "ls" => TerminalOutput::Command(Command::Ls),
                "cd" =>  TerminalOutput::Command(Command::Cd(parts.next().unwrap().to_string())),
                bad => panic!("Unknown command '{}'", bad)
            }
            "dir" => TerminalOutput::OutputLine(FileSystemNode::Dir(parts.next().unwrap().to_string())),
            size => TerminalOutput::OutputLine(FileSystemNode::File(parts.next().unwrap().to_string(), size.parse().unwrap()))
            }
        }).collect()
}

// *********************
// *** Part1 & Part2 ***
// *********************

#[aoc(day7, part1)]
pub fn part1(terminal: &[TerminalOutput]) -> u64 {
    let mut fs: HashMap<String, Vec<(String, u64)>> = HashMap::new();
    let mut traversed = Path { path: Vec::new() };
    for line in terminal {
        match line {
            Command(Ls) => (),
            Command(Cd(dir)) if &dir[..] == ".." => {traversed.path.pop();},
            Command(Cd(dir)) => {
                if dir == "/" {traversed.path.clear();}
                traversed.path.push(dir.to_string());
                fs.insert(traversed.to_string(), Vec::new());
            },
            // ignore directories until we cd into them
            OutputLine(Dir(_dir)) => (),
            OutputLine(File(name, size)) => {
                fs.entry(traversed.to_string()).and_modify(|v|v.push((name.to_string(),*size)));
            },
        }
    }
    // Sum sizes of files-only in directores, flat, not hierachical
    let dir_sizes: HashMap<String, u64> = fs.iter()
        .map(|(path, dir_list)| {
            let dir_size: u64 = dir_list.iter()
                .map(|(_name, size)| *size)
                .sum();
            (path.to_string(), dir_size)
        }).collect();
    // Sum sizes of directores WITH subdirectories
    dir_sizes.keys()
        .map(|dir| {
            dir_sizes.iter()
            .filter(|(path,_)| path.starts_with(dir))
            .map(|(_,size)|*size)
            .sum()
        })
        .filter(|dir_size: &u64| *dir_size <= 100_000)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(terminal: &[TerminalOutput]) -> u64 {
    let mut fs: HashMap<String, Vec<(String, u64)>> = HashMap::new();
    let mut traversed = Path { path: Vec::new() };
    for line in terminal {
        match line {
            Command(Ls) => (),
            Command(Cd(dir)) if &dir[..] == ".." => {traversed.path.pop();},
            Command(Cd(dir)) => {
                if dir == "/" {traversed.path.clear();}
                traversed.path.push(dir.to_string());
                fs.insert(traversed.to_string(), Vec::new());
            },
            // ignore directories until we cd into them
            OutputLine(Dir(_dir)) => (),
            OutputLine(File(name, size)) => {
                fs.entry(traversed.to_string()).and_modify(|v|v.push((name.to_string(),*size)));
            },
        }
    }
    // Sum sizes of files-only in directores, flat, not hierachical
    let dir_sizes: HashMap<String, u64> = fs.iter()
        .map(|(path, dir_list)| {
            let dir_size: u64 = dir_list.iter()
                .map(|(_name, size)| *size)
                .sum();
            (path.to_string(), dir_size)
        }).collect();
    // Sum sizes of directores WITH subdirectories
    let dir_sizes: HashMap<String, u64> = dir_sizes.keys()
        .map(|dir| {
            let dir_size = dir_sizes.iter()
            .filter(|(path,_)| path.starts_with(dir))
            .map(|(_,size)|*size)
            .sum();
            (dir.to_string(),dir_size)
        }).collect();
    let total_disk_space = 70_000_000;
    let used_disk_space = dir_sizes["/_"];
    let available = total_disk_space - used_disk_space;
    let target_available = 30_000_000;
    let needed_to_free = target_available - available;
    // find smallest dir size that is greater than needed_to_free
    *dir_sizes.values()
        .filter(|size| **size >= needed_to_free)
        .min().unwrap()
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        assert_eq!(part1(&gen1(EX1)), 95_437);
    }

    #[test]
    fn test_part2_ex1() {
        assert_eq!(part2(&gen1(EX1)), 24_933_642);
    }

    const EX1: &'static str = 
r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
}
