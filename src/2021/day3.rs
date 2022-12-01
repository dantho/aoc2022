use std::{collections::binary_heap, panic};

/// https://adventofcode.com/2021/day/3
/// ADI: https://adventofcode.com/2021/leaderboard/private/view/380786 
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 
/// 
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax 
extern crate regex;
// use self::regex::{Captures, Regex};

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day3)]
pub fn gen1(input: &[u8]) -> Vec<Vec<u32>> {
    let input: Vec<u32> = input.iter()
        .map(|num| match num {
            48 => 0u32,
            49 => 1,
            10 => 10,
            _ => panic!("Unrecognized input"),
        }).collect();
    let input: Vec<Vec<u32>> = input.split(|&num| num == 10)
        .map(|s|s.to_vec())
        .collect();
    input
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day3, part1)]
pub fn part1(input: &[Vec<u32>]) -> u32 {
    let len = input.len() as u32;
    let sum_of_1s = input.iter()
        .fold(vec![0;input[0].len()],|sum, v|sum.iter().zip(v.iter()).map(|(s,v)|s+v).collect());
    let gamma = sum_of_1s.iter().fold(0,|word,n|word*2+n*2/len);
    let bitmask = (1<<input[0].len())-1;
    let epsilon = (!gamma)&bitmask;
    println!("Input length: {}", len);
    println!("Sum of 1's: {:?}", sum_of_1s);
    println!("Gamma rate: {}", gamma);
    println!("Epsilon rate: {}", epsilon);
    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn part2(input: &[Vec<u32>]) -> u32 {
    let len = input.len() as u32;
    let width = input[0].len() as u32;
    let binary_input: Vec<u32> = input.iter()
        .map(|v|v.iter().fold(0,|bin,bit|bin*2+bit)).collect();

    let mut diagdata = binary_input.clone();
    if cfg!(test) {println!("remaining vals: {:?}", diagdata)};
    for bit in 0..width {
        let bitwgt = 1<<(width-bit-1); // msb first
        let bitcrit = diagdata.iter().filter(|&n|n&bitwgt>0).count()*2/diagdata.len();
        let bitcrit = match bitcrit {
            1 => bitwgt,
            0 => 0,
            _ => panic!("Expected bitcrit value of 0 or 1, got {}", bitcrit),
        };
        diagdata = diagdata.into_iter().filter(|&n| n&bitwgt==bitcrit).collect();
        if cfg!(test) {println!("remaining vals: {:?}", diagdata)};
        if diagdata.len() == 1 {break};
    };
    let oxy_gen = if diagdata.len() == 1 {
        diagdata[0]
    } else {
        panic!("Expected diagdata.len() == 1, got {}", diagdata.len())
    };

    let mut diagdata = binary_input;
    if cfg!(test) {println!("remaining vals: {:?}", diagdata)};
    for bit in 0..width {
        let bitwgt = 1<<(width-bit-1); // msb first
        let bitcrit = diagdata.iter().filter(|&n|n&bitwgt>0).count()*2/diagdata.len();
        let bitcrit = match bitcrit {
            0 => bitwgt,
            1 => 0,
            _ => panic!("Expected bitcrit value of 0 or 1, got {}", bitcrit),
        };
        diagdata = diagdata.into_iter().filter(|&n| n&bitwgt==bitcrit).collect();
        let co2scrub = if diagdata.len() == 1 {
            Some(diagdata[0])
        } else {
            None
        };
        if cfg!(test) {println!("remaining vals: {:?}", diagdata)};
        if diagdata.len() == 1 {break};
    };
    let co2scrub = if diagdata.len() == 1 {
        diagdata[0]
    } else {
        panic!("Expected diagdata.len() == 1, got {}", diagdata.len())
    };

    println!("Oxygen generator rating: {}", oxy_gen);
    println!("CO2 scrubber rating: {}", co2scrub);

    let ans = oxy_gen * co2scrub;
    if !cfg!(test) 
    {
        assert!(ans < 14405818, "Answer {} is too high", ans);
        assert!(ans > 1166600, "Answer {} is too low", ans);    
    }
    ans
}

fn inv_bits(v:u32,bitwidth:u32) -> u32 {
    let bitmask = (1<<bitwidth)-1;
    (!v)&bitmask
}
// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(888, 999);
    }

    #[test]
    fn test_ex2_part2() {
        let input = vec![
            vec![0,0,1,0,0],
            vec![1,1,1,1,0],
            vec![1,0,1,1,0],
            vec![1,0,1,1,1],
            vec![1,0,1,0,1],
            vec![0,1,1,1,1],
            vec![0,0,1,1,1],
            vec![1,1,1,0,0],
            vec![1,0,0,0,0],
            vec![1,1,0,0,1],
            vec![0,0,0,1,0],
            vec![0,1,0,1,0],
        ];
        let ans = part2(&input);
        assert_eq!(ans, 230);
    }

const EX1: &'static str =
r"
";

const EX2: &'static str =
r"
";

}