/// https://adventofcode.com/2022/day/22
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754
use crate::day22::Dir::*;
use std::iter::{once, repeat};
use std::fmt::Display;
use std::collections::HashMap;

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day22)]
pub fn gen1(input: &str) -> (Vec<Vec<char>>, String) {
    let movement = input.lines().last().unwrap().to_string();
    let lines = input.lines().collect::<Vec<_>>(); 
    let map: Vec<Vec<char>> = lines[..lines.len()-2].iter() //  All but last 2 rows
        .map(|line|line.chars().collect::<Vec<_>>())
        .collect();
    let max_col = map.iter().map(|row|row.len()).max().unwrap();
    let map: Vec<Vec<char>> = map.into_iter()
        .map(|row| row.into_iter()
            .chain(repeat(' '))
            .take(max_col+1)
            .filter(|&c| c ==' ' || c =='.' || c =='#')
            .collect::<Vec<char>>())
        .collect();
    (map, movement)
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day22, part1)]
pub fn part1(input: &(Vec<Vec<char>>, String)) -> usize {
    let (raw_map, movement) = input;
    let pw_board = PasswordBoard::new(raw_map);
    let just_nums: Vec<usize> = movement.chars()
        .map(|c| if c.is_numeric() {c} else {' '})
        .collect::<String>().split(' ')
        .filter(|s|!s.is_empty())
        .map(|s|s.parse().unwrap())
        .collect();
    let just_dirs: Vec<char> = movement.chars()
        .map(|c| if !c.is_numeric() {c} else {' '})
        .collect::<String>().split(' ')
        .filter(|s|!s.is_empty())
        .map(|s|{assert!(s.len()== 1); s.chars().nth(0).unwrap()})
        .collect();
    assert_eq!(just_nums.len(), just_dirs.len()+1);
    let prefixed_dirs = once(&'R').chain(just_dirs.iter()); // Add an extra Right turn at start for uniformity
    let moves = prefixed_dirs.zip(just_nums.iter()).collect::<Vec<_>>();

    let mut facing = Right;
    let mut pos = (0,0);
    if pw_board.map_c(pos) != '.' {
        pos = pw_board.move_if_possible(pos, facing, 1);
    }
    assert_eq!(pw_board.map_c(pos), '.');
    facing = facing.turn(Left); // Anticipate extra Right turn we added at start, above

    moves.iter().for_each(|(&turn_c, &steps)| {
        assert_eq!(pw_board.map[pos.0][pos.1], '.');
        facing = facing.turn(Dir::from(turn_c));
        pos = pw_board.move_if_possible(pos, facing, steps);
    });

    let final_row = pos.0+1;
    let final_col = pos.1+1;
    let pw = 1000 * final_row + 4 * final_col + facing as usize;
    pw
}

#[aoc(day22, part2)]
pub fn part2(input: &(Vec<Vec<char>>, String)) -> usize {
    let (raw_map, movement) = input;
    let pw_board = PasswordBoard::new(raw_map);
    let just_nums: Vec<usize> = movement.chars()
        .map(|c| if c.is_numeric() {c} else {' '})
        .collect::<String>().split(' ')
        .filter(|s|!s.is_empty())
        .map(|s|s.parse().unwrap())
        .collect();
    let just_dirs: Vec<char> = movement.chars()
        .map(|c| if !c.is_numeric() {c} else {' '})
        .collect::<String>().split(' ')
        .filter(|s|!s.is_empty())
        .map(|s|{assert!(s.len()== 1); s.chars().nth(0).unwrap()})
        .collect();
    assert_eq!(just_nums.len(), just_dirs.len()+1);
    let prefixed_dirs = once(&'R').chain(just_dirs.iter()); // Add an extra Right turn at start for uniformity
    let moves = prefixed_dirs.zip(just_nums.iter()).collect::<Vec<_>>();

    let mut facing = Right;
    let mut pos = pw_board.abs_pos((0,0), (0,2));
    assert_eq!(pw_board.map_c(pos), '.');
    assert_eq!(pos, (0,pw_board.map[0].len()/4*2));
    facing = facing.turn(Left); // Anticipate extra Right turn we added at start, above

    #[cfg(test)]
    {
        println!("{}",pw_board);
        println!();
        println!("pos[0][8] is '{}'",pw_board.map[0][8]);
    }
    moves.iter()
    .for_each(|(&turn_c, &steps)| {
        facing = facing.turn(Dir::from(turn_c));
        #[cfg(test)]
        println!("at {:?} facing {:?} and about to move {}", pos, facing, steps);
        assert_eq!(pw_board.map[pos.0][pos.1], '.');
        (pos, facing) = pw_board.move_in_3d(pos, facing, steps);
    });

    let final_row = pos.0+1;
    let final_col = pos.1+1;
    let pw = 1000 * final_row + 4 * final_col + facing as usize;
    pw
}


