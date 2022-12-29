use crate::day13::ListOrVal::*;
/// https://adventofcode.com/2022/day/13
/// DAN AoC: https://adventofcode.com/2022/leaderboard/private/view/380786
/// HLOTYAK: https://adventofcode.com/2022/leaderboard/private/view/951754

use std::{
    cmp::Ordering::{self, Equal, Less},
    str::FromStr,
};

#[derive(Clone, Debug, Eq, Ord)]
pub enum ListOrVal {
    Val(u8),
    List(Vec<Box<ListOrVal>>), // Embedded lists must be pointers, not actual lists, to avoid infinite size
}
impl PartialEq for ListOrVal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Val(lhs), Self::Val(rhs)) => lhs == rhs,
            (Self::Val(lhs), rhs) => ListOrVal::from(lhs) == *rhs,
            (lhs, Self::Val(rhs)) => *lhs == ListOrVal::from(rhs),
            (Self::List(lhs), Self::List(rhs)) => lhs == rhs,
        }
    }
}
impl PartialOrd for ListOrVal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Val(lhs), Val(rhs)) => lhs.partial_cmp(rhs),
            (Val(lhs), rhs) => ListOrVal::from(lhs).partial_cmp(rhs),
            (lhs, Val(rhs)) => lhs.partial_cmp(&ListOrVal::from(rhs)),
            (List(lhs), List(rhs)) => {
                let res = lhs.iter().zip(rhs).fold(Some(Equal), |acc, (lhs, rhs)| {
                    if acc == Some(Equal) {
                        lhs.partial_cmp(rhs)
                    } else {
                        acc
                    }
                });
                if res == Some(Equal) {
                    lhs.len().partial_cmp(&rhs.len())
                } else {
                    res
                }
            }
        }
    }
}
impl From<&u8> for ListOrVal {
    fn from(v: &u8) -> Self {
        List(vec![Box::new(Val(*v))])
    }
}
impl FromStr for ListOrVal {
    type Err = String;
    fn from_str(str_to_parse: &str) -> Result<Self, <Self as FromStr>::Err> {
        let chars: Vec<char> = str_to_parse.chars().collect();
        let len = chars.len();
        // expect '[' as first char and expect last to be matching ']'
        // The chars in the middle comprise the list of Vals or sub-lists.
        // if another '[' is found, find, then recurse on string between '[' and matching ']'
        if '[' == chars[0] {
            if ']' == chars[len - 1] {
                let mut nested_list = Vec::new();
                let mut ndx = 1;
                while ndx <= len - 2 {
                    match chars[ndx] {
                        '[' => {
                            // We're starting a new sublist, find ending postion, recurse to process, then push
                            let mut depth = 0u8;
                            let mut matched = false;
                            for endx in ndx + 1..len - 1 {
                                match (chars[endx], depth) {
                                    (']', 0) => {
                                        let sublist = ListOrVal::from_str(
                                            &chars[ndx..=endx].iter().collect::<String>(),
                                        )
                                        .unwrap();
                                        nested_list.push(Box::new(sublist));
                                        matched = true;
                                        ndx = endx;
                                    }
                                    (']', _) => depth -= 1,
                                    ('[', _) => depth += 1,
                                    _ => (),
                                }
                            }
                            if !matched {
                                return Err(format!(
                                    "Unmatched '[' in \"{}\" at position {}",
                                    str_to_parse, ndx
                                ));
                            }
                        }
                        n if n.is_numeric() => {
                            // We found a Val, parse it, push it
                            let mut num = n.to_string();
                            // Peek ahead to next char
                            while chars[ndx + 1].is_numeric() {
                                num.push(chars[ndx + 1]);
                                ndx += 1;
                            }
                            let val: u8 = num.parse().unwrap();
                            nested_list.push(Box::new(Val(val)));
                        }
                        ',' => (),
                        _ => {
                            return Err(format!(
                                "Unexpected char in \"{}\" at position {}",
                                str_to_parse, ndx
                            ))
                        }
                    }
                    ndx += 1;
                }
                Ok(List(nested_list))
            } else {
                Err(format!(
                    "Expected closing List char ']' at end of \"{}\"",
                    str_to_parse
                ))
            }
        } else {
            Err(format!(
                "Expected openning List char '[' at start of \"{}\"",
                str_to_parse
            ))
        }
    }
}
// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day13)]
pub fn gen1(input: &str) -> Vec<ListOrVal> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| ListOrVal::from_str(line))
        .flatten()
        .collect()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day13, part1)]
pub fn part1(input: &[ListOrVal]) -> usize {
    input
        .iter()
        .step_by(2)
        .zip(input.iter().skip(1).step_by(2))
        .enumerate()
        .filter(|(_, (list1, list2))| list1.partial_cmp(list2) == Some(Less))
        .map(|(ndx, _)| ndx + 1)
        .sum()
}
#[aoc(day13, part2)]
pub fn part2(input: &[ListOrVal]) -> usize {
    let mut input = input.to_vec();
    let divider1 = List(vec![Box::new(List(vec![Box::new(Val(2))]))]);
    let divider2 = List(vec![Box::new(List(vec![Box::new(Val(6))]))]);
    input.push(divider1.clone());
    input.push(divider2.clone());
    input.sort();
    input
        .iter()
        .enumerate()
        .filter(|(_, list)| list == &&divider1 || list == &&divider2)
        .map(|(ndx, _)| ndx + 1)
        .product()
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 13);
    }

    #[test]
    fn test_ex1_part2() {
        assert_eq!(part2(&gen1(EX1)), 140);
    }

    const EX1: &'static str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
}
