/// https://adventofcode.com/2021/day/N
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(dayN)]
pub fn gen1(input: &str) -> () {
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(dayN, part1)]
pub fn part1(input: &()) -> usize {
    0
}

#[aoc(dayN, part2)]
pub fn part2(input: &()) -> usize {
    0
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        let g = gen1(EX1);
        assert_eq!(g.len(), 999);
    }

    #[test]
    fn test_ex1_part1() {
        let g = gen1(EX1);
        let p1 = part1(&g);
        assert_eq!(p1, 999);
    }

const EX1: &'static str =
r"
";

const EX2: &'static str =
r"
";

}