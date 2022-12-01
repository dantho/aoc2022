/// https://adventofcode.com/2021/day/15
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day15)]
pub fn gen1(input: &str) -> Vec<Vec<u32>> {
    input.lines()
    .map(|l|l.chars()
        .map(|c|c as u32 - '0' as u32).collect::<Vec<_>>()
    ).collect::<Vec<_>>()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day15, part1)]
pub fn part1(input: &[Vec<u32>]) -> u32 {
    let mut explored:Vec<Vec<u32>> = input.iter()
    .map(|row|row.iter()
        .map(|_|u32::MAX).collect::<Vec<_>>()
    ).collect::<Vec<_>>();
    explored[0][0] = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            let path_so_far = explored[y][x];
            for &(xx,yy) in &[(x+1,y),(x,y+1)] {
                if yy < input.len() && xx < input[0].len() {
                    let risk  = input[yy][xx];
                    explored[yy][xx] = explored[yy][xx].min(path_so_far+risk);
                }
            }
        }
    }
    explored[explored.len()-1][explored[0].len()-1]
}

#[aoc(day15, part2)]
pub fn part2(input: &[Vec<u32>]) -> u32 {
    let height = input.len();
    let width = input[0].len();
    // copy in x dir
    let mut bigger = Vec::new();
    for y in 0..height {
        bigger.push(Vec::new());
        for _xx in 0..5 {
            bigger[y].append(&mut input[y].clone());
        }
    }
    // copy in y dir
    for _yy in 1..5 {
        for y in 0..height {
            bigger.push(bigger[y].clone());
        }
    }
    // include adder and Mod 10
    for yy in 0..5 {
        for xx in 0..5 {
            for y in 0..height {
                for x in 0..width {
                    let loc = &mut bigger[yy*height+y][xx*width+x];
                    *loc += xx as u32 + yy as u32;
                    if *loc >= 10 {*loc += 1;}
                    *loc %= 10;
                }
            }
        }
    }

    let input = bigger;
    let height = input.len();
    let width = input[0].len();

    let mut explored = input.clone();
    for y in 0..height {
        for x in 0..width {
            explored[y][x] = u32::MAX;
        }
    }
    explored[0][0] = 0;

    for y in 0..height {
        for x in 0..width {
            assert!(input[y][x] > 0);
            assert!(input[y][x] < 10);
        }
    }

    for y in 0..height {
        for x in 0..width {
            let path_so_far = explored[y][x];
            for &(xx,yy) in &[(x+1,y),(x,y+1)] {
                if yy < height && xx < width {
                    let risk  = input[yy][xx];
                    explored[yy][xx] = explored[yy][xx].min(path_so_far+risk);
                }
            }
        }
    }
    // iterate/scrub for upward/leftward paths
    loop {
        let mut not_found = true; // Optimism
        for y in 0..height {
            for x in 0..width {
                let path_so_far = explored[y][x];
                for &(xx,yy) in &[(x+1,y),(x,y+1),(if x>0 {x-1} else {0}, y),(x, if y>0 {y-1} else {0})] {
                    if yy < height && xx < width {
                        let risk  = input[yy][xx];
                        if explored[yy][xx] > path_so_far+risk {
                            explored[yy][xx] = path_so_far+risk;
                            not_found = false;
                        }
                    }
                }
            }
        }
        if not_found {break;}
    }

    let ans = explored[explored.len()-1][explored[0].len()-1];
    // if !cfg!(test) {assert!(ans>2185);}  // 2186 too high BUT 2185 too low!! :O
    // if !cfg!(test) {assert!(ans<2186);}  // 2186 too high BUT 2185 too low!! :O
    ans
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
        assert_eq!(g.len(), 10);
        assert_eq!(g[0].len(), 10);
    }

    #[test]
    fn test_ex1_part1() {
        let g = gen1(EX1);
        let p1 = part1(&g);
        assert_eq!(p1, 40);
    }

    #[test]
    fn test_ex1_part2() {
        let g = gen1(EX1);
        let p1 = part2(&g);
        assert_eq!(p1, 315);
    }

const EX1: &'static str =
r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

const EX2: &'static str =
r"
";

}