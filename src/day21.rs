use std::collections::HashMap;

/// https://adventofcode.com/2022/day/21
/// DAN AoC: https://adventofcode.com/2022/leaderboard/private/view/380786
/// HLOTYAK: https://adventofcode.com/2022/leaderboard/private/view/951754
use crate::day21::Ops::*;

// ***********************
// *** Monkey Business ***
// ***********************
#[derive(Clone, Debug)]
pub struct Monkey {
    name: String,
    monkeys: Option<(String, String)>,
    known: Option<u64>,
    math: Ops,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ops {
    Add,
    Sub,
    Mul,
    Div,
    Known,
}

// ********************
// *** Generator(s) ***
// ********************
#[aoc_generator(day21)]
pub fn gen1(input: &str) -> Vec<Monkey> {
    input.lines().map(|line| {
        let mut pieces = line.split(' ');
        let name = pieces.next().unwrap()[0..4].to_string();
        let name_or_num = pieces.next().unwrap();
        if let Ok(num) = name_or_num.parse() {
            Monkey { name, known: Some(num), math: Known, monkeys: None }
        } else {
            let op = match pieces.next() {
                Some("+") => Add,
                Some("-") => Sub,
                Some("*") => Mul,
                Some("/") => Div,
                Some(bad) => panic!("Unknown math operator {}", bad),
                None => panic!("Parsing Error"),
            };
            let name2 = pieces.next().unwrap();
            Monkey { name, known: None, math: op, monkeys: Some((name_or_num.to_string(), name2.to_string()))}
        }
    }).collect()

}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day21, part1)]
pub fn part1(input: &[Monkey]) -> u64 {
    let monkeys = input.to_vec();
    let mut known_monkeys = monkeys.iter().filter(|m|m.known.is_some())
        .map(|m| (m.name.to_string(), m.clone()))
        .collect::<HashMap<_, _>>();
    let mut unknown_monkeys = monkeys.into_iter().filter(|m|m.known.is_none()).collect::<Vec<_>>();
    assert_eq!(input.len(), known_monkeys.len() + unknown_monkeys.len());

    while !unknown_monkeys.is_empty() {
        for unk_monk in &mut unknown_monkeys {
            if let Some((m1, m2)) = &unk_monk.monkeys {
                if known_monkeys.contains_key(m1) && known_monkeys.contains_key(m2) {
                    if let Some(n1) = known_monkeys[m1].known {
                        if let Some(n2) = known_monkeys[m2].known {
                            let now_known = match &unk_monk.math {
                                Add => Some(n1 + n2),
                                Sub => Some(n1 - n2),
                                Mul => Some(n1 * n2),
                                Div => Some(n1 / n2),
                                Known => panic!("Should not be known already"),
                            };
                            unk_monk.known = now_known;
                            unk_monk.monkeys = None;
                        }
                    }
                }
            } else {
                panic!("Unknown monkeys MUST have .monkeys defined");
            }
        }
        unknown_monkeys = unknown_monkeys.into_iter().filter_map(|m| {
            if m.known.is_some() {
                known_monkeys.insert(m.name.to_string(), m);
                None
            } else {
                Some(m)
            }
        }).collect();
    }
    
    known_monkeys["root"].known.unwrap()

}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 152);
    }

    // #[test]
    // fn test_ex1_part2() {
    //     assert_eq!(part2(&true), 2713310158);
    // }

    #[allow(unused)]
    const EX1: &'static str = r"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
}
