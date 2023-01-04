/// https://adventofcode.com/2022/day/20
/// DAN AoC: https://adventofcode.com/2022/leaderboard/private/view/380786
/// HLOTYAK: https://adventofcode.com/2022/leaderboard/private/view/951754

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
    // Instead of mixing values, we'll mix ptrs (indices) to those values
    let mut input_index: Vec<usize> = (0..len).collect();
    // The above will be scrambled, so we need a descrambler
    // This answers the question: If I'm looking for the scrambled position of the index originally in position i,
    // in what scambled index would I find that?
    let mut input_index_lookup = input_index.clone();
    // Algo:  reverse-scramble indices to original (input) values based on that value -- leave input array untouched
    // The complexity is that forward movement can never end up at the end of the list, but it can end up at the begginning.
    // And backward motion can never end up at the begginning of the list, but it can end up at the end.
    // This endpoint management is vital because the final answer depends on absolute positions, not just relative.
    // When moving forward, insertion between end and beginning becomes new beginning. 
    // When moving backward, insertion between end and beginning becomes new end.
    for i in 0..len {
        // Debug
        let value = input[i];
        #[cfg(test)]
        {
            println!("Input Indexes: {:?}", input_index);
            println!("Index lookup:  {:?}", input_index_lookup);
            println!("Moving '{}'", value);
        }
        assert_eq!(input[input_index[input_index_lookup[i]]], value);  // Verify descramble of scramble matches original
        if value != 0 {
            // Where is this value (indirectly) right now?
            let from_pos = input_index_lookup[i];
            #[cfg(test)] dbg!(from_pos);
            if value > 0 {
                // And where should the value be moved?
                let to_pos = signed_mod(from_pos as isize + value, len);
                #[cfg(test)] dbg!(to_pos);
                // mix! (move an element in input_index from the "from" position to just after the "to" position, being careful with endpoints.)
                let ndx_to_move = input_index[from_pos];
                #[cfg(test)] dbg!(ndx_to_move);
                // Start by inserting a copy of the item -- this is simpler than deleting then adding, or doing both simultaneously
                input_index = if to_pos == len-1 {
                    [&[ndx_to_move], &input_index[..]].concat()
                } else {
                    [&input_index[..=to_pos], &[ndx_to_move], &input_index[to_pos+1..]].concat()
                };
                #[cfg(test)] println!("Input Indexes after add: {:?}", input_index);
                assert_eq!(input_index.len(), len + 1);
                // Now delete the item from its "from" position, which may have been bumped right by 1.
                let from_pos = if to_pos < from_pos || to_pos == len-1 {from_pos + 1} else {from_pos};
                input_index = [&input_index[..from_pos], &input_index[from_pos+1..]].concat();
                #[cfg(test)] println!("Input Indexes after remove: {:?}", input_index);
                assert_eq!(input_index.len(), len);
            } else {
                // And where should the value be moved?
                let to_pos = signed_mod(from_pos as isize + value - 1, len);
                #[cfg(test)] dbg!(to_pos);
                // mix! (move an element in input_indexes from the "from" position to just after the "to" position, being careful with endpoints.)
                let ndx_to_move = input_index[from_pos];
                // Start by inserting a copy of the item -- this is simpler than deleting then adding, or doing both simultaneously
                input_index = [&input_index[..=to_pos], &[ndx_to_move], &input_index[to_pos+1..]].concat();
                // Now delete the item from its "from" position, which may have been bumped right by 1.
                let from_pos = if to_pos < from_pos {from_pos + 1} else {from_pos};
                input_index = [&input_index[..from_pos], &input_index[from_pos+1..]].concat();
            }
            // Now rebuild the index lookup table
            let mut tmp: Vec<(usize,usize)> = input_index.iter().enumerate().map(|(i,ndx)| (*ndx,i)).collect();
            tmp.sort();
            input_index_lookup = tmp.iter().map(|(_,i)|*i).collect();
        }
        // Debug
        #[cfg(test)]
        {
            let ddd: Vec<isize> = input_index.iter().map(|ndx|input[*ndx]).collect();
            println!("New values:    {:?}", ddd);
        }
    }

    // Finally, using the mixed indices, "mix" the inputs to decrypt:
    let mut mixed_input = Vec::new();
    for ndx in input_index_lookup {
        mixed_input.push(input[ndx]);
    }

    let ndx_of_zero = mixed_input.iter()
        .enumerate()
        .fold(None, |ndx0, (i, v)| if 0 == *v {Some(i)} else {ndx0})
        .unwrap();

    dbg!(ndx_of_zero);
    [(1000+ndx_of_zero) % len,(2000+ndx_of_zero) % len,(3000+ndx_of_zero) % len].into_iter()
        .map(|ndx|dbg!(mixed_input[ndx])).sum()
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
        assert_eq!(part1(&gen1(EX1)), 3);
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
