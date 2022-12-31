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

impl Monkey {
    fn swap(self, with_first: bool) -> Monkey {
        let Monkey { name, known, monkeys, math} = self;
        if let Some((m1,m2)) = monkeys {
            Monkey {
                name: if with_first {m1.clone()} else {m2.clone()},
                known,
                monkeys: Some(
                    if with_first {
                        (name, m2)
                    } else {
                        if math == Sub || math == Div {
                            (m1, name)
                        } else {
                            (name, m1)
                        }
                    }),
                math: match math {
                    Add => Sub,
                    Sub => if with_first {
                            Add
                        } else {
                            Sub
                        },
                    Mul => Div,
                    Div => if with_first {
                            Mul
                        } else {
                            Div
                        },
                Known => panic!("swap not supported for Known monkeys"),
                }
            }
        } else {
            panic!("Can't swap a Known monkey!");
        }
    }
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
pub fn part1(input: &[Monkey]) -> Option<u64> {
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
    
    known_monkeys["root"].known

}

#[aoc(day21, part2)]
pub fn part2(input: &[Monkey]) -> Option<u64> {
    let monkeys = input.to_vec();
    let mut known_monkeys = monkeys.iter().filter(|m|m.known.is_some())
        .map(|m| (m.name.to_string(), m.clone()))
        .collect::<HashMap<_, _>>();
    known_monkeys.remove("humn");
    let mut unknown_monkeys = monkeys.into_iter()
        .filter(|m| m.known.is_none()).collect::<Vec<_>>();
    let mut last_len = usize::MIN;
    while known_monkeys.len() > last_len {
        last_len = known_monkeys.len();
        for unk_monk in &mut unknown_monkeys {
            if let Some((m1, m2)) = &unk_monk.monkeys {
                if known_monkeys.contains_key(m1) && known_monkeys.contains_key(m2) {
                    if let Some(n1) = known_monkeys[m1].known {
                        if let Some(n2) = known_monkeys[m2].known {
                            let now_known = match &unk_monk.math {
                                Add => Some(n1 + n2),
                                Sub => {
                                    if n1 >= n2 {
                                        Some(n1 - n2)
                                    } else {
                                        Some(0)
                                    }
                                },
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

    // Root's math is a wierd == in Part 2.  This translates into subtracting equals...
    unknown_monkeys = unknown_monkeys.into_iter().map(|mut m| {
        if m.name == "root" {
            m.math = Sub;
        }
        m
    }).collect();

    // ...yields 0.
    known_monkeys.insert("root".to_string(),
        Monkey {
            name: "root".to_string(),
            known: Some(0),
            monkeys: None,
            math: Known,
        });

    // This is the heart of Part 2's solution -- rejiggering equations via .swap().
    unknown_monkeys = unknown_monkeys.into_iter().map(|m| {
        if let Some((m1,m2)) = &m.monkeys {
            match (known_monkeys.contains_key(m1), known_monkeys.contains_key(m2)) {
                (false, false) => m,
                (false, true) => m.swap(true),
                (true, false) => m.swap(false),
                (true, true) => panic!("Found a known monkey in the unknowns."),
            }
        } else {
            panic!("Can't happen.")
        }
    }).collect();

    // Finally, continue as in Part 1
    while !unknown_monkeys.is_empty() {
        for unk_monk in &mut unknown_monkeys {
            if let Some((m1, m2)) = &unk_monk.monkeys {
                if known_monkeys.contains_key(m1) && known_monkeys.contains_key(m2) {
                    if let Some(n1) = known_monkeys[m1].known {
                        if let Some(n2) = known_monkeys[m2].known {
                            let now_known = match &unk_monk.math {
                                Add => Some(n1 + n2),
                                Sub => {
                                    if n1 >= n2 {
                                        Some(n1 - n2)
                                    } else {
                                        Some(0)
                                    }
                                },
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

    let partially_known = unknown_monkeys.iter().filter(|m| {
        if let Some((m1,m2)) = &m.monkeys {
            known_monkeys.contains_key(m1) || known_monkeys.contains_key(m2)
        } else {
            false
        }
    }).count();
    println!("{} unknowns have {} that contain a known.", unknown_monkeys.len(), partially_known);

    known_monkeys["humn"].known

}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), Some(152));
    }

    #[test]
    fn test_ex1_part2() {
        assert_eq!(part2(&gen1(EX1)), Some(301));
    }

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
