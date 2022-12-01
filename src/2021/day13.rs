use std::collections::HashSet;

/// https://adventofcode.com/2021/day/13
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 
use colored::*;

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day13)]
pub fn gen1(input: &str) -> (Vec<(usize,usize)>,Vec<(char,usize)>) {
    let mut upper_lower = input.split("\n\n");
    let dots = upper_lower.next().unwrap();
    let folds = upper_lower.next().unwrap();
    let dots = dots.lines().map(|l|{
        let mut coords = l.split(",");
        let x = coords.next().unwrap().parse().unwrap();
        let y = coords.next().unwrap().parse().unwrap();
        (x,y)
    }).collect();
    let folds = folds.lines().map(|l|{
        let xy = 11usize;
        assert_eq!(&l[0..xy], "fold along ");
        assert_eq!(l.chars().nth(xy+1).unwrap(), '=');
        let fold_pos = (&l[xy+2..]).parse().unwrap();
        let fold_dir = l.chars().nth(xy).unwrap();
        assert!(fold_dir == 'x' || fold_dir == 'y');
        (fold_dir,fold_pos)
    }).collect();
    (dots,folds)
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day13, part1)]
pub fn part1(input: &(Vec<(usize,usize)>,Vec<(char,usize)>)) -> usize {
    let mut dots:HashSet<(usize,usize)> = input.0.iter()
        .map(|(x,y)|(*x,*y))
        .collect();
    for (xy,n) in &input.1 {
        let n = *n;
        dots = dots.into_iter()
        .map(|(x,y)|{
            if *xy == 'x' {
                if x > n {(n-(x-n),y)} else {(x,y)}
            } else {
                if y > n {(x,n-(y-n))} else {(x,y)}
            }
        }).collect();
        break;  // Iterate just once
    };
    dots.len()
}

#[aoc(day13, part2)]
pub fn part2(input: &(Vec<(usize,usize)>,Vec<(char,usize)>)) -> usize {
    let mut dots:HashSet<(usize,usize)> = input.0.iter()
        .map(|(x,y)|(*x,*y))
        .collect();
    for (xy,n) in &input.1 {
        let n = *n;
        dots = dots.into_iter()
        .map(|(x,y)|{
            if *xy == 'x' {
                if x > n {(n-(x-n),y)} else {(x,y)}
            } else {
                if y > n {(x,n-(y-n))} else {(x,y)}
            }
        }).collect();
    };
    // Convert dots to printout
    let max_x = *dots.iter().map(|(x,_)|x).max().unwrap();
    let max_y = *dots.iter().map(|(_,y)|y).max().unwrap();
    let mut grid_of_dots = Vec::with_capacity(max_y+1);
    for y in 0..=max_y {
        grid_of_dots.push(vec![".".red(); max_x+1]);
    }
    for (x,y) in dots {
        grid_of_dots[y][x] = "#".bright_green();
    }
    for line in grid_of_dots {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
    0
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
        assert_eq!(g.0.len(), 18);
        assert_eq!(g.1.len(), 2);
    }

    #[test]
    fn test_ex1_part1() {
        let g = gen1(EX1);
        let p1 = part1(&g);
        assert_eq!(p1, 17);
    }

const EX1: &'static str =
r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

const EX2: &'static str =
r"
";

}