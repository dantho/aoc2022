/// https://adventofcode.com/2021/day/23
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 
use self::Amphipod::*;
use self::GameSpace::*;
use std::collections::HashMap;
use std::fmt;
use std::io::stdout;
use std::time::Duration;
use std::thread::sleep;
use crossterm::style::Stylize;
use crossterm::{
    execute,
    cursor::{MoveTo, Hide},
    terminal::{Clear,ClearType},
};

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day23, part1)]
pub fn gen1(input: &str) -> Vec<Amphipod> {
    input.lines().skip(2).take(2)
    .map(|line|line[3..10].split("#")
        .map(|amphi| match amphi {
            "A" => A,
            "B" => B,
            "C" => C,
            "D" => D,
            wrong => panic!("This is wrong: {}", wrong),
        })).flatten().collect::<Vec<_>>()
}

#[aoc_generator(day23, part2)]
pub fn gen2(input: &str) -> Vec<Amphipod> {
    let orig = gen1(input);
    let insert2rows = vec![D,C,B,A,D,B,A,C];
    orig.iter()
    .take(4)
    .chain(insert2rows.iter())
    .chain(orig.iter().skip(4).take(4))
    .map(|ptr|*ptr)
    .collect::<Vec<_>>()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day23, part1)]
pub fn part1(input: &[Amphipod]) -> usize {
    assert_eq!(input.len(), 8);
    0
}

#[aoc(day23, part2)]
pub fn part2(input: &[Amphipod]) -> usize {
    let mut gb = GameBoard::new(input);

    let animate = true; // animate or debug

    if animate {
        execute!(stdout(),Hide,Clear(ClearType::All),MoveTo(0,0)).unwrap()
    };
    println!("Initial configuration");
    if animate {
        execute!(stdout(),MoveTo(0,2),).unwrap()
    };
    gb.show();
    println!("\nFinding solutions...");

    let solns = gb.solve(Vec::new());
    println!("Found {} solution{}",solns.len(),if solns.len()==1 {" "} else {"s"});

    // 11943 is too low, obviously
    // 59489 is wrong
    // 59285 is wrong
    // 58439 is wrong

    let ans = *solns.keys().min().unwrap();
    
    // Perform diagnostic validation for this solution
    for (mov,target) in solns.get(&ans).unwrap().iter() {
        assert!(mov.1 != target.1); // All valid moves change columns
        assert!(mov.0 != 1 || target.0 != 1); // All valid moves involve a non-row 1
    }

    // Restarting for animation (debug) of solution:
    let mut gb = GameBoard::new(input);
    if animate {execute!(stdout(),Hide,Clear(ClearType::All),MoveTo(0,0)).unwrap()};
    println!("Length of soln: {}",solns.get(&ans).unwrap().len());
    for (mov,target) in solns.get(&ans).unwrap().iter() {
        gb.move_amphi(*mov,*target).unwrap();
        if animate {execute!(stdout(),MoveTo(0,2),).unwrap()};
        gb.show();
        if !animate {
            if let Amphi(amphi) = *gb.spaces.get(target).unwrap() {
                println!("  {} from {:?} to {:?} in {} steps",
                    amphi,mov,target,dist(*mov,*target));
            }
        }
        if animate {std::thread::sleep(std::time::Duration::from_millis(200))};
    }
    ans
}