#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
enum Dir {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3
}

impl Dir {
    fn turn(&self, left_or_right: Dir) -> Dir {
        match left_or_right {
            Right => match *self {
                Right => Down,
                Down => Left,
                Left => Up,
                Up => Right,
            },
            Left => match *self {
                Right => Up,
                Down => Right,
                Left => Down,
                Up => Left,
            },
            _ => panic!("Only left or right turns expected."),
        }
    }
}

impl From<char> for Dir {
    fn from(dir_char: char) -> Self {
        match dir_char {
            'R' => Right,
            'D' => Down,
            'L' => Left,
            'U' => Up,
            bad => panic!("Expecting 'R', 'L', 'U', or 'D', got '{}'", bad),
        }
    }
}

type FoldMap = HashMap<((usize, usize), Dir), ((usize, usize), Dir, bool, bool)>;

struct PasswordBoard {
    map: Vec<Vec<char>>,
    fold_map: FoldMap,
    quad_rows: usize,
    quad_cols: usize,
}

impl PasswordBoard {
    fn new(map: &[Vec<char>]) -> Self {
        let (quad_rows, quad_cols) = if map.len() > map[0].len() {
            (4,3)
        } else {
            (3,4)
        };
        let map = map.to_vec();
        #[cfg(test)]
        let fold_map = [
            // from dir       to    dir   is_rev is_transposed
            (((0,2), Up),   ((1,0), Down,  true)),
            (((0,2), Down), ((1,2), Down,  false)),
            (((0,2), Left), ((1,1), Down,  false)),
            (((0,2), Right),((2,3), Left,  false)),
  
            (((1,0), Up),   ((0,2), Down,  false)),
            (((1,0), Down), ((2,2), Up,    false)),
            (((1,0), Left), ((2,3), Up,    false)),
            (((1,0), Right),((1,1), Right, false)),
  
            (((1,1), Up),   ((0,2), Right, false)),
            (((1,1), Down), ((2,2), Right, true)),
            (((1,1), Left), ((1,0), Left,  false)),
            (((1,1), Right),((1,2), Right, false)),
  
            (((1,2), Up),   ((0,2), Up,    false)),
            (((1,2), Down), ((2,2), Down,  false)),
            (((1,2), Left), ((1,1), Left,  false)),
            (((1,2), Right),((2,3), Down,  true)),
  
            (((2,2), Up),   ((1,2), Up,    false)),
            (((2,2), Down), ((1,0), Up,    true)),
            (((2,2), Left), ((1,1), Up,    true)),
            (((2,2), Right),((2,3), Right, false)),
  
            (((2,3), Up),   ((1,2), Left,  true)),
            (((2,3), Down), ((1,0), Right, true)),
            (((2,3), Left), ((2,2), Left,  false)),
            (((2,3), Right),((0,2), Left,  true)),
        ];
        #[cfg(not(test))]
        let fold_map = [
            // from dir       to    dir   is_rev is_transposed
            (((0,1), Up),   ((3,0), Right, false)),
            (((0,1), Down), ((1,1), Down, false)),
            (((0,1), Left), ((2,0), Right, true)),
            (((0,1), Right),((0,2), Right, false)),

            (((0,2), Up),   ((3,0), Up, false)),
            (((0,2), Down), ((1,1), Left, false)),
            (((0,2), Left), ((0,1), Left, false)),
            (((0,2), Right),((2,1), Left, true)),
  
            (((1,1), Up),   ((0,1), Up, false)),
            (((1,1), Down), ((2,1), Down, false)),
            (((1,1), Left), ((2,0), Down, false)),
            (((1,1), Right),((0,2), Up, false)),
  
            (((2,0), Up),   ((1,1), Right, false)),
            (((2,0), Down), ((3,0), Down, false)),
            (((2,0), Left), ((0,1), Right, true)),
            (((2,0), Right),((2,1), Right, false)),
  
            (((2,1), Up),   ((1,1), Up, false)),
            (((2,1), Down), ((3,0), Left, false)),
            (((2,1), Left), ((2,0), Left, false)),
            (((2,1), Right),((0,2), Left, true)),
  
            (((3,0), Up),   ((2,0), Up, false)),
            (((3,0), Down), ((0,2), Down, false)),
            (((3,0), Left), ((0,1), Down, false)),
            (((3,0), Right),((2,1), Up, false)),
        ];
        let fold_map: FoldMap = fold_map.to_vec().into_iter()
        .map(|((from_q,from_dir),(to_q,to_dir,to_rev))| {
            let to_transpose = match from_dir {
                Up => to_dir == Left || to_dir == Right,
                Down => to_dir == Left || to_dir == Right,
                Left => to_dir == Up || to_dir == Down,
                Right => to_dir == Up || to_dir == Down,
            };
            ((from_q,from_dir),(to_q,to_dir,to_rev,to_transpose))
        }).collect::<FoldMap>();

        // Above maps are manually entered and determined by inspection
        // in 3-space with folded paper.  This is an error-prone process.
        // Fortunately, due to symmetries, we can perform many checks:
        let quad_cnts = fold_map.iter().fold(HashMap::new(),|mut h,(_,(quad,_,_,_))| {
            h.entry(quad).and_modify(|c: &mut u8| *c += 1).or_insert(1);
            h 
        });
        let dir_cnts = fold_map.iter().fold(HashMap::new(),|mut h,(_,(_,dir,_,_))| {
            h.entry(dir).and_modify(|c: &mut u8| *c += 1).or_insert(1);
            h 
        });
        println!("quad_cnts: {:?}",quad_cnts);
        assert_eq!(6, quad_cnts.values().filter(|cnt|**cnt==4).count()); // All 6 quadrants should be entered from all 4 sides
        assert_eq!(0, quad_cnts.values().filter(|cnt|**cnt!=4).count());
        println!("{:?}",dir_cnts);
        assert_eq!(4, dir_cnts.values().filter(|cnt|**cnt==6).count());  // Every DIR should be specified exactly 6 times
        PasswordBoard { map, fold_map, quad_rows, quad_cols }
    }

