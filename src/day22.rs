use std::{iter::{self, once, repeat}, fmt::Display};

/// https://adventofcode.com/2022/day/22
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754
use crate::day22::Dir::*;

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day22)]
pub fn gen1(input: &str) -> (Vec<Vec<char>>, String) {
    let movement = input.lines().last().unwrap().to_string();
    let lines = input.lines().collect::<Vec<_>>(); 
    let map: Vec<Vec<char>> = lines[..lines.len()-2].iter() // All but last 2 rows
        .map(|line|line.chars().collect::<Vec<_>>())
        .collect();
    let max_col = map.iter().map(|row|row.len()).max().unwrap();
    let map: Vec<Vec<char>> = map.into_iter()
        .map(|row| row.into_iter()
            .chain(repeat(' '))
            .take(max_col+1)
            .collect::<Vec<char>>())
        .collect();
    (map, movement)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

struct PasswordBoard {
    map: Vec<Vec<char>>,
}

impl PasswordBoard {
    fn map_c(&self, pos: (usize, usize)) -> char {
        self.map[pos.0][pos.1]
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

    fn next_pos_3D(&self, pos: (usize, usize), facing: Dir) -> ((usize, usize), Dir) {
        let meta_rows = self.map.len();
        let meta_cols = self.map[0].len();
        let rows = meta_rows / 3;
        let cols = meta_cols / 4;
        // If we're not moving off an edge, processing is 2D, same as next_pos()
        // If we _are_ moving off an edge (in unwrapped 3-space) then a LOT changes!!
        let is_edge = pos.0 % rows == 0 || pos.0 % rows == rows-1
                && pos.1 % cols == 0 || pos.1 % cols == cols-1;
        if is_edge {
            todo!();
        } else {
            (match facing {
                Right => (pos.0 % rows, (pos.1 + 1) % cols),
                Down  => ((pos.0 + 1) % rows, pos.1 % cols),
                Left  => (pos.0 % rows, (pos.1 + cols - 1) % cols),
                Up    => ((pos.0 + rows - 1) % rows, pos.1 % cols),
            }, facing)
        }
    }

    fn move_if_possible(&self, mut pos: (usize, usize), facing: Dir, steps: usize) -> (usize, usize) {
        let dbg = false;
        let mut next_p = self.next_pos(pos, facing);
        for _step in 0..steps {
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
            if dbg {println!("pos could be {:?}", next_p);}
        }
        if dbg {println!("pos is {:?}", pos);}
        pos
    }

    fn move_in_3d(&self, mut pos: (usize, usize), mut facing: Dir, steps: usize) -> ((usize, usize), Dir) {
        let dbg = false;
        let (mut next_p, mut next_f) = self.next_pos_3D(pos, facing);
        for _step in 0..steps {
            let mut next_c = self.map_c(next_p);
            (pos, facing) = match next_c {
                '.' => (next_p, next_f), // OK, move
                '#' => (pos, facing), // Blocked, don't move
                ' ' => panic!("Encountered ' ', so 3D logic is broken."), 
                bad => panic!("Illegal char '{}' found on map at {:?}", bad, next_p),
            };
            (next_p, next_f) = self.next_pos_3D(pos, facing);
            if dbg {println!("pos could be {:?}", next_p);}
        }
        if dbg {println!("pos is {:?}", pos);}
        (pos, facing)
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

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day22, part1)]
pub fn part1(input: &(Vec<Vec<char>>, String)) -> usize {
    let (raw_map, movement) = input;
    let pw_board = PasswordBoard { map: raw_map.to_vec() };
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

    moves.iter()
    .for_each(|(&turn_c, &steps)| {
        // println!("at {:?} facing {:?} about to turn {:?} and move {}", pos, facing, Dir::from(turn_c), steps);
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
    let pw_board = PasswordBoard { map: raw_map.to_vec() };
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
    let mut pos = (0, pw_board.map.len() / 4 * 3 - 1);
    assert_eq!(pw_board.map_c(pos), '.');
    facing = facing.turn(Left); // Anticipate extra Right turn we added at start, above

    moves.iter()
    .for_each(|(&turn_c, &steps)| {
        // println!("at {:?} facing {:?} about to turn {:?} and move {}", pos, facing, Dir::from(turn_c), steps);
        assert_eq!(pw_board.map[pos.0][pos.1], '.');
        facing = facing.turn(Dir::from(turn_c));
        (pos, facing) = pw_board.move_in_3d(pos, facing, steps);
    });

    let final_row = pos.0+1;
    let final_col = pos.1+1;
    let pw = 1000 * final_row + 4 * final_col + facing as usize;
    pw
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

    // #[test]
    // fn test_ex1_part2() {
    //     assert_eq!(part2(&gen1(EX1)), 45000);
    // }

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
