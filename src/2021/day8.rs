/// https://adventofcode.com/2021/day/8
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 
/// 
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax 
// extern crate regex;
// use self::regex::{Captures, Regex};

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day8)]
pub fn gen1(input: &str) -> Vec<Vec<Vec<String>>> {
    input.lines()
    .map(|ln|ln.split(" | ")
        .map(|half|half.split(" ")
            .map(|s|s.to_string())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>())
    .collect::<Vec<_>>()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day8, part1)]
pub fn part1(input: &[Vec<Vec<String>>]) -> usize {
// &[Vec]: scrambled signal input and segment output for each of multiple 4-digit 7-segment displays
// Vec: input, output (signals and segments, respectively)
// Vec: digits (set of signals or segments, max of 7 representing the "8" with all segments lit)
// String: chars are signals if input, segments if output
    let mut sig_count = [0usize;8];
    for disp in input {
        let output = &disp[1];
        for digit in output {
            sig_count[digit.len()]+=1;
        }
    }
    sig_count[2]+
    sig_count[4]+
    sig_count[3]+
    sig_count[7]
}

#[aoc(day8, part2)]
pub fn part2(input: &[Vec<Vec<String>>]) -> usize {
    input.iter().map(|disp|decode(&disp[0],&disp[1])).sum()
}

fn decode(input: &[String], output: &[String]) -> usize {
    // Sort by size
    let mut size2 = "";
    let mut size3 = "";
    let mut size4 = "";
    let mut size6: Vec<String> = Vec::new(); // Expecting 3
    let mut size7 = "";
    for digit in input {
        match digit.len() {
            2 => size2 = digit,
            3 => size3 = digit,
            4 => size4 = digit,
            5 => (),
            6 => size6.push(digit.to_string()),
            7 => size7 = digit,
            _ => panic!("Unexpected input length"),
        }
    }
    // Rules!
    let cf = size2.to_string();
    let a = subtract(size3, &cf);
    let bd = subtract(size4, &cf);
    let ecd = subtract(size7, &size6[0])+&subtract(size7, &size6[1])+&subtract(size7, &size6[2]);
    let e = subtract(&subtract(&ecd, &cf), &bd);
    let d = subtract(&subtract(&ecd, &cf), &e);
    let b = subtract(&bd, &d);
    let c = subtract(&subtract(&ecd, &e), &d);
    let f = subtract(&cf, &c);
    let g = subtract(&subtract(&subtract(&subtract(size7, &ecd), &a), &b), &f);
    // Decoding
    let mut four_digit_value = 0;
    for digit in output {
        four_digit_value *= 10;
        four_digit_value +=
        match (digit.contains(&a),digit.contains(&b),digit.contains(&c),digit.contains(&d),digit.contains(&e),digit.contains(&f),digit.contains(&g)) {
            (true,true,true,false,true,true,true) => 0,
            (false,false,true,false,false,true,false) => 1,
            (true,false,true,true,true,false,true) => 2,
            (true,false,true,true,false,true,true) => 3,
            (false,true,true,true,false,true,false) => 4,
            (true,true,false,true,false,true,true) => 5,
            (true,true,false,true,true,true,true) => 6,
            (true,false,true,false,false,true,false) => 7,
            (true,true,true,true,true,true,true) => 8,
            (true,true,true,true,false,true,true) => 9,
            _ => 9999,
        };
    }

    four_digit_value
}

fn intersection(s1: &str, s2: &str) -> String {
    s1.chars().filter(|c|s2.contains(&c.to_string())).collect()
}
fn remainder(s1: &str, s2: &str) -> String {
    subtract(s1, s2)+&subtract(s2, s1)
}
fn subtract(s: &str, subtrahend: &str) -> String {
    s.chars().filter(|c|!subtrahend.contains(&c.to_string())).collect()
}
// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen1_ex1() {
        let g = gen1(EX1);
        assert_eq!(g.len(), 1);
        assert_eq!(g[0].len(), 2);
        assert_eq!(g[0][0].len(), 10);
        assert_eq!(g[0][0][0].len(), 7);
    }

    #[test]
    fn test_gen1_ex2() {
        let g = gen1(EX2);
        assert_eq!(g.len(), 10); // count of displays
        assert_eq!(g[0].len(), 2); // input, output for first display
        assert_eq!(g[0][0].len(), 10); // all digits in input
        assert_eq!(g[0][0][0].len(), 2); // chars in first input
    }

    #[test]
    fn test_part1_ex1() {
        let g = gen1(EX1);
        let p1 = part1(&g);
        assert_eq!(p1, 0);
    }

    #[test]
    fn test_part1_ex2() {
        let g = gen1(EX2);
        let p1 = part1(&g);
        assert_eq!(p1, 26);
    }

    #[test]
    fn test_part2_ex2() {
        let g = gen1(EX2);
        let p1 = part2(&g);
        assert_eq!(p1, 61229);
    }

const EX1: &'static str =
r"acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

const EX2: &'static str =
r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

const SEVEN_SEGMENT: &'static str =
r"
  0:      1:      2:      3:      4:
 aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
 ....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
 gggg    ....    gggg    gggg    ....

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
 dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
 gggg    gggg    ....    gggg    gggg
";
}