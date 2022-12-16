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
    coverage_map
        .iter()
        .fold((0, isize::MIN), |(cnt_in_range, lastx), (xxmin, xxmax)| {
            let new_in_range = match (lastx >= *xxmax, lastx >= *xxmin) {
                (false, false) => *xxmax - *xxmin + 1,
                (false, true) => *xxmax - lastx,
                (true, false) => panic!("Sorting error"),
                (true, true) => 0,
            };
            (cnt_in_range + new_in_range, *xxmax.max(&lastx))
        })
        .0
        - beacons_on_target_row
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
    let distress_beacons = (target_rows.0..=target_rows.1)
        .map(|target_row| {
            let mut coverage_map = input
                .iter()
                .filter_map(|(sensor, _beacon, range)| {
                    if sensor.0 + range >= 0 || sensor.0 - range <= 4_000_000 {
                        Some((sensor.0.max(0).min(4_000_000), sensor.1))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
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
            coverage_map
                .iter()
                .fold((0, isize::MIN), |(cnt_in_range, lastx), (xxmin, xxmax)| {
                    let new_in_range = match (lastx >= *xxmax, lastx >= *xxmin) {
                        (false, false) => *xxmax - *xxmin + 1,
                        (false, true) => *xxmax - lastx,
                        (true, false) => panic!("Sorting error"),
                        (true, true) => 0,
                    };
                    (cnt_in_range + new_in_range, *xxmax.max(&lastx))
                })
        })
        .collect::<Vec<_>>();
    assert!(distress_beacons.len() == 1);
    let (x, y) = distress_beacons.into_iter().nth(0).unwrap();
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

    // #[test]
    // fn test_ex1_part2() {
    //     assert_eq!(part2(&gen1(EX1)), 45000);
    // }

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
