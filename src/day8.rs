/// https://adventofcode.com/2022/day/8
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
#[aoc_generator(day8)]
pub fn gen1(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|row| {
        row.chars().map(|c| {
            c.to_digit(10).unwrap() as i32
        }).collect()
    }).collect()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day8, part1)]
pub fn part1(input: &[Vec<i32>]) -> usize {
    let mut visible: Vec<Vec<bool>> = input.iter()
        .map(|row| row.iter()
            .map(|_| false).collect()
    ).collect();
    let mut max_vis_vert: Vec<i32>;
    let mut max_vis_horz: Vec<i32>;

    // search vert & horz in index order
    max_vis_vert = input.iter().map(|_| -1i32).collect();
    max_vis_horz = input[0].iter().map(|_| -1i32).collect();
    
    for row in 0..input.len() {
        for col in 0..input[0].len() {
            let this_tree = input[row][col];
            visible[row][col] = visible[row][col]
                || this_tree > max_vis_horz[col]
                || this_tree > max_vis_vert[row];
            max_vis_horz[col] = max_vis_horz[col].max(this_tree);
            max_vis_vert[row] = max_vis_vert[row].max(this_tree);
        }
    }

    // search vert & horz in reverse-index order
    max_vis_vert = input.iter().map(|_| -1i32).collect();
    max_vis_horz = input[0].iter().map(|_| -1i32).collect();
    
    for row in (0..input.len()).rev() {
        for col in (0..input[0].len()).rev() {
            let this_tree = input[row][col];
            visible[row][col] = visible[row][col]
                || this_tree > max_vis_horz[col]
                || this_tree > max_vis_vert[row];
            max_vis_horz[col] = max_vis_horz[col].max(this_tree);
            max_vis_vert[row] = max_vis_vert[row].max(this_tree);
        }
    }
    #[cfg(test)] input.iter().for_each(|row|println!("{:?}", row));
    #[cfg(test)] visible.iter().for_each(|row|println!("{:?}", row));

    // How many are visible?
    visible.iter().map(|col|col.iter()).flatten().filter(|b|**b).count()
}

#[aoc(day8, part2)]
pub fn part2(input: &[Vec<i32>]) -> usize {
    let mut scenic_score: Vec<Vec<usize>> = input.iter()
        .map(|row| row.iter()
            .map(|_| 1).collect()
    ).collect();

    for row in 0..input.len() {
        for col in 0..input[0].len() {
            let this_tree = input[row][col];

            let mut this_scenic_score = 0;
            for view_row in (0..row).rev() {
                this_scenic_score += 1;
                if input[view_row][col] >= this_tree {
                    break;
                }
            }
            scenic_score[row][col] *= this_scenic_score;

            let mut this_scenic_score = 0;
            for view_row in row+1..input.len() {
                this_scenic_score += 1;
                if input[view_row][col] >= this_tree {
                    break;
                }
            }
            scenic_score[row][col] *= this_scenic_score;

            let mut this_scenic_score = 0;
            for view_col in (0..col).rev() {
                this_scenic_score += 1;
                if input[row][view_col] >= this_tree {
                    break;
                }
            }
            scenic_score[row][col] *= this_scenic_score;

            let mut this_scenic_score = 0;
            for view_col in col+1..input.len() {
                this_scenic_score += 1;
                if input[row][view_col] >= this_tree {
                    break;
                }
            }
            scenic_score[row][col] *= this_scenic_score;
        }
    }

    #[cfg(test)] println!("Input:");
    #[cfg(test)] input.iter().for_each(|row|println!("{:?}", row));
    #[cfg(test)] println!("Scenic score:");
    #[cfg(test)] scenic_score.iter().for_each(|row|println!("{:?}", row));

    // Best scenic score?
    scenic_score.into_iter().map(|col|col.into_iter()).flatten().max().unwrap()
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 21);
    }

    #[test]
    fn test_ex1_part2() {
        assert_eq!(part2(&gen1(EX1)), 8);
    }

    const EX1: &'static str = r"30373
25512
65332
33549
35390";
}
