/// https://adventofcode.com/2022/day/13
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754
///
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax
// extern crate regex;
// use self::regex::{Captures, Regex};

use std::str::FromStr;
use crate::day13::ListOrVal::*;

#[derive(Debug)]
pub enum ListOrVal {
    Val(u8),
    List(Vec<Box<ListOrVal>>) // Embedded lists must be pointers, not actual lists, to avoid infinite size    
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
            if ']' == chars[len-1] {
                let mut nested_list = Vec::new();
                let mut ndx = 1;
                while ndx <= len-2 {
                    match chars[ndx] {
                        '[' => {
                            // We're starting a new sublist, find ending postion, recurse to process, then push
                            let mut depth = 0u8;
                            let mut matched = false;
                            for endx in ndx+1..len-1 {
                                match (chars[endx], depth) {
                                    (']', 0) => {
                                        let sublist = ListOrVal::from_str(&chars[ndx..=endx].iter().collect::<String>()).unwrap();
                                        nested_list.push(Box::new(sublist));
                                        matched = true;
                                        ndx = endx;
                                    },
                                    (']', _) => depth -= 1,
                                    ('[', _) => depth += 1,
                                    _ => (),
                                }
                            }
                            if !matched {
                                return Err(format!("Unmatched '[' in \"{}\" at position {}", str_to_parse, ndx));
                            }
                        },
                        n if n.is_numeric() => { // We found a Val, parse it, push it
                            let mut num = n.to_string();
                            // Peek ahead to next char
                            while chars[ndx+1].is_numeric() {
                                num.push(chars[ndx+1]);
                                ndx += 1;
                            }
                            let val: u8 = num.parse().unwrap();
                            nested_list.push(Box::new(Val(val)));
                        },
                        ',' => (),
                        _ => return Err(format!("Unexpected char in \"{}\" at position {}", str_to_parse, ndx))
                    }
                    ndx += 1;
                }
                Ok(List(nested_list))
            } else {
                Err(format!("Expected closing List char ']' at end of \"{}\"", str_to_parse))
            }
        } else {
            Err(format!("Expected openning List char '[' at start of \"{}\"", str_to_parse))
        }
    }
}
// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day13)]
pub fn gen1(input: &str) -> Vec<ListOrVal> {
    input.lines()
        .filter(|line|!line.trim().is_empty())
        .map(|line|ListOrVal::from_str(line))
        .flatten()
        .collect()    
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day13, part1)]
pub fn part1(input: &[ListOrVal]) -> u32 {
    dbg!(input);
    0
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 999);
    }

    // #[test]
    // fn test_ex1_part2() {
    //     assert_eq!(part2(&gen1(EX1)), 45000);
    // }

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
