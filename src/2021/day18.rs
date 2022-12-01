/// https://adventofcode.com/2021/day/18
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 
use Snailfish::*;

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day18)]
pub fn gen1(input: &str) -> Vec<Snailfish> {
    input.lines().map(|line|Snailfish::from(line)).collect()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day18, part1)]
pub fn part1(input: &[Snailfish]) -> u64 {
    let mut fishes = input.into_iter().map(|fish|(*fish).clone());
    let firstfish: Snailfish = fishes.next().unwrap();
    let finalfish: Snailfish = fishes.fold(firstfish,|sum,fish|sum.add(fish));
    finalfish.mag()
}

#[aoc(day18, part2)]
pub fn part2(input: &[Snailfish]) -> u64 {
    input.iter().map(|f1orig| {
        let f1 = f1orig.clone();
        input.iter().map(move |f2| {
            if f1 == *f2 {
                vec![0,0]
            } else {
                vec![f1.clone().add(f2.clone()).mag(),
                     f2.clone().add(f1.clone()).mag()]
            }
        }).flatten()
    }).flatten()
    .max().unwrap()
}

#[derive(PartialEq,Clone,Debug)]
pub enum Snailfish {
    Number(u16),
    Fish(Box<Snailfish>,Box<Snailfish>),
}
impl Snailfish {
    fn reduce(&mut self) -> bool {
        self.explode() || self.split()
    }
    fn explode(&mut self) -> bool {
        let (lfish,exploding_fish,rfish) = Self::explode_search(self,false,1);
        match exploding_fish {
            None => false, // No exploding fish
            Some(efish) => {
                let deadfish = efish.clone();
                *efish = Number(0); // Replace exploded fish with 0
                if let Fish(elfish,erfish) = deadfish {
                    if let (Number(ldead), Number(rdead)) = (*elfish,*erfish) {
                        if let Some(lptr) = lfish {
                            if let Number(lorig) = *lptr {
                                *lptr = Number(lorig+ldead);
                            }
                        };
                        if let Some(rptr) = rfish {
                            if let Number(rorig) = *rptr {
                                *rptr = Number(rorig+rdead);
                            }
                        };
                        true // We exploded a fish!
                    } else {
                        panic!("Exploding fish must be a pair of natural numbers!");
                    }
                } else {
                    panic!("Trouble deconstruction exploding fish");
                }
            } 
        }
    }
    // Return is (Number just before Fish, exploding Fish, Number just after Fish)
    fn explode_search<'a>(fish: &'a mut Snailfish, exploding_found: bool, depth: u16) -> (Option<&'a mut Snailfish>, Option<&'a mut Snailfish>, Option<&'a mut Snailfish>) {
        if exploding_found {
            match fish {
                Number(_) => (None, None, Some(fish)), // Found Number on right, return it
                Fish(left,right) => {
                    let (_,_,r) = Self::explode_search(left, true, depth+1);
                    match r {
                        Some(_) => (None, None, r), // Found Number on right, return it
                        None => (None, None, Self::explode_search(right, true, depth+1).2),
                    }
                },
            }
        } else {
            match fish {
                Number(_) => (Some(fish), None, None),
                Fish(_,_) if depth == 5 => (None, Some(fish), None), // Kaboom!  Return this exploding fish
                Fish(left,right) => {
                    let (l,e,r) = Self::explode_search(left, false, depth+1);
                    match (e,r) {
                        (Some(efish),Some(rnum)) => (l,Some(efish),Some(rnum)),
                        (Some(efish),None) => {
                            let (_,_,rr) = Self::explode_search(right, true, depth+1);
                            (l,Some(efish),rr)
                        },
                        (None,None) => {
                            let (ll,ee,rr) = Self::explode_search(right, false, depth+1);
                            match ll {
                                None => (l,ee,rr),
                                Some(_) => (ll,ee,rr),
                            }
                        },
                        (None, Some(_)) => panic!("Got a right before an explosion!")
                    }
                },
            }
        }
    }
    fn split(&mut self) -> bool {
        match Self::split_search(self) {
            None => false,
            Some(fish) => {
                if let Number(ten_or_more) = *fish {
                    *fish = Fish(Box::new(Number(ten_or_more/2)),Box::new(Number((ten_or_more+1)/2)));
                    true
                } else {
                    panic!("split_search returned an unnatural fish!")
                }
            }
        }
    }
    fn split_search<'a>(fish: &'a mut Snailfish) -> Option<&'a mut Snailfish> {
        match fish {
            Number(n) if *n >= 10 => Some(fish),
            Number(_) => None,
            Fish(left, right) => {
                if let Some(later_fish) = Self::split_search(left) {
                    Some(later_fish)
                } else {
                    if let Some(later_fish) = Self::split_search(right) {
                        Some(later_fish)
                    } else {
                        None
                    }
                }
            }
        }
    }

    fn add(self, addendum: Self) -> Self {
        let mut sum = Fish(Box::new(self),Box::new(addendum));
        while sum.reduce() {};
        sum
    }
    fn mag(&self) -> u64 {
        match self {
            Number(n) => *n as u64,
            Fish(left,right) => 3 * (*left).mag() + 2 * (*right).mag(),
        }
    }
    fn split_at_my_comma(txt:&str) -> (&str,&str) {
        let mut nested = 0;
        let my_comma_ndx = txt.chars().enumerate()
        .fold(None,|maybe_found,(ndx,c)| match maybe_found {
            Some(_) => maybe_found,
            None => match (c,nested) {
                (',',0) => Some(ndx), // This is the only path to success, propogate
                ('[',_) => {
                    nested += 1;
                    None
                },
                (']',n) if n>0 => {
                    nested -= 1;
                    None
                },
                (']',n) if n==0 => 
                    panic!("Found unexpected closing brace at ndx={} in '{}'", ndx, txt),
                _ => None,
            }
        }).unwrap();
        txt.split_at(my_comma_ndx)
    }
}
impl From<&str> for Snailfish {
    fn from(txt:&str) -> Self {
        let first = txt.chars().nth(0).expect("Can't build a Snailfish from an empty string!");
        let length = txt.chars().count();
        match first {
            d if d.is_ascii_digit() => {
                if length == 1 {
                    Number(first as u16 - '0' as u16)
                } else {
                    panic!("Strings with single digits are Snailfish, this is not: \"{}\"", txt);
                }
            },
            '[' => {
                assert_eq!(txt.chars().last(), Some(']'));
                let inner_txt = txt.chars().skip(1).take(length-2).collect::<String>();
                let (left, right) = Self::split_at_my_comma(&inner_txt);
                let right = right.split_at(1).1;
                Fish(
                    Box::new(Self::from(left)),
                    Box::new(Self::from(right)),
                )
            },
            _ => panic!("Snailfish string must start with a '[' or be a single digit"),
        }
    }
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_gen() {
    //     let g = gen1("[[1,2],[[3,4],5]]");
    //     assert_eq!(g.len(), 1);
    // }
    // #[test]
    // fn test_explode1() {
    //     let g = gen1("[[[[[9,8],1],2],3],4]");
    //     let mut fishes = g.into_iter();
    //     let mut first = fishes.next().unwrap();
    //     first.reduce();
    //     // let sum = fishes.fold(first,|sum,fish|sum.add(fish));
    //     let mut g = gen1("[[[[0,9],2],3],4]");
    //     let expected = g.pop().unwrap();
    //     assert_eq!(first, expected);
    // }
    #[test]
    fn test_explode2() {
        let g = gen1("[0,0]\n[[[[9,7],1],2],3]");
        let mut fishes = g.into_iter();
        let first = fishes.next().unwrap();
        let sum = fishes.fold(first,|sum,fish|sum.add(fish));
        let mut g = gen1("[[0,9],[[[0,8],2],3]]");
        let expected = g.pop().unwrap();
        assert_eq!(sum, expected);
    }
    #[test]
    fn test_split_ex() {
        let g = gen1("[[[[4,3],4],4],[7,[[8,4],9]]]\n[1,1]");
        let mut fishes = g.into_iter();
        let first = fishes.next().unwrap();
        let sum = fishes.fold(first,|sum,fish|sum.add(fish));
        let mut g = gen1("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        let expected = g.pop().unwrap();
        assert_eq!(sum, expected);
    }
    // #[test]
    // fn test_sum2_part1() {
    //     let g = gen1("[1,1]\n[2,2]");
    //     let mut fishes = g.into_iter();
    //     let first = fishes.next().unwrap();
    //     let sum = fishes.fold(first,|sum,fish|sum.add(fish));
    //     let g = gen1("[1,1]\n[2,2]");
    //     let p1 = part1(&g);
    //     assert_eq!(sum.mag(), p1);
    // }
    // #[test]
    // fn test_sum2() {
    //     let g = gen1("[1,1]\n[2,2]");
    //     let mut fishes = g.into_iter();
    //     let first = fishes.next().unwrap();
    //     let sum = fishes.fold(first,|sum,fish|sum.add(fish));
    //     // expected
    //     let mut g = gen1("[[1,1],[2,2]]");
    //     let exp = g.pop().unwrap();
    //     assert_eq!(sum, exp);
    // }
    // #[test]
    // fn test_sum4() {
    //     let g = gen1("[1,1]\n[2,2]\n[3,3]\n[4,4]");
    //     let mut fishes = g.into_iter();
    //     let first = fishes.next().unwrap();
    //     let sum = fishes.fold(first,|sum,fish|sum.add(fish));
    //     let mut g = gen1("[[[[1,1],[2,2]],[3,3]],[4,4]]");
    //     let exp = g.pop().unwrap();
    //     assert_eq!(sum, exp);
    // }
    // #[test]
    // fn test_sum2x2() {
    //     let g = gen1("[1,1]\n[2,2]");
    //     let mut fishes = g.into_iter();
    //     let first = fishes.next().unwrap();
    //     let sum1 = fishes.fold(first,|sum,fish|sum.add(fish));
    //     let g = gen1("[3,3]\n[4,4]");
    //     let mut fishes = g.into_iter();
    //     let first = fishes.next().unwrap();
    //     let sum2 = fishes.fold(first,|sum,fish|sum.add(fish));
    //     let sum_of_sums = sum1.add(sum2);
    //     // expected
    //     let g = gen1("[[[1,1],[2,2]],[[3,3],[4,4]]]");
    //     let exp = g[0].clone();
    //     assert_eq!(sum_of_sums, exp);
    // }
    // #[test]
    // fn test_ex1_part1() {
    //     let p1 = part1(&gen1(EX1));
    //     assert_eq!(p1, 4140);
    // }
    // #[test]
    // fn test_mag1_part1() {
    //     let p1 = part1(&gen1("[[1,2],[[3,4],5]]"));
    //     assert_eq!(p1, 143);
    // }
    // #[test]
    // fn test_mag2_part1() {
    //     let p1 = part1(&gen1("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    //     assert_eq!(p1, 1384);
    // }
    // #[test]
    // fn test_mag3_part1() {
    //     let p1 = part1(&gen1("[[[[1,1],[2,2]],[3,3]],[4,4]]"));
    //     assert_eq!(p1, 445);
    // }
    // #[test]
    // fn test_mag6_part1() {
    //     let p1 = part1(&gen1("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"));
    //     assert_eq!(p1, 3488);
    // }

const EX1: &'static str =
r"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

}