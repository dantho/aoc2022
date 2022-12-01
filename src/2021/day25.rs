
/// https://adventofcode.com/2021/day/25
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 
use colored::*;
use self::Cuc::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cuc {
    East,
    South,
    Empty,
}

pub type SeaCucs = Vec<Vec<Cuc>>;

fn print_sea_cucs(sea_cucs: &SeaCucs) {
    print!("{}[2J", 27 as char); // Clear Screen with Esc code
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Cursor at Row 1 Col 1
    for y in 0..sea_cucs.len() {
        println!();
        for x in 0..sea_cucs[0].len() {
            print!("{}", match sea_cucs[y][x] {
                East => ">".red(),
                South => "v".green(),
                Empty => ".".cyan(),
            });
        }
    }
    print!("\n");
}

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day25)]
pub fn gen1(input: &str) -> Vec<Vec<Cuc>> {
    input.lines()
    .map(|line|line.chars()
    .map(|c|match c {
        '>' => East,
        'v' => South,
        '.' => Empty,
        _ => panic!("Unexpected input"),
    }).collect::<Vec<_>>()).collect::<Vec<_>>()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day25, part1)]
pub fn part1(input: &[Vec<Cuc>]) -> usize {
    let mut input = input.to_vec();
    print_sea_cucs(&input);
    let mut any_movement_occurred;
    let eastlen = input[0].len();
    let southlen = input.len();
    let mut step_count = 0;
    loop {
        // East
        let mut movers = Vec::new();
        for y in 0..southlen {
            for x in 0..eastlen {
                if input[y][x] == East && input[y][(x+1)%eastlen] == Empty {
                    movers.push((x,y));
                }
            }
        }
        let movement_occurred = movers.len() > 0;
        any_movement_occurred = movement_occurred;
        for (x,y) in movers {
            input[y][x] = Empty;
            input[y][(x+1)%eastlen] = East;
        }
        // Show movement
        if movement_occurred {
            print_sea_cucs(&input)
        };

        //South
        let mut movers = Vec::new();
        for y in 0..southlen {
            for x in 0..eastlen {
                if input[y][x] == South && input[(y+1)%southlen][x] == Empty {
                    movers.push((x,y));
                }
            }
        }
        let movement_occurred = movers.len() > 0;
        for (x,y) in movers {
            input[y][x] = Empty;
            input[(y+1)%southlen][x] = South;
        }
        // Show movement
        if movement_occurred {
            print_sea_cucs(&input)
        };
        any_movement_occurred = any_movement_occurred || movement_occurred;
        step_count += 1;
        if !any_movement_occurred {break step_count}
    }
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        let g = gen1(EX1);
        assert_eq!(g.len(), 9);
    }

    #[test]
    fn test_ex1_part1() {
        let g = gen1(EX1);
        let p1 = part1(&g);
        assert_eq!(p1, 58);
    }

const EX1: &'static str =
r"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

}