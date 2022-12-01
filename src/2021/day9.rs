/// https://adventofcode.com/2021/day/9
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 
/// 
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax 
// extern crate regex;
// use self::regex::{Captures, Regex};

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day9)]
pub fn gen1(input: &str) -> Vec<Vec<u16>> {
    let mut grid: Vec<Vec<u16>> = input.lines()
        .map(|l| l.chars()
            .map(|n|n.to_string().parse::<u16>().unwrap())
            .collect::<Vec<u16>>())
        .collect();
    for row in &mut grid {
        row.reverse();
        row.push(9);
        row.reverse();
        row.push(9);
    }
    // Pad with border of 9's
    let row_of_9s: Vec<_> = grid[0].iter().map(|_|9).collect();
    grid.reverse();
    grid.push(row_of_9s.clone());
    grid.reverse();
    grid.push(row_of_9s);

    grid
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day9, part1)]
pub fn part1(input: &[Vec<u16>]) -> usize {
    let tuple_grid: Vec<Vec<(u16,u16,u16,u16,u16)>> = input.iter()
        .zip(input.iter().skip(1))
        .zip(input.iter().skip(2))
        .map(|((above,middle),below)|
            middle.iter()
            .zip(middle.iter().skip(1))
            .zip(middle.iter().skip(2))
            .zip(above.iter().skip(1))
            .zip(below.iter().skip(1))
            .map(|((((&left,&center),&right),&above),&below)|(center,above,below,left,right))
            .collect())
        .collect();
    let tuple_local_minima: Vec<(u16,u16,u16,u16,u16)> = tuple_grid.into_iter()
        .map(|v|v.into_iter())
        .flatten()
        .filter(|&(c,a,b,l,r)|c<a && c<b && c<l && c<r)
        .collect();
    tuple_local_minima.iter()
        .map(|tup|tup.0 as usize + 1)
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &[Vec<u16>]) -> u32 {
    let mut input = input.to_vec(); // Need ownership to mutate at bottom
    let tuple_grid: Vec<Vec<(u16,u16,u16,u16,u16)>> = input.iter()
        .zip(input.iter().skip(1))
        .zip(input.iter().skip(2))
        .map(|((above,middle),below)|
            middle.iter()
            .zip(middle.iter().skip(1))
            .zip(middle.iter().skip(2))
            .zip(above.iter().skip(1))
            .zip(below.iter().skip(1))
            .map(|((((&left,&center),&right),&above),&below)|(center,above,below,left,right))
            .collect())
        .collect();
    let row_size = tuple_grid[0].len();
    let coordinated_local_minima: Vec<(usize,usize,u16)> = tuple_grid.into_iter()
        .map(|v|v.into_iter())
        .flatten()
        .enumerate()
        .filter(|&(i,(c,a,b,l,r))|c<a && c<b && c<l && c<r)
        .map(|(i,tup)|{
            let y = i / row_size;
            let x = i - y*row_size + 1; // +1 accounts for input's padding
            let y = y + 1; // +1 accounts for input's padding
            assert!(x>0);
            assert!(y>0);
            assert!(x<input[0].len());
            assert!(y<input.len());
            (x,y,tup.0)
        })
        .collect();
    // Quick verification against Part 1's different solution
    // tuple_local_minima_with_coords.iter()
    //     .map(|tup|tup.2 as usize + 1)
    //     .sum()
    let mut basin_sizes: Vec<u32> = coordinated_local_minima.into_iter()
        .map(|(x,y,_)|fill9_and_count(&mut input, x, y))
        .collect();
        basin_sizes.sort();
        basin_sizes.reverse();
        basin_sizes[0]*basin_sizes[1]*basin_sizes[2]
}

fn fill9_and_count(grid: &mut [Vec<u16>], x: usize, y: usize) -> u32 {
    if grid[y][x] == 9 {
        0
    } else {
        grid[y][x] = 9; // Fill this location, to avoid recounting it
        1 +
        fill9_and_count(grid, x-1, y) +
        fill9_and_count(grid, x+1, y) +
        fill9_and_count(grid, x, y-1) +
        fill9_and_count(grid, x, y+1)
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
        let ydim = g.len();
        let xdim = g[0].len();
        assert_eq!(xdim, 12);
        assert_eq!(ydim, 7);
    }

    #[test]
    fn test_ex1_part1() {
        let g = gen1(EX1);
        let p1 = part1(&g);
        assert_eq!(p1, 15);
    }

    #[test]
    fn test_ex1_part2() {
        let g = gen1(EX1);
        let p1 = part2(&g);
        assert_eq!(p1, 1134);
    }

const EX1: &'static str =
r"2199943210
3987894921
9856789892
8767896789
9899965678";

const EX2: &'static str =
r"
";

}