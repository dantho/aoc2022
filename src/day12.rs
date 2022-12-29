/// https://adventofcode.com/2022/day/12
/// DAN AoC: https://adventofcode.com/2022/leaderboard/private/view/380786
/// HLOTYAK: https://adventofcode.com/2022/leaderboard/private/view/951754


pub struct Terrain {
    tmap: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}
// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day12)]
pub fn gen1(input: &str) -> Terrain {
    let mut tmap: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().filter(|c| c.is_alphabetic()).collect())
        .collect();
    let mut start = (usize::MAX, usize::MAX);
    let mut end = (usize::MAX, usize::MAX);
    for y in 0..tmap.len() {
        for x in 0..tmap[y].len() {
            match tmap[y][x] {
                'S' => {
                    start = (x, y);
                    tmap[y][x] = 'a';
                }
                'E' => {
                    end = (x, y);
                    tmap[y][x] = 'z';
                }
                _ => (),
            }
            assert!(tmap[y][x].is_ascii_lowercase())
        }
    }

    let tmap = tmap
        .iter()
        .map(|row| row.iter().map(|&c| c as u8 - 'a' as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Terrain { tmap, start, end }
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day12, part1)]
pub fn part1(input: &Terrain) -> u16 {
    let start = input.start;
    let end = input.end;
    let tmap = &input.tmap;
    let mut pathmap: Vec<Vec<Option<u16>>> =
        tmap.iter().map(|v| vec![None; v.len()]).collect::<Vec<_>>();
    pathmap[start.1][start.0] = Some(0);

    let mut hash = "".to_string();
    let mut prev = format!("{:?}", pathmap); // An inefficent hash!

    let mut iterations = 0;
    while hash != prev {
        prev = hash;
        search_step(tmap, &mut pathmap);
        hash = format!("{:?}", pathmap);
        iterations += 1;
        if iterations > pathmap.len() * pathmap[0].len() {
            panic!("Search iteration ({}) too high.", iterations);
        }
    }

    let steps_to_end = pathmap[end.1][end.0];
    #[cfg(test)]
    println!(
        "Min path of {:?} steps found in {} iterations.",
        steps_to_end, iterations
    );

    steps_to_end.unwrap()
}

#[aoc(day12, part2)]
pub fn part2(input: &Terrain) -> u16 {
    let start = input.start;
    let end = input.end;
    let tmap = &input.tmap;
    let mut pathmap: Vec<Vec<Option<u16>>> =
        tmap.iter().map(|v| vec![None; v.len()]).collect::<Vec<_>>();
    pathmap[start.1][start.0] = Some(0);

    // Part 2 difference, set all 0's in tmap to Some(0) in pathmap -- all are alternate starting points
    for y in 0..tmap.len() {
        for x in 0..tmap[0].len() {
            if tmap[y][x] == 0 {
                pathmap[y][x] = Some(0);
            }
        }
    }
    // End of Part 2 difference (all the rest of this fn is shameful cut-n-paste reuse!)

    let mut hash = "".to_string();
    let mut prev = format!("{:?}", pathmap); // An inefficent hash!

    let mut iterations = 0;
    while hash != prev {
        prev = hash;
        search_step(tmap, &mut pathmap);
        hash = format!("{:?}", pathmap);
        iterations += 1;
        if iterations > pathmap.len() * pathmap[0].len() {
            panic!("Search iteration ({}) too high.", iterations);
        }
    }

    let steps_to_end = pathmap[end.1][end.0];
    #[cfg(test)]
    println!(
        "Min path of {:?} steps found in {} iterations.",
        steps_to_end, iterations
    );

    steps_to_end.unwrap()
}

fn search_step(tmap: &[Vec<u8>], pathmap: &mut [Vec<Option<u16>>]) {
    let ymax = pathmap.len() - 1;
    let xmax = pathmap[0].len() - 1;
    for y in 0..=ymax {
        for x in 0..=xmax {
            let this_height = tmap[y][x];
            let adjacent_list = match (y > 0, x > 0, x < xmax, y < ymax) {
                (true, true, true, true) => vec![(y - 1, x), (y, x - 1), (y, x + 1), (y + 1, x)],
                (false, true, true, _) => vec![(y, x - 1), (y, x + 1), (y + 1, x)],
                (true, false, _, true) => vec![(y - 1, x), (y, x + 1), (y + 1, x)],
                (true, true, false, true) => vec![(y - 1, x), (y, x - 1), (y + 1, x)],
                (true, true, true, false) => vec![(y - 1, x), (y, x - 1), (y, x + 1)],
                (false, false, _, _) => vec![(y, x + 1), (y + 1, x)],
                (true, true, false, false) => vec![(y - 1, x), (y, x - 1)],
                (true, false, _, false) => vec![(y - 1, x), (y, x + 1)],
                (false, true, false, _) => vec![(y, x - 1), (y + 1, x)],
            };
            for (yy, xx) in adjacent_list {
                let this_path = pathmap[y][x].unwrap_or(u16::MAX); // pathmap can change on each iteration
                let adjacent_height = tmap[yy][xx];
                if let Some(adjacent_path) = pathmap[yy][xx] {
                    if this_height > adjacent_height + 1 {
                        continue;
                    }
                    if this_path > adjacent_path + 1 {
                        pathmap[y][x] = Some(adjacent_path + 1);
                    }
                }
            }
        }
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
        assert_eq!(part1(&gen1(EX1)), 31);
    }

    #[test]
    fn test_ex1_part2() {
        assert_eq!(part2(&gen1(EX1)), 29);
    }

    const EX1: &'static str = r"Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi";
}