#[derive(Debug, Clone)]
struct GameBoard {
    spaces: HashMap<(usize,usize), GameSpace>,
    score: usize,
}
impl GameBoard {
    fn my_home_col(amphi: Amphipod) -> usize {
        match amphi {
            A => 3,
            B => 5,
            C => 7,
            D => 9,
        }
    }
    fn show(&self) {
        println!("{}", self);
    }
    fn is_done(&self) -> bool {
        let mut out_of_place = false;
        for col in vec![3,5,7,9].into_iter() {
            let home_amphi = match col {
                3 => A,
                5 => B,
                7 => C,
                9 => D,
                _ => panic!("Nope"),
            };
            for row in 2..=5 {
                out_of_place = match self.spaces.get(&(row,col)).unwrap() {
                    Amphi(amphi) => *amphi != home_amphi,
                    _ => true,
                };
                if out_of_place {break;}
            }
            if out_of_place {break;}
        }
        !out_of_place
    }
    fn solve(&mut self, steps_to_get_here: Vec<((usize,usize),(usize,usize))>) -> HashMap<usize,Vec<((usize,usize),(usize,usize))>> {
        let mut solutions = HashMap::new();
        let valid_moves = self.find_moves();
        if !valid_moves.is_empty() {
            for (mov, target) in valid_moves {
                let mut alt_game = self.clone();
                alt_game.move_amphi(mov, target).unwrap();
                let mut one_step_closer = steps_to_get_here.clone();
                one_step_closer.push((mov,target));
                if alt_game.is_done() {
                    // OMG! Found a solution
                    solutions.insert(alt_game.score, one_step_closer);
                } else {
                    solutions.extend(alt_game.solve(one_step_closer));
                }
            }
        }
        solutions // is usually returned empty, but SOMETIMES with a solution!
    }
    fn find_moves(&self) -> Vec<((usize,usize),(usize,usize))> {
        let mut valid_moves = Vec::new();
        //
        // Find targets
        //
        let targets: Vec<(usize,usize)> = self.spaces.iter()
        .filter(|(_,target)| **target == Available) // Targets are Available spaces
         .map(|(coord, _)| *coord)
        .collect();
        // Find highest index in every home column
        let highest = targets.iter()
        .fold((usize::MIN,usize::MIN,usize::MIN,usize::MIN,), |mut high_so_far, (row,col)| {
            let row = *row;
            let col = *col;
            if row > 1 { // validate only home columns ( == rows above 1)
                match col {
                    3 => high_so_far.0 = high_so_far.0.max(row),
                    5 => high_so_far.1 = high_so_far.1.max(row),
                    7 => high_so_far.2 = high_so_far.2.max(row),
                    9 => high_so_far.3 = high_so_far.3.max(row),
                    inv => panic!("Invalid home column number: {}", inv),
                }
            }
            high_so_far
        });
        // For Targets in home columns, keep only the target with the highest index
        let targets: Vec<(usize,usize)> = targets.into_iter()
        .filter(|(row,col)|{
            let row = *row;
            let col = *col;
            if row > 1 { // is home column
                match col { // Keep highest ndx in each column 
                    3 => row == highest.0,
                    5 => row == highest.1,
                    7 => row == highest.2,
                    9 => row == highest.3,
                    inv => panic!("Invalid home column number: {}", inv),
                }
            } else {true}
        }).collect();
        //
        // Find Movers
        //
        // Movers, below, and Targets, above, build up with similar structure
        let movers: Vec<(usize,usize)> = self.spaces.iter()
        .filter(|(_,target)| **target != Available) // Movers are not Available spaces
         .map(|(coord, _)| *coord)
        .collect();
        let lowest = movers.iter()
        .fold((usize::MAX,usize::MAX,usize::MAX,usize::MAX,), |mut low_so_far, (row,col)| {
            let row = *row;
            let col = *col;
            if row > 1 { // validate only home columns ( == rows above 1)
                match col {
                    3 => low_so_far.0 = low_so_far.0.min(row),
                    5 => low_so_far.1 = low_so_far.1.min(row),
                    7 => low_so_far.2 = low_so_far.2.min(row),
                    9 => low_so_far.3 = low_so_far.3.min(row),
                    inv => panic!("Invalid home column number: {}", inv),
                }
            }
            low_so_far
        });
        // For Movers in home columns, keep only the mover with the lowest index
        let movers: Vec<(usize,usize)> = movers.into_iter()
        .filter(|(row,col)|{
            let row = *row;
            let col = *col;
            if row > 1 { // is home column
                match col { // Keep lowest ndx in each column 
                    3 => row == lowest.0,
                    5 => row == lowest.1,
                    7 => row == lowest.2,
                    9 => row == lowest.3,
                    inv => panic!("Invalid home column number: {}", inv),
                }
            } else {true} // keep all other movers
        }).collect();
        //
        // Validate each mover:
        //
        // 1) Movers should move out of their home only if an Amphi "below" them is other
        for (move_row,move_col) in movers.into_iter().filter(|(move_row,move_col)| {
            // 1) Movers should move out of their home only if an Amphi "below" them is other
            let me = if let Amphi(amphi) = self.spaces.get(&(*move_row,*move_col)).unwrap() {
                *amphi
            } else {
                panic!("Didn't find Amphi with validated move list.")
            };
            let home_col = Self::my_home_col(me);
            *move_row == 1 || if *move_col == home_col {
                let other_below = (move_row+1..=5)
                .fold(false,|found_other_below, row| {
                    found_other_below ||
                        if let Some(Amphi(other)) = self.spaces.get(&(row,*move_col)) {
                            *other != me
                        } else {
                            panic!("Amphi not found in move_col under move_row");
                        }
                });
                other_below
            } else {true} // We can always move out of someone else's home, assuming the path is clear
        }) {
            // Validate each target with these mover-specific rules:
            // 2) Movers can't move to the same column
            // 3) Row 1 movers must move out of Row 1
            // 4) Any mover to a home column must belong to that home,
            //    and the home must be empty of "others"
            // 5) The path from mover to target must be empty.
            let my_targets: Vec<(usize, usize)> = targets.iter()
            .filter(|(target_row,target_col)| {
                // 2) Movers can't move to the same column
                // 3) Row 1 movers must move out of Row 1
                move_col != *target_col &&
                !(move_row == 1 && *target_row == 1)
            })
            .filter(|(target_row,target_col)| {
                // 4) Any mover to a home column must belong to that home,
                //    and the home must be empty of "others"
                *target_row == 1 || {
                    if let Amphi(amphi) = self.spaces.get(&(move_row,move_col)).unwrap() {
                        let my_home_col = Self::my_home_col(*amphi);
                        *target_col == my_home_col
                        && (2..=5).fold(true, |clean_of_others, row| {
                            clean_of_others &&
                            match self.spaces.get(&(row,my_home_col)) {
                                Some(Available) => true,
                                Some(Amphi(amphi2)) => amphi2 == amphi,
                                _ => panic!("Should be Amphis or Availables in Home columns"),
                            }
                        })                   } else {
                        panic!("Didn't find Amphi with validated move list.")
                    }
                }
            })
            .filter(|(target_row,target_col)| {
                // Validate each target with these mover-specific rules:
                // 5) The path from mover to target must be empty.
                // o  Verify move_col contains Empty between move_row and Row 1 (exclusive on both ends)
                // o  Verify target_col is Empty between Row 1 and target_row
                // o  Verify Row 1 clear from move_col to target_col (check only even cols)
                (2..move_row).fold(true,|clear_so_far, row|
                    clear_so_far && Some(&Available) == self.spaces.get(&(row,move_col)))
                &&
                (2..*target_row).fold(true,|clear_so_far, row|
                    clear_so_far && Some(&Available) == self.spaces.get(&(row,*target_col)))
                && {
                    let mincol = target_col.min(&move_col)+1;
                    let maxcol = target_col.max(&move_col)-1;
                    (mincol..=maxcol).fold(true, |clear_so_far, col| clear_so_far && {
                        match self.spaces.get(&(1,col)) {
                            None => true, // Row 1 has some always-available spots not included in spaces
                            Some(&Available) => true, 
                            Some(&Amphi(_)) => false,
                            _ => panic!("Not sure what this is"),
                        }
                    })
                }
            })
            .map(|ptr|*ptr)
            .collect();
            //
            // Optimization: Greedy select moves to Home
            //
            let my_home_col = if let Amphi(amphi) = self.spaces.get(&(move_row,move_col)).unwrap() {
                Self::my_home_col(*amphi)
            } else {
                panic!("Already validated, can't fail");
            };
            let validated_moves_home = my_targets.iter()
            .filter(|(_,target_col)| *target_col == my_home_col)
            .collect::<Vec<_>>();
            if !validated_moves_home.is_empty() {
                valid_moves.clear();
                valid_moves.push(((move_row,move_col),*validated_moves_home[0]));
                return valid_moves;
            }
            //
            // Add to collection of validated moves
            //
            valid_moves.extend(my_targets.into_iter()
            .map(|(target_row, target_col)| {
                ((move_row,move_col),(target_row, target_col))
            }));
        }
        valid_moves
    }
    fn move_amphi(&mut self, from: (usize,usize), to: (usize,usize)) -> Result<(),()> {
        assert_eq!(*self.spaces.get(&to).unwrap(), Available);
        let amphi = if let Amphi(amphi) = *self.spaces.get(&from).unwrap() {
            amphi
        } else {
            panic!("Param 'from' did not contain an Amphi()!")
        };
        self.spaces.insert(to, Amphi(amphi));
        self.spaces.insert(from, Available);
        let dist = dist(from, to);
        let score = dist * match amphi {
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
        };
        self.score += score;
        Ok(())
    }
    fn new(input: &[Amphipod]) -> Self {
        assert_eq!(input.len(), 16);
        let spaces:HashMap<(usize,usize),GameSpace> = vec![
            (2,3),
            (2,5),
            (2,7),
            (2,9),
            (3,3),
            (3,5),
            (3,7),
            (3,9),
            (4,3),
            (4,5),
            (4,7),
            (4,9),
            (5,3),
            (5,5),
            (5,7),
            (5,9),
        ].into_iter().zip(input.iter())
        .map(|((y,x),amphi)| ((y,x),Amphi(*amphi)))
        .chain(vec![
            ((1,1),Available),
            ((1,2),Available),
            ((1,4),Available),
            ((1,6),Available),
            ((1,8),Available),
            ((1,10),Available),
            ((1,11),Available),
        ].into_iter())
        .collect();
        Self {spaces, score: 0}
    }    
}

