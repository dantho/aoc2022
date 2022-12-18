/// https://adventofcode.com/2022/day/11
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754
///
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax
// extern crate regex;
// use self::regex::{Captures, Regex};

// ***********************
// *** Monkey Business ***
// ***********************
pub struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    divisor: u64,
    targets: (usize, usize),
}

impl Monkey {
    fn new(
        items: Vec<u64>,
        operation: Box<dyn Fn(u64) -> u64>,
        divisor: u64,
        targets: (usize, usize),
    ) -> Self {
        Self {
            items,
            operation,
            divisor,
            targets,
        }
    }
}

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day11)]
pub fn gen1(_input: &str) -> bool {
    false
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day11, part1)]
pub fn part1(use_example: &bool) -> u64 {
    let monkeys = get_monkeys(*use_example);
    monkey_business(monkeys, true)
}

#[aoc(day11, part2)]
pub fn part2(use_example: &bool) -> u64 {
    let monkeys = get_monkeys(*use_example);
    monkey_business(monkeys, false)
}

fn get_monkeys(use_example: bool) -> Vec<Monkey> {
    if use_example {
        vec![
            Monkey::new(vec![79, 98], Box::new(|worry| worry * 19), 23, (2, 3)),
            Monkey::new(
                vec![54, 65, 75, 74],
                Box::new(|worry| worry + 6),
                19,
                (2, 0),
            ),
            Monkey::new(
                vec![79, 60, 97],
                Box::new(|worry| worry * worry),
                13,
                (1, 3),
            ),
            Monkey::new(vec![74], Box::new(|worry| worry + 3), 17, (0, 1)),
        ]
    } else {
        vec![
            Monkey::new(vec![93, 98], Box::new(|worry| worry * 17), 19, (5, 3)),
            Monkey::new(
                vec![95, 72, 98, 82, 86],
                Box::new(|worry| worry + 5),
                13,
                (7, 6),
            ),
            Monkey::new(
                vec![85, 62, 82, 86, 70, 65, 83, 76],
                Box::new(|worry| worry + 8),
                5,
                (3, 0),
            ),
            Monkey::new(vec![86, 70, 71, 56], Box::new(|worry| worry + 1), 7, (4, 5)),
            Monkey::new(
                vec![77, 71, 86, 52, 81, 67],
                Box::new(|worry| worry + 4),
                17,
                (1, 6),
            ),
            Monkey::new(
                vec![89, 87, 60, 78, 54, 77, 98],
                Box::new(|worry| worry * 7),
                2,
                (1, 4),
            ),
            Monkey::new(vec![69, 65, 63], Box::new(|worry| worry + 6), 3, (7, 2)),
            Monkey::new(vec![89], Box::new(|worry| worry * worry), 11, (0, 2)),
        ]
    }
}

fn monkey_business(mut monkeys: Vec<Monkey>, is_part1: bool) -> u64 {
    let multiple_of_all_divisors = monkeys.iter().fold(1, |acc, monkey| acc * monkey.divisor);
    let mut inspect_count = vec![0; monkeys.len()];
    for _round in 0..if is_part1 { 20 } else { 10_000 } {
        for m in 0..monkeys.len() {
            // println!("Monkey {} at start of round {}: {:?}", m, round, monkeys[m].items);
            while !monkeys[m].items.is_empty() {
                inspect_count[m] += 1;
                monkeys[m].items.reverse();
                let item = monkeys[m].items.pop().unwrap();
                monkeys[m].items.reverse();
                let item = (monkeys[m].operation)(item);
                let item = if is_part1 {
                    item / 3
                } else {
                    item % multiple_of_all_divisors
                };
                let target = if item % monkeys[m].divisor == 0 {
                    monkeys[m].targets.0
                } else {
                    monkeys[m].targets.1
                };
                monkeys[target].items.push(item);
            }
        }
        // println!("End of Round {}:", round);
        // for m in 0..monkeys.len() {
        //     println!("Monkey {}: {:?}", m, monkeys[m].items);
        // }
    }
    // println!("{:?}", inspect_count);
    inspect_count.sort();
    inspect_count.reverse();
    inspect_count[0] * inspect_count[1]
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&true), 10605);
    }

    #[test]
    fn test_ex1_part2() {
        assert_eq!(part2(&true), 2713310158);
    }

    #[allow(unused)]
    const EX1: &'static str = r"Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
}
