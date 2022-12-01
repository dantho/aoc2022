/// https://adventofcode.com/2021/day/11
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 
use colored::*;

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day11)]
pub fn gen1(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l|l.chars().map(|c|c as u8 - '0' as u8).collect::<Vec<_>>()).collect::<Vec<_>>()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day11, part1)]
pub fn part1(input: &[Vec<u8>]) -> usize {
    let mut octopi = input.to_vec();
    let mut flash_count = 0;
    for _ in &octopi {
        println!("");
    }
    println!("");
    println!("");
    for _step in 1..=100 {
        take_step(&mut octopi);
        flash(&mut octopi);
        flash_count += octopi.iter().map(|v|v.iter()).flatten().filter(|&&e|e==0).count();
    }
    flash_count
}

#[aoc(day11, part2)]
pub fn part2(input: &[Vec<u8>]) -> usize {
    let mut octopi = input.to_vec();
    for _ in &octopi {
        println!("");
    }
    println!("");
    for step in 1..=1000 {
        take_step(&mut octopi);
        flash(&mut octopi);
        let non_flash_count = octopi.iter().map(|v|v.iter()).flatten().filter(|&&e|e!=0).count();
        if non_flash_count == 0 {return step;}
    }
    0
}

fn take_step(octopi: &mut [Vec<u8>]) {
    for row in 0..octopi.len() {
        for col in 0..octopi[0].len() {
            let oct = &mut octopi[row][col];
            *oct += 1;
        }
    }
}

fn flash(octopi: &mut [Vec<u8>]) {
    for row in 0..octopi.len() {
        for col in 0..octopi[0].len() {
            let oct = &mut octopi[row][col];
            if *oct >= 10 {
                flash_pt(octopi, row, col);
                print_grid(octopi);
            }
        }
    }
}

fn flash_pt(octopi: &mut [Vec<u8>], y: usize, x: usize) {
    octopi[y][x] = 0;
    let ymin = if y == 0 {0} else {y-1};
    let ymax = (octopi.len()-1).min(y+1);
    let xmin = if x == 0 {0} else {x-1};
    let xmax = (octopi[0].len()-1).min(x+1);
    for yy in ymin..=ymax {
        for xx in xmin..=xmax {
            let oct =  &mut octopi[yy][xx];
            if *oct > 0 {
                *oct += 1;
            }
            if *oct >= 10 {
                flash_pt(octopi, yy, xx);
                print_grid(octopi);
            }
        }
    }
}

fn print_grid(octopi:&[Vec<u8>]) {
    print!("\x1B[{}A", octopi.len()+2);
    println!("");
    for row in octopi {
        for v in row {
            print!("{}", match v {
                0 => "0".bold().bright_white(),
                9 => "9".red(),
                n if *n>9 => "*".red(),
                n => n.to_string().yellow(),
            });
        }
        println!("");
    }
    println!("");
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
        assert_eq!(g.len(), 999);
    }

    #[test]
    fn test_ex1_part1() {
        let g = gen1(EX1);
        let p1 = part1(&g);
        assert_eq!(p1, 999);
    }

const EX1: &'static str =
r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

const EX2: &'static str =
r"11111
19991
19191
19991
11111";

}