// Manhattan distance with a mandatory pass through row 1 (for Bagels)
fn dist(from: (usize, usize), to: (usize, usize)) -> usize {
    (from.0 - 1) + (to.0 - 1) + from.1.max(to.1) - from.1.min(to.1)
}

impl fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board = [[Blank;13];7];
        board[0] = [Wall;13];
        board[1][0] = Wall;
        board[1][3] = Available;
        board[1][5] = Available;
        board[1][7] = Available;
        board[1][9] = Available;
        board[1][12] = Wall;
        board[2][0] = Wall;
        board[2][1] = Wall;
        board[2][11] = Wall;
        board[2][12] = Wall;
        for row in 2..6 {
            board[row][2] = Wall;
            board[row][4] = Wall;
            board[row][6] = Wall;
            board[row][8] = Wall;
            board[row][10] = Wall;
        }
        board[6] = [Blank,Blank,Wall,Wall,Wall,Wall,Wall,Wall,Wall,Wall,Wall,Blank,Blank];
        for ((y,x),gs) in self.spaces.iter() {
            board[*y][*x] = *gs;
        }
        for y in 0..7 {
            for x in 0..13 {
                write!(f, "{}", board[y][x])?;
            }
            write!(f, "\n")?;
        }        
        write!(f, "  ({})", self.score)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Amphipod {
    A,
    B,
    C,
    D,
}
impl fmt::Display for Amphipod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match *self {
            A => "A".red(),
            B => "B".dark_green(),
            C => "C".blue(),
            D => "D".magenta(),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameSpace {
    Blank,
    Wall,
    Available,
    Amphi(Amphipod),
}

impl fmt::Display for GameSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Blank => " ".to_string(),
            Wall => "#".white().to_string(),
            Available => ".".dark_grey().to_string(),
            Amphi(amphi) => amphi.to_string(),
        })
    }
}
// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part2() {
        let g = gen2(EX1);
        let p1 = part2(&g);
        assert_eq!(p1, 44169);
    }

const EX1: &'static str =
r"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

}