    fn map_c(&self, pos: (usize, usize)) -> char {
        self.map[pos.0][pos.1]
    }

    fn quadrant(&self, abs_pos: (usize, usize)) -> (usize, usize) {
        let quad_size = self.map.len() / 3;
        (abs_pos.0 / quad_size, abs_pos.1 / quad_size)
    }

    fn next_pos(&self, pos: (usize, usize), facing: Dir) -> (usize, usize) {
        let rows = self.map.len();
        let cols = self.map[0].len();
        match facing {
            Right => (pos.0 % rows, (pos.1 + 1) % cols),
            Down  => ((pos.0 + 1) % rows, pos.1 % cols),
            Left  => (pos.0 % rows, (pos.1 + cols - 1) % cols),
            Up    => ((pos.0 + rows - 1) % rows, pos.1 % cols),
        }
    } 

    fn move_if_possible(&self, mut pos: (usize, usize), facing: Dir, steps: usize) -> (usize, usize) {
        let dbg = false;
        let mut next_p = self.next_pos(pos, facing);
        for _step in 0..steps {
            if dbg {println!("pos could be {:?}", next_p);}
            let mut next_c = self.map_c(next_p);
            while next_c == ' ' {
                if dbg {println!("blank at {:?}", next_p);}
                next_p = self.next_pos(next_p, facing);
                next_c = self.map_c(next_p);
            }
            pos = match next_c {
                '.' => next_p, // OK, move
                '#' => pos, // Blocked, don't move
                ' ' => panic!("above logic for skipping ' ' is broken."), 
                bad => panic!("Illegal char '{}' found on map at {:?}", bad, next_p),
            };
            next_p = self.next_pos(pos, facing);
        }
        if dbg {println!("pos is {:?}", pos);}
        pos
    }

