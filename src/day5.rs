/// https://adventofcode.com/2022/day/5
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754
///
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax
// extern crate regex;
// use self::regex::{Captures, Regex};
use std::cmp::Reverse;

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day5)]
pub fn gen1(input: &str) -> (Vec<Vec<char>>, Vec<Vec<usize>>) {
    let mut lines = input.lines();
    let mut crates: Vec<Vec<Option<char>>> = Vec::new();
    let mut row = lines.next().unwrap();
    loop {
        if row.chars().filter(|&c| c != ' ').nth(0) != Some('[') {
            break;
        }
        // process row of crates
        let crate_row = row
            .chars()
            .skip(1)
            .step_by(4)
            .map(|c| match c {
                ' ' => None,
                a if (a >= 'A' && a <= 'Z') || a == ' ' => Some(c),
                bad => panic!("Bad char found while parsing rows: '{}'", bad),
            })
            .collect::<Vec<Option<char>>>();
        crates.push(crate_row);
        match lines.next() {
            Some(r) => row = r,
            None => break,
        }
    }
    // Crates done, now parse moves
    lines.next().unwrap(); // Discard one blank line
    let moves = lines
        .map(|line| {
            line.split(' ')
                .skip(1)
                .step_by(2)
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Transpose crates from fully populated rows of crates (with blanks)
    // to sparsely populated columns of crates in reversed (LIFO) order.
    let mut transposed = Vec::new();
    for _col in 0..crates[0].len() {
        transposed.push(Vec::new());
    }
    let crates: Vec<Vec<char>> = crates.iter().rev().fold(transposed, |mut trsposd, row| {
        for (c_ndx, maybe_v) in row.iter().enumerate() {
            if let Some(v) = maybe_v {
                trsposd[c_ndx].push(*v);
            }
        }
        trsposd
    });

    (crates, moves)
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day5, part1)]
pub fn part1(input: &(Vec<Vec<char>>, Vec<Vec<usize>>)) -> String {
    let (crates, moves) = input;
    // Move crates [count/from/to] (note move indices are 1-based!)
    let mut crates = crates.clone();
    for move_cmd in moves {
        #[cfg(test)]
        println!("crates: {:?}", crates);
        #[cfg(test)]
        println!(
            "move {} from {} to {}",
            move_cmd[0], move_cmd[1], move_cmd[2]
        );
        for _cnt in 0..move_cmd[0] {
            let tmp = crates[move_cmd[1] - 1].pop().unwrap();
            crates[move_cmd[2] - 1].push(tmp);
        }
    }
    #[cfg(test)]
    println!("crates: {:?}", crates);
    let tops: String = crates
        .into_iter()
        .map(|mut col| col.pop().unwrap())
        .collect();
    tops
}

#[aoc(day5, part2)]
pub fn part2(input: &(Vec<Vec<char>>, Vec<Vec<usize>>)) -> String {
    let (crates, moves) = input;
    // Move crates [count/from/to] (note move indices are 1-based!)
    let mut crates = crates.clone();
    for move_cmd in moves {
        #[cfg(test)]
        println!("crates: {:?}", crates);
        #[cfg(test)]
        println!(
            "move {} from {} to {}",
            move_cmd[0], move_cmd[1], move_cmd[2]
        );
        let mut tmp = Vec::new();
        for _cnt in 0..move_cmd[0] {
            tmp.push(crates[move_cmd[1] - 1].pop().unwrap());
        }
        tmp.reverse();
        crates[move_cmd[2] - 1].append(&mut tmp);
    }
    #[cfg(test)]
    println!("crates: {:?}", crates);
    let tops: String = crates
        .into_iter()
        .map(|mut col| col.pop().unwrap())
        .collect();
    tops
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_gen1() {
        let (crates, moves) = gen1(EX1);
        assert_eq!(crates.len(), 3);
        assert_eq!(moves.len(), 4);
        assert_eq!(crates[0], ['Z', 'N']);
        assert_eq!(crates[1], ['M', 'C', 'D']);
        assert_eq!(crates[2], ['P']);
        assert_eq!(moves[0][0], 1);
        assert_eq!(moves[3][2], 2);
    }

    #[test]
    fn test_ex1_part1() {
        let ans = part1(&gen1(EX1));
        assert_eq!(ans, "CMZ");
    }

    #[test]
    fn test_ex1_part2() {
        assert_eq!(part2(&gen1(EX1)), "MCD");
    }

    const EX1: &'static str = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
}
