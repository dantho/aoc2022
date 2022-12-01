/// https://adventofcode.com/2021/day/4
/// ADI: https://adventofcode.com/2021/leaderboard/private/view/380786 
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 
/// 
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax 
extern crate regex;
// use self::regex::{Captures, Regex};

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day4)]
pub fn gen1(input: &str) -> (Vec<Vec<usize>>, Vec<usize>) {
    let mut input = input.split("\n\n");
    let draws = input.next().unwrap().split(",").map(|s|s.parse().unwrap()).collect();
    let boards = input.map(|rawbrd|rawbrd.split_whitespace().map(|s|s.parse().unwrap()).collect()).collect();
    (boards,draws)
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day4, part1)]
pub fn part1(input: &(Vec<Vec<usize>>, Vec<usize>)) -> usize {
    let mut winning_draw_ndx = None;
    let mut winning_board = None;
    let (boards, draws) = input;
    for d in 4..draws.len() {
        for brd in boards {
            for row in 0..5 {
                if (0..5).fold(true,|is_bingo,col|is_bingo && draws[0..=d].contains(&brd[row*5+col])) {
                    winning_draw_ndx = Some(d);
                    winning_board = Some(brd);
                    break;    
                }
            }
            if winning_board != None {break};
            for col in 0..5 {
                if (0..5).fold(true,|is_bingo,row|is_bingo && draws[0..=d].contains(&brd[row*5+col])) {
                    winning_draw_ndx = Some(d);
                    winning_board = Some(brd);
                    break;    
                }
            }
        }
        if winning_board != None {break};
    }
    match winning_board {
        Some(brd) => {
            let d = winning_draw_ndx.unwrap();
            let sum_of_unused: usize = brd.iter().filter(|n|!draws[0..=d].contains(n)).sum();
            draws[d]*sum_of_unused
        },
        None => panic!("No winner found!"),
    }
}

#[aoc(day4, part2)]
pub fn part2(input: &(Vec<Vec<usize>>, Vec<usize>)) -> usize {
    let (boards, draws) = input;
    let mut winning_draw_by_board = vec![None;boards.len()];
    for d in 4..draws.len() {
        for brd_ndx in 0..boards.len() {
            let brd = &boards[brd_ndx];
            // Check rows
            if None == winning_draw_by_board[brd_ndx] {
                for row in 0..5 {
                    if (0..5).fold(true,|is_bingo,col|is_bingo && draws[0..=d].contains(&brd[row*5+col])) {
                        // Bingo!
                        winning_draw_by_board[brd_ndx] = Some(d);
                        break;
                    }
                }
            }
            // Check cols
            if None == winning_draw_by_board[brd_ndx] {
                for col in 0..5 {
                    if (0..5).fold(true,|is_bingo,row|is_bingo && draws[0..=d].contains(&brd[row*5+col])) {
                        // Bingo!
                        winning_draw_by_board[brd_ndx] = Some(d);
                        break;
                    }
                }
            }
        }
    }
    assert!(!winning_draw_by_board.contains(&None));
    let winning_draw_by_board: Vec<usize> = winning_draw_by_board.into_iter().map(|d|d.unwrap()).collect();
    let (brd_ndx, d) = winning_draw_by_board.into_iter().enumerate()
        .fold((0,0),|(ndx_of_max,max_d),(ndx, d)| if d > max_d {
            (ndx, d)
        } else {
            (ndx_of_max, max_d)
        });
    let sum_of_unused: usize = boards[brd_ndx].iter().filter(|n|!draws[0..=d].contains(n)).sum();
    draws[d]*sum_of_unused
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_gen1() {
        let (boards, draws) = gen1(EX1);
        assert_eq!(draws.len(), 27);
        assert_eq!(draws[0], 7);
        assert_eq!(draws[draws.len()-1], 1);
        assert_eq!(boards.len(), 3);
        assert_eq!(boards[0].len(), 25);
        assert_eq!(boards[0][0], 22);
        assert_eq!(boards[0][24], 19);
        assert_eq!(boards[2].len(), 25);
        assert_eq!(boards[2][0], 14);
        assert_eq!(boards[2][24], 7);
    }

    #[test]
    fn test_ex2_part1() {
        let ans = part1(&gen1(EX1));
        assert_eq!(ans, 4512);
    }

    #[test]
    fn test_ex2_part2() {
        let ans = part2(&gen1(EX1));
        assert_eq!(ans, 1924);
    }

const EX1: &'static str =
r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

const EX2: &'static str =
r"
";

}