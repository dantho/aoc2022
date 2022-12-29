/// https://adventofcode.com/2022/day/15
/// DAN AoC: https://adventofcode.com/2022/leaderboard/private/view/380786
/// HLOTYAK: https://adventofcode.com/2022/leaderboard/private/view/951754

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

fn coverage_gap(coverage_map: &Vec<(isize,isize)>, range: Option<(isize,isize)>) -> Option<isize> {
    let range = range.unwrap_or((isize::MIN+1, isize::MAX-1));
    coverage_map
        .iter()
        .fold((None, range.0-1), |(gapx, lastx), (xxmin, xxmax)| {
            let found_gap = match xxmin-lastx {
                2 => {
                    if let Some(prior) = gapx {
                        panic!("Prior gap detected at {}", prior)
                    } else {
                        Some(lastx+1)
                    }
                },
                n if n > 2 => panic!("Expecting coverage gap of 1, found gap of {}!", n),
                _ => gapx,
            };
            (found_gap, *xxmax.max(&lastx))
        }).0
}

#[aoc(day15, part2)]
pub fn part2(input: &[((isize, isize), (isize, isize))]) -> isize {
    // Add distance from sensor to beacon for each
    let input = input
        .iter()
        .map(|(sensor, beacon)| (sensor, beacon, manhattan(sensor, beacon)))
        .collect::<Vec<_>>();

    #[cfg(test)]
    let target_range = (0, 20);
    #[cfg(not(test))]
    let target_range = (0, 4_000_000);
    let distress_beacon_pos = (target_range.0..=target_range.1)
        .fold(None,|prev_found, row_to_search| {
            let mut coverage_map = input
                .iter()
                .filter_map(|(sensor, _beacon, range)| {
                    // find segment within range of this beacon and centered on beacon's x
                    let horizontal_range = range - (sensor.1 - row_to_search).abs();
                    if horizontal_range < 0 {
                        None
                    } else {
                        let leftmost = sensor.0 - horizontal_range;
                        let rightmost = sensor.0 + horizontal_range;
                        if leftmost > target_range.1 || rightmost < target_range.0 {
                            None
                        } else {
                            Some((leftmost.max(target_range.0), rightmost.min(target_range.1)))
                        }
                    }
                })
                .collect::<Vec<_>>();
            // sort required for covered_count() and coverage_gap() algos
            coverage_map.sort();
            if let Some(found_x) = coverage_gap(&coverage_map, Some(target_range)) {
                if let Some(other) = prev_found {
                    #[cfg(test)]
                    if row_to_search == 11 {println!("Row {}:\n{:?}", row_to_search, coverage_map);}
                    panic!("Found gap at {:?} but previously found another at {:?}", (found_x,row_to_search), other);
                }
                #[cfg(test)]
                if row_to_search == 10 {println!("Row {}:\n{:?}", row_to_search, coverage_map);}
                Some((found_x,row_to_search))
            } else {
                prev_found
            }
        });
    let (x,y) = distress_beacon_pos.unwrap();
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