    fn next_pos_3d(&self, pos: (usize, usize), facing: Dir) -> ((usize, usize), Dir) {
        let quad_size = self.map.len() / self.quad_rows;
        assert_eq!(quad_size, self.map[0].len() / self.quad_cols);
        let qmax = quad_size-1;
        // If we're not moving around an edge, processing is 2D, simple.
        // If we _are_ moving around an edge (in unwrapped 3-space) then a LOT changes!!
        // Edges cross modulo row/col boundaries
        let qpos = (pos.0 % quad_size, pos.1 % quad_size);
        let quadrant = (pos.0 / quad_size, pos.1 / quad_size);
        let edge_detected =
               facing == Up && qpos.0 == 0
            || facing == Down && qpos.0 == qmax
            || facing == Left && qpos.1 == 0
            || facing == Right && qpos.1 == qmax;
        if edge_detected {
            // Edge transition logic.  Warning!
            // See Part 2 here, https://adventofcode.com/2022/day/22, to comprehend unwrapped 3-space
            // We are in COMPLEX land here -- interpretting an unwrapped map in 3-space as a folded cube!
            let (new_quadrant, new_facing, is_reversed, is_transposed) = self.fold_map[&(quadrant, facing)];
            let qpos = if is_reversed {
                (qmax-qpos.0,qmax-qpos.1)
            } else {qpos};
            let qpos = if is_transposed {
                (qpos.1, qpos.0)
            } else {qpos};
            let new_qpos = match new_facing {
                    Up => (qmax, qpos.1),
                    Down => (0, qpos.1),
                    Left => (qpos.0, qmax),
                    Right => (qpos.0, 0),
                };
            let new_pos = self.abs_pos(new_qpos, new_quadrant);
            (new_pos, new_facing)
        } else {
            let new_pos = match facing {
                Right => self.abs_pos((qpos.0, qpos.1 + 1), quadrant),
                Down  => self.abs_pos((qpos.0 + 1, qpos.1), quadrant),
                Left  => self.abs_pos((qpos.0, qpos.1 - 1), quadrant),
                Up    => self.abs_pos((qpos.0 - 1, qpos.1), quadrant),
            };
            (new_pos, facing)
        }
    }

    fn move_in_3d(&self, mut pos: (usize, usize), mut facing: Dir, steps: usize) -> ((usize, usize), Dir) {
        let dbg = true;
        let (mut next_p, mut next_f) = self.next_pos_3d(pos, facing);
        for _step in 0..steps {
            if dbg {println!("pos could be {:?} in quadrant {:?}", next_p, self.quadrant(next_p));}
            let next_c = self.map_c(next_p);
            (pos, facing) = match next_c {
                '.' => (next_p, next_f), // OK, move
                '#' => (pos, facing), // Blocked, don't move
                ' ' => panic!("Encountered ' ' at pos {:?}, so 3D logic is broken.", next_p),
                bad => panic!("Illegal char '{}' found on map at {:?}", bad, next_p),
            };
            (next_p, next_f) = self.next_pos_3d(pos, facing);
        }
        if dbg {println!("pos is {:?} in quadrant {:?}", pos, self.quadrant(pos));}
        (pos, facing)
    }

    // Given quadrant = (abs_pos.0 / quad_size, abs_pos.1 / quad_size), this is the reverse;
    fn abs_pos(&self, rel_pos: (usize, usize), quad: (usize, usize)) -> (usize, usize) {
        assert!(quad.0 < self.quad_rows);
        assert!(quad.1 < self.quad_cols);
        let abs_rows = self.map.len();
        let abs_cols = self.map[0].len();
        let quad_size = abs_rows / self.quad_rows;
        assert_eq!(abs_cols / self.quad_cols, quad_size);
        assert!(rel_pos.0 < quad_size);
        assert!(rel_pos.1 < quad_size);
        
        (rel_pos.0 + quad.0*quad_size, rel_pos.1 + quad.1*quad_size)
    }

}

impl Display for PasswordBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}


// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 6032);
    }

    #[test]
    fn test_ex1_part2() {
        assert_eq!(part2(&gen1(EX1)), 5031);
    }

    const EX1: &'static str =
r"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
}
