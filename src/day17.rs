/// https://adventofcode.com/2022/day/17
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754
///
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax
// extern crate regex;
// use self::regex::{Captures, Regex};

use std::{fmt::Display, collections::HashSet, iter::Map};

const CAVE_WIDTH: usize = 7;

#[derive(Clone)]
pub struct Rock {
    shape: Vec<(usize,usize)>,
    pos: (usize, usize),
}

impl Rock {
    fn top(&self) -> usize {
        self.pos.1 + *self.shape.iter().map(|(_,y)|y).max().unwrap()
    }
    fn bottom(&self) -> usize {
        self.pos.1
    }
    fn right(&self) -> usize {
        self.pos.0 + *self.shape.iter().map(|(x,_)|x).max().unwrap()
    }
    fn left(&self) -> usize {
        self.pos.0
    }
    fn width(&self) -> usize {
        self.right()-self.left() + 1
    }
    fn height(&self) -> usize {
        self.top()-self.bottom() + 1
    }
    fn new(ndx: usize, pos: (usize, usize)) -> Self {
        let ndx = ndx % 5;
        let shape = match ndx {
            0 => vec![(0,0),(1,0),(2,0),(3,0)],
            1 => vec![(1,2),(0,1),(1,1),(2,1),(1,0)],
            2 => vec![(2,2),(2,1),(0,0),(1,0),(2,0)],
            3 => vec![(0,3),(0,2),(0,1),(0,0)],
            4 => vec![(0,1),(1,1),(0,0),(1,0)],
            _ => panic!("Illegal Rock ndx!"),
        };
        let mut rock = Rock {shape, pos};
        rock
    }
    fn pieces(&self) -> Vec<(usize, usize)> {
        let pieces = self.shape.iter().map(|(x,y)| (*x+self.pos.0, *y+self.pos.1)).collect::<Vec<_>>();
        pieces
    }
    fn move_left(&mut self) {
        self.pos.0 = if self.pos.0 == 0 {0} else {self.pos.0 - 1};
    }
    fn move_right(&mut self) {
        self.pos.0 = (self.pos.0 + 1).min(CAVE_WIDTH - self.width());
    }

    fn move_down(&mut self) {
        if self.bottom() == 0 { 
            panic!("Cannot move below the floor!")
        }
        self.pos.1 -= 1;
    }
}

pub struct RockPile {
    pile: HashSet<(usize,usize)>,
}

impl RockPile {
    fn new() -> Self {
        RockPile { pile: HashSet::new() }
    }
    fn top(&self) -> usize {
        self.pile.iter().fold(usize::MIN,|max, (_,y)| *y.max(&max))
    }
    fn collision(&self, pieces: &[(usize, usize)]) -> bool {
        pieces.iter().fold(false,|did_collide, piece| 
            did_collide || self.pile.contains(piece))
    }
    fn rock_at_rest(&self, pieces: &[(usize, usize)]) -> bool {
        let lower_pieces = pieces.iter().map(|(x,y)|(*x,*y-1)).collect::<Vec<_>>();
        self.collision(&lower_pieces[..])
    }
    fn add(&mut self, pieces: Vec<(usize, usize)>) {
        pieces.into_iter().for_each(|piece| assert!(self.pile.insert(piece)));
    }
}

impl Display for RockPile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.top()+3).rev() {
            let mut row = '|'.to_string();
            for x in 0..CAVE_WIDTH {
                row.push(if self.pile.contains(&(x,y)) {
                    '#'
                } else {
                    '.'
                });
            }
            row.push('|');
            writeln!(f,"{}",row)?;
        }
        let mut row = '+'.to_string();
        for _x in 0..CAVE_WIDTH {
            row.push('-');
        }
        row.push('+');
        writeln!(f,"{}",row)?;
        Ok(())
    }
}

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day17)]
pub fn gen1(input: &str) -> String {
    input.lines().nth(0).unwrap().to_string() // Lot of effort to trim off /n
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day17, part1)]
pub fn part1(input: &str) -> usize {
    const ROCK_MAX: usize = 2022;
    const ROCK_LEN: usize = 5;
    let jets = input.chars().collect::<Vec<_>>();
    let mut rock_pile = RockPile::new();
    let mut rock_cnt = 0;
    let mut rock_in_motion = None;
    for step in 0.. {
        let even_step = 0 == step % 2;
        let ndx = step / 2;
        if ndx % jets.len() == 0 && even_step || rock_cnt == 1011 {
            println!("ndx: {}, Rock_cnt: {}, Rock: {}  Pile_top: {}", ndx, rock_cnt, rock_cnt % 5, rock_pile.top());
        }
        // First make new rock appear if necessary
        if rock_in_motion.is_none() {
            let starting_pos = if rock_pile.pile.is_empty() {(2, 3)} else {(2, rock_pile.top()+4)};
            rock_in_motion = Some(Rock::new(rock_cnt % ROCK_LEN, starting_pos));
            rock_cnt += 1;
            if rock_cnt > ROCK_MAX {break}; // TERMINATE INFINITE LOOP
            if rock_cnt == 5 {println!("{}", rock_pile);}

        }
        let mut rock = rock_in_motion.unwrap();
        rock_in_motion = 
            if even_step { // Blow rock left or right based on jet of air
                match jets[ndx % jets.len()] {
                    '>' => {
                        if rock_cnt <= 5 {dbg!(rock.pos); println!("Right");}
                        rock.move_right();
                        if rock_pile.collision(&rock.pieces()) {rock.move_left();} // undo move
                    },
                    '<' => {
                        if rock_cnt <= 5 {dbg!(rock.pos); println!("Left")};
                        rock.move_left();
                        if rock_pile.collision(&rock.pieces()) {rock.move_right();} // undo move
                    },
                    _ => panic!("Unknown jet char!"),
                }
                Some(rock)
            } else { // Move rock and add pieces to pile if at rest (further movement would cause a collision)
                if rock.bottom() == 0 || rock_pile.rock_at_rest(&rock.pieces()) {
                    rock_pile.add(rock.pieces());
                    None
                } else {
                    rock.move_down();
                    Some(rock)
                }
            }
    }
    rock_pile.top() + 1
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 3068);
    }

    // #[test]
    // fn test_ex1_part2() {
    //     assert_eq!(part2(&gen1(EX1)), 999);
    // }

    const EX1: &'static str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
}
