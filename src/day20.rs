/// https://adventofcode.com/2022/day/20
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754
///
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax
// extern crate regex;
// use self::regex::{Captures, Regex};

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day20)]
pub fn gen1(input: &str) -> Vec<isize> {
    input.lines().map(|numstr|numstr.parse().unwrap()).collect()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day20, part1)]
pub fn part1(input: &[isize]) -> isize {
    #[cfg(test)]
    println!("Orig values:   {:?}", input);
    let len = input.len();
    // Instead of mixing values, we'll mix ptrs (original index) to those values
    let mut input_indices: Vec<usize> = (0..len).collect();
    // The above will be scrambled, so we need a descrambler
    // This answers the question: If I'm looking for the scrambled position of what was originally in position i, where would I find that?
    let mut input_index_lookup = input_indices.clone();
    // Algo:  mix value indices based on value (input)
    for i in 0..input.len() {
        // Debug
        #[cfg(test)]
        {
            println!("Input Indices: {:?}", input_indices);
            println!("Index lookup:  {:?}", input_index_lookup);
            println!("Moving '{}'", input[i]);
        }
        let value = input[i];
        let before = input_index_lookup[i];
        let after = match signed_mod(before as isize + value, len) {
            n if n <= before => n,
            n if n > before => n-1, // account for missing value (the moved value)
            _ => panic!("Not logically possible.")
        };
        // mix! (move an element in input_indices from position before to position after)
        let ndx_to_move = input_indices[before];
        let other_indices = [&input_indices[..before], &input_indices[before + 1..]].concat();
        input_indices = [&other_indices[..after], &[ndx_to_move], &other_indices[after..]].concat();
        // Now rebuild the index lookup table
        let mut tmp: Vec<(usize,usize)> = input_indices.iter().enumerate().map(|(i,ndx)| (*ndx,i)).collect();
        tmp.sort();
        input_index_lookup = tmp.iter().map(|(_,ndx)|*ndx).collect();
        // Debug
        #[cfg(test)]
        {
            let ddd: Vec<isize> = input_indices.iter().map(|ndx|input[*ndx]).collect();
            println!("New values:    {:?}", ddd);
        }
    }
    input[input_indices[1000 % len]]+input[input_indices[2000 % len]]+input[input_indices[3000 % len]]
}

fn signed_mod(v: isize, modulo: usize) -> usize {
    let modulo = modulo as isize;
    ((v % modulo + modulo) % modulo) as usize
}

// #[aoc(day20, part2)]
// pub fn part2(input: &Vec<Vec<u32>>) -> u32 {
// }

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signed_mod() {
        assert_eq!(signed_mod(-13, 7), 0);
        assert_eq!(signed_mod(-8, 7), 6);
        assert_eq!(signed_mod(-7, 7), 0);
        assert_eq!(signed_mod(-1, 7), 6);
        assert_eq!(signed_mod(0, 7), 0);
        assert_eq!(signed_mod(6, 7), 6);
        assert_eq!(signed_mod(7, 7), 0);
        assert_eq!(signed_mod(13, 7), 6);
        assert_eq!(signed_mod(14, 7), 0);
    }

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 999);
    }

    // #[test]
    // fn test_ex1_part2() {
    //     assert_eq!(part2(&gen1(EX1)), 45000);
    // }

    const EX1: &'static str = r"1
2
-3
3
-2
0
4";
}
