/// https://adventofcode.com/2022/day/5
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786 
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754 
/// 
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax 
// extern crate regex;
// use self::regex::{Captures, Regex};

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day5)]
pub fn gen1(input: &str) -> (Vec<Vec<Option<char>>>, Vec<Vec<u16>>) {
    let mut lines = input.lines();
    let mut crates: Vec<Vec<Option<char>>> = Vec::new();
    let mut row = lines.next().unwrap();
    loop {
        if row.chars().filter(|&c|c != ' ').nth(0) != Some('[') {break}
        // process row of crates
        let crate_row = row.chars().skip(1).step_by(4)
            .map(|c|{
                match c {
                    ' ' => None,
                    a if (a >= 'A' && a <='Z') || a == ' ' => Some(c),
                    bad => panic!("Bad char found while parsing rows: '{}'", bad)
                }
            }).collect::<Vec<Option<char>>>();
        crates.push(crate_row);
        match lines.next() {
            Some(r) => {row = r},
            None => {break}
        }
    }
    // Crates done, now process moves
    lines.next().unwrap(); // Discard one blank line
    let moves = lines.map(|line|{
        line.split(' ')
        .skip(1).step_by(2)
        .map(|n|n.parse::<u16>().unwrap())
        .collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    (crates, moves)
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day5, part1)]
pub fn part1(input: &(Vec<Vec<Option<char>>>, Vec<Vec<u16>>)) -> usize {
    0
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        let (crates, moves) = gen1(EX1);
        assert_eq!(crates.len(), 3);
        assert_eq!(moves.len(), 4);
        assert_eq!(crates[0], [None, Some('D'), None]);
        assert_eq!(crates[2], [Some('Z'), Some('M'), Some('P')]);
        assert_eq!(moves[0][0], 1);
        assert_eq!(moves[3][2], 2);
    }

    // #[test]
    // fn test_ex1_part2() {
    //     assert_eq!(part2(&gen1(EX1)), 4);
    // }

const EX1: &'static str =
r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
}