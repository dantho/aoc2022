/// https://adventofcode.com/2022/day/15
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754
///
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax
// extern crate regex;
// use self::regex::{Captures, Regex};

use std::iter::once;

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day15)]
pub fn gen1(input: &str) -> Vec<((isize,isize),(isize,isize))> {
    input.lines()
        .map(|line| {
            line.chars().chain(once('z')).fold((false, Vec::<String>::new()),|(was_numeric, mut nums), c| {
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
            }).1.iter().map(|s|s.parse::<isize>().unwrap()).collect::<Vec<_>>()
        })
        .map(|v|((v[0],v[1]),(v[2],v[3])))
        .collect::<Vec<_>>()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day15, part1)]
pub fn part1(input: &[((isize,isize),(isize,isize))]) -> usize {
    let input = input.iter()
        .map(|(sensor,beacon)| (sensor, beacon, manhattan(sensor, beacon)))
        .collect::<Vec<_>>();
    let (xmin, xmax, _ymin, _ymax) = input.iter()
        .fold((isize::MAX,isize::MIN,isize::MAX,isize::MIN),|(xmin,xmax,ymin,ymax),(sensor, _beacon, range)| {
            (xmin.min(sensor.0-range),
            xmax.max(sensor.0+range),
            ymin.min(sensor.1-range),
            ymax.max(sensor.1+range))
        });
        dbg!(xmin);
        dbg!(xmax);
        dbg!(_ymin);
        dbg!(_ymax);
        
        #[cfg(test)]
        let target_row = 10;
        #[cfg(not(test))]
        let target_row = 2_000_000;
    (xmin..=xmax).map(|x| {
        let mut in_range = false;
        for (sensor, beacon, range) in &input {
            let test_pos = (x, target_row);
            if beacon == &&test_pos || manhattan(&test_pos, sensor) <= *range {
                in_range = true;
                break;
            }
        }
        in_range
    }).filter(|b|*b).count()
}

fn manhattan(a: &(isize, isize), b: &(isize, isize)) -> isize {
    (a.0-b.0).abs()+(a.1-b.1).abs()
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 26+973);
    }

    // #[test]
    // fn test_ex1_part2() {
    //     assert_eq!(part2(&gen1(EX1)), 45000);
    // }

    const EX1: &'static str = 
r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
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
