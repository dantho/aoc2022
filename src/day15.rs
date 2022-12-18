/// https://adventofcode.com/2022/day/15
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754
///
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax
// extern crate regex;
// use self::regex::{Captures, Regex};
use std::{collections::HashSet, iter::once};

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day15)]
pub fn gen1(input: &str) -> Vec<((isize, isize), (isize, isize))> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .chain(once('z'))
                .fold(
                    (false, Vec::<String>::new()),
                    |(was_numeric, mut nums), c| {
                        let is_numeric_or_minus = c.is_numeric() || c == '-';
                        if is_numeric_or_minus {
                            if was_numeric {
                                let mut msdigits = nums.pop().unwrap();
                                msdigits.push(c);
                                nums.push(msdigits);
                            } else {
                                nums.push(c.to_string());
                            }
                        }
                        (is_numeric_or_minus, nums)
                    },
                )
                .1
                .iter()
                .map(|s| s.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| ((v[0], v[1]), (v[2], v[3])))
        .collect::<Vec<_>>()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day15, part1)]
pub fn part1(input: &[((isize, isize), (isize, isize))]) -> isize {
    let input = input
        .iter()
        .map(|(sensor, beacon)| (sensor, beacon, manhattan(sensor, beacon)))
        .collect::<Vec<_>>();

    #[cfg(test)]
    let target_row = 10;
    #[cfg(not(test))]
    let target_row = 2_000_000;

    let mut coverage_map = input
        .iter()
        .filter_map(|(sensor, _beacon, range)| {
            let horizontal_range = range - (sensor.1 - target_row).abs();
            if horizontal_range >= 0 {
                Some((sensor.0 - horizontal_range, sensor.0 + horizontal_range))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    coverage_map.sort();
    #[cfg(test)]
    println!("{:?}", coverage_map);
    let beacons_on_target_row = input
        .iter()
        .filter_map(|(_, beacon, _)| {
            if beacon.1 == target_row {
                Some(beacon)
            } else {
                None
            }
        })
        .collect::<HashSet<_>>()
        .len() as isize;
        covered_count(&coverage_map, None) - beacons_on_target_row
}

fn covered_count(coverage_map: &Vec<(isize,isize)>, range: Option<(isize,isize)>) -> isize {
    let range = range.unwrap_or((isize::MIN+1, isize::MAX-1));
    coverage_map
        .iter()
        .fold((0, range.0-1), |(cnt_in_range, lastx), (xxmin, xxmax)| {
            let new_in_range = match (lastx >= *xxmax, lastx >= *xxmin) {
                (false, false) => *xxmax - *xxmin + 1,
                (false, true) => *xxmax - lastx,
                (true, false) => panic!("Sorting error"),
                (true, true) => 0,
            };
            (cnt_in_range + new_in_range, *xxmax.max(&lastx))
        })
        .0
}

#[aoc(day15, part2)]
pub fn part2(input: &[((isize, isize), (isize, isize))]) -> isize {
    let input = input
        .iter()
        .map(|(sensor, beacon)| (sensor, beacon, manhattan(sensor, beacon)))
        .collect::<Vec<_>>();

    #[cfg(test)]
    let target_rows = (0, 20);
    #[cfg(not(test))]
    let target_rows = (0, 4_000_000);
    // let input = input
    //     .iter()
    //     .filter(|(sensor, _, range)| {
    //         sensor.0 + range >= target_rows.0 || sensor.0 - range <= target_rows.1 
    //     })
    //     .collect::<Vec<_>>();
    let distress_beacon_row = (target_rows.0..=target_rows.1)
        .fold(None,|prev_found, target_row| {
            let mut coverage_map = input
                .iter()
                .filter_map(|(sensor, _beacon, range)| {
                    let horizontal_range = range - (sensor.1 - target_row).abs();
                    if horizontal_range < 0 {
                        None
                    } else {
                        let leftmost = sensor.0 - horizontal_range;
                        let rightmost = sensor.0 + horizontal_range;
                        if leftmost > target_rows.1 || rightmost < target_rows.0 {
                            None
                        } else {
                            Some((leftmost.max(target_rows.0), rightmost.min(target_rows.1)))
                        }
                    }
                })
                .collect::<Vec<_>>();
            coverage_map.sort();
            let covered_cnt = covered_count(&coverage_map, Some(target_rows));
            match (target_rows.1-target_rows.0 + 1) - covered_cnt {
                0 => prev_found,
                1 => {
                    if let Some(prev_row) = prev_found {
                        panic!("Found 2nd row ({}) with coverage gap.  First was {}.", target_row, prev_row);
                    }
                    Some(target_row)
                },
                n => panic!("Error: {} locations left uncovered in row {}!", n, target_row)
            }
        });
    let (x,y) = if let Some(found_y) = distress_beacon_row {
        let found_x = {
            let mut coverage_map = input
                .iter()
                .filter_map(|(sensor, _beacon, range)| {
                    let horizontal_range = range - (sensor.1 - found_y).abs();
                    if horizontal_range < 0 {
                        None
                    } else {
                        let leftmost = sensor.0 - horizontal_range;
                        let rightmost = sensor.0 + horizontal_range;
                        if leftmost > target_rows.1 || rightmost < target_rows.0 {
                            None
                        } else {
                            Some((leftmost.max(target_rows.0), rightmost.min(target_rows.1)))
                        }
                    }
                })
                .collect::<Vec<_>>();            
            coverage_map.sort();
            coverage_map
                .iter()
                .fold((None, -1), |(foundx, lastx), (xxmin, xxmax)| {
                    let found_gap = match (lastx >= *xxmax, lastx >= *xxmin-1) {
                        (false, false) => {
                            if let Some(prev_found_x) = foundx {
                                panic!("Found gap starting at {} after previous gap found at {} in row {}", lastx+1, prev_found_x, found_y)
                            }
                            if *xxmin-lastx == 2 {
                                Some(*xxmin-1)
                            } else {
                                dbg!(foundx);
                                panic!("Too many uncovered locations found ({}) between x = {} and x = {} in row {}", *xxmin-lastx-1, lastx, *xxmin, found_y)
                            }
                        },
                        (false, true) => foundx,
                        (true, false) => panic!("Sorting error"),
                        (true, true) => foundx,
                    };
                    (found_gap, *xxmax)
                })
                .0.unwrap()   
        };
        (found_x, found_y)      
    } else {
        panic!("No row found with uncovered location!");
    };
    assert!(x != 0);
    let tuning_frequency = x * 4_000_000 + y;
    tuning_frequency
}

fn manhattan(a: &(isize, isize), b: &(isize, isize)) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 26);
    }

    #[test]
    fn test_ex1_part2() {
        assert_eq!(part2(&gen1(EX1)), 56000011);
    }

    const EX1: &'static str = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
}
