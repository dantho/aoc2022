/// https://adventofcode.com/2022/day/20
/// DAN AoC: https://adventofcode.com/2022/leaderboard/private/view/380786
/// HLOTYAK: https://adventofcode.com/2022/leaderboard/private/view/951754


use std::cmp::Ordering::{Greater, Equal, Less};

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
    let len = dbg!(input.len());
    // Instead of mixing values, we'll mix ptrs (input index) to those values
    let mut input_index: Vec<usize> = (0..len).collect();
    // The above will be scrambled, so we need a descrambler
    // This answers the question: If I'm looking for the scrambled position of the index originally in position i,
    // in what scambled index would I find that?
    let mut input_index_lookup = input_index.clone();
    // Algo:  reverse-scramble indices to original (input) values based on that value -- leave input array untouched
    for i in 0..input.len() {
        // Debug
        let value = input[i];
        #[cfg(test)]
        {
            println!("Input Indexes: {:?}", input_index);
            println!("Index lookup:  {:?}", input_index_lookup);
            println!("Moving '{}'", value);
        }
        assert_eq!(input[input_index[input_index_lookup[i]]], value);

        let before_pos = input_index_lookup[i];
        let after_pos = signed_mod(before_pos as isize + value + if value >= 0 {0} else {-1}, len);
        #[cfg(test)]
        {
            dbg!(before_pos);
            dbg!(after_pos);
        }
        // mix! (move an element in input_indexes from the before position to just after the after position)
        let ndx_to_move = input_index[before_pos];
        input_index = match before_pos.cmp(&after_pos) {
            Less => [&input_index[..before_pos], &input_index[(before_pos+1)..=after_pos], &[ndx_to_move], &input_index[(after_pos+1)..]].concat(),
            Equal => input_index,
            Greater => [&input_index[..=after_pos], &[ndx_to_move], &input_index[(after_pos+1)..before_pos], &input_index[(before_pos+1)..]].concat(),
        };
        // Now rebuild the index lookup table
        let mut tmp: Vec<(usize,usize)> = input_index.iter().enumerate().map(|(i,ndx)| (*ndx,i)).collect();
        tmp.sort();
        input_index_lookup = tmp.iter().map(|(_,i)|*i).collect();
        // Debug
        #[cfg(test)]
        {
            let ddd: Vec<isize> = input_index.iter().map(|ndx|input[*ndx]).collect();
            println!("New values:    {:?}", ddd);
        }
    }

    // Now that the indices are all reverse-scrambled, use them to de-scramble the input.
    let mut descrambled_input = input.into_iter()
        .enumerate()
        .map(|(orig_ndx, v)| (input_index[orig_ndx], *v))
        .collect::<Vec<_>>();
    // Actual descrambling event
    descrambled_input.sort();

    let input = descrambled_input.into_iter().map(|(_,v)|v).collect::<Vec<_>>();

    let index_of_zero = input.iter()
        .enumerate()
        .fold(None, |ndx0, (i, v)| if 0 == *v {Some(i)} else {ndx0})
        .unwrap();

    dbg!([(1000+index_of_zero) % len,(2000+index_of_zero) % len,(3000+index_of_zero) % len]).into_iter()
        .map(|i|dbg!(input[i])).sum()
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
        assert_eq!(signed_mod(-14, 7), 0);
        assert_eq!(signed_mod(-7, 7), 0);
        assert_eq!(signed_mod(0, 7), 0);
        assert_eq!(signed_mod(7, 7), 0);
        assert_eq!(signed_mod(14, 7), 0);
        assert_eq!(signed_mod(14000007, 7), 0);
        assert_eq!(signed_mod(13, 7), 6);
        assert_eq!(signed_mod(6, 7), 6);
        assert_eq!(signed_mod(-1, 7), 6);
        assert_eq!(signed_mod(-8, 7), 6);
        assert_eq!(signed_mod(-9, 7), 5);
        assert_eq!(signed_mod(-10, 7), 4);
        assert_eq!(signed_mod(-11, 7), 3);
        assert_eq!(signed_mod(-12, 7), 2);
        assert_eq!(signed_mod(-13, 7), 1);
    }

    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 3+996);
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
