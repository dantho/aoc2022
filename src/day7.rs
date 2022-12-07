/// https://adventofcode.com/2022/day/7
/// AoC: https://adventofcode.com/2022/leaderboard/private/view/380786
/// SEGCC: https://adventofcode.com/2022/leaderboard/private/view/951754
///
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax
// extern crate regex;
// use self::regex::{Captures, Regex};
use trees::{tr, Node, Tree};
use crate::day7::FileSystemNode::*;

// ********************
// *** Generator(s) ***
// ********************/
struct FileSystem {
    root: trees::Tree<FileSystemNode>,
}

impl FileSystem {
    fn new() -> FileSystem {
        let newfs: FileSystem = FileSystem { root: Tree::<FileSystemNode>::new(Dir("/".to_string())) };
        newfs
    }
}

#[derive(Clone, Debug)]
pub enum TerminalOutput {
    Command(Command),
    OutputLine(FileSystemNode)
}

#[derive(Clone, Debug)]
pub enum Command {
    Ls,
    Cd(String)
}

#[derive(Clone, Debug)]
pub enum FileSystemNode {
    Dir(String),
    File(String, u64)
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
pub fn part1(terminal: &[TerminalOutput]) -> usize {
    println!("{:?}",terminal);
    // let mut fs = FileSystem::new();
    // assert!(fs.root.has_no_child());
    // fs.root.push_back(Tree::new(Dir("a".to_string())));
    // fs.root.push_back(Tree::new(File("my_first_file".to_string(), 12345)));
    // println!("{:?}", fs.root);
    // println!("Size of my_first_file in a is {:?}", {
    //     let File(_, size) = fs.root.iter().nth(1).unwrap();
    // });

    0
}

fn find_no_dups(window_size: usize, input: &str) -> Option<usize> {
    let array: Vec<_> = input.chars().collect();
    array
        .windows(window_size)
        .enumerate()
        .fold(None, |found, (ndx, chars)| match found {
            Some(f) => Some(f),
            None => {
                let mut found = Some(ndx + window_size); // Optimistic default
                for c in chars {
                    if chars.iter().filter(|c2| &c == c2).count() > 1 {
                        found = None;
                        break;
                    }
                }
                found
            }
        })
}
// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        assert_eq!(part1(&gen1(EX1)), 95437);
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
