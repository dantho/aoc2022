/// https://adventofcode.com/2022/day/25
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754
///
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax
// extern crate regex;
// use self::regex::{Captures, Regex};

use std::fmt::Display;
use crate::day25::SnafuDigit::*;


// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day25)]
pub fn gen1(input: &str) -> Vec<Snafu> {
    input.lines().map(|line|Snafu::from(line)).collect()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day25, part1)]
pub fn part1(input: &[Snafu]) -> String {
    Snafu::from(input.iter().fold(0, |sum, sf| sum + i64::from(sf))).to_string()
}

#[derive(Debug, Clone)]
pub struct Snafu {
    snafs: Vec<SnafuDigit>,
}

impl Snafu {
    fn new() -> Self {
        Snafu { snafs: Vec::new() }
    }
}


impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut num = String::new();
        for dig in &self.snafs {
            num.push(dig.to_char());
        }
        write!(f,"{}",num)?;
        Ok(())
    }
}

impl From<&str> for Snafu {
    fn from(s: &str) -> Self {
        let mut num = Vec::new();
        for c in s.chars() {
            num.push(SnafuDigit::from(c));
        }
        Snafu { snafs: num }
    }
}

impl From<i64> for Snafu {
    fn from(mut n: i64) -> Snafu {
        let mut sf = Snafu::new();
        let mut carry = 0;
        while n > 0 || carry > 0 {
            let sdig = match (n + carry) % 5 {
                4 => {
                    carry = 1;
                    MinusOne
                },
                3 => {
                    carry = 1;
                    MinusTwo
                },
                2 => {
                    carry = 0;
                    Two
                },
                1 => {
                    carry = 0;
                    One
                },
                0 => {
                    // carry = 0;
                    Zero
                },
                _ => panic!("Can't happen."),
            };
            sf.snafs.push(sdig);
            n /= 5;
        }
        sf.snafs.reverse();
        sf
    }
}

impl From<&Snafu> for i64 {
    fn from(sf: &Snafu) -> Self {
        sf.snafs.iter().fold(0,|dec, sfdig| {
            dec * 5 + *sfdig as i64
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SnafuDigit {
    Two = 2,
    One = 1,
    Zero = 0,
    MinusOne = -1,
    MinusTwo = -2
}

impl SnafuDigit {
    fn to_char(&self) -> char {
        match self {
            Two => '2',
            One => '1',
            Zero => '0',
            MinusOne => '-',
            MinusTwo => '=',
        }
    }
}

impl From<char> for SnafuDigit {
    fn from(c: char) -> Self {
        match c {
            '2' => Two,
            '1' => One,
            '0' => Zero,
            '-' => MinusOne,
            '=' => MinusTwo,
            bad => panic!("Bad SnafuDigit char: '{}'", bad)
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
    fn test_snafudigit() {
                // Decimal     SnafuDigit
        assert_eq!(1        ,  i64::from(&Snafu::from("1")));
        assert_eq!(2        ,  i64::from(&Snafu::from("2")));
        assert_eq!(3        ,  i64::from(&Snafu::from("1=")));
        assert_eq!(4        ,  i64::from(&Snafu::from("1-")));
        assert_eq!(5        ,  i64::from(&Snafu::from("10")));
        assert_eq!(6        ,  i64::from(&Snafu::from("11")));
        assert_eq!(7        ,  i64::from(&Snafu::from("12")));
        assert_eq!(8        ,  i64::from(&Snafu::from("2=")));
        assert_eq!(9        ,  i64::from(&Snafu::from("2-")));
        assert_eq!(10       ,  i64::from(&Snafu::from("20")));
        assert_eq!(15       ,  i64::from(&Snafu::from("1=0")));
        assert_eq!(20       ,  i64::from(&Snafu::from("1-0")));
        assert_eq!(2022     ,  i64::from(&Snafu::from("1=11-2")));
        assert_eq!(12345    ,  i64::from(&Snafu::from("1-0---0")));
        assert_eq!(314159265,  i64::from(&Snafu::from("1121-1110-1=0")));
    }
    #[test]
    fn test_snafudigit2() {
        //                     Decimal                 SnafuDigit
        assert_eq!(Snafu::from(1        ).to_string(), "1");
        assert_eq!(Snafu::from(2        ).to_string(), "2");
        assert_eq!(Snafu::from(3        ).to_string(), "1=");
        assert_eq!(Snafu::from(4        ).to_string(), "1-");
        assert_eq!(Snafu::from(5        ).to_string(), "10");
        assert_eq!(Snafu::from(6        ).to_string(), "11");
        assert_eq!(Snafu::from(7        ).to_string(), "12");
        assert_eq!(Snafu::from(8        ).to_string(), "2=");
        assert_eq!(Snafu::from(9        ).to_string(), "2-");
        assert_eq!(Snafu::from(10       ).to_string(), "20");
        assert_eq!(Snafu::from(15       ).to_string(), "1=0");
        assert_eq!(Snafu::from(20       ).to_string(), "1-0");
        assert_eq!(Snafu::from(2022     ).to_string(), "1=11-2");
        assert_eq!(Snafu::from(12345    ).to_string(), "1-0---0");
        assert_eq!(Snafu::from(314159265).to_string(), "1121-1110-1=0");
    }
    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), "2=-1=0".to_string());
    }

    // #[test]
    // fn test_ex1_part2() {
    //     assert_eq!(part2(&gen1(EX1)), 45000);
    // }

    const EX1: &'static str = r"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
}
