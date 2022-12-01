use std::collections::HashMap;

/// https://adventofcode.com/2021/day/14
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day14)]
pub fn gen1(input: &str) -> (String, Vec<(String,char)>) {
    let mut input = input.lines();
    let polymer_template = input.next().unwrap().to_string();
    assert_eq!(Some(""),input.next()); // discard blank line
    let pair_insertions = input.map(|line|{
        let mut halves = line.split(" -> ");
        (halves.next().unwrap().to_string(),halves.next().unwrap().chars().next().unwrap())
    }).collect();
    (polymer_template, pair_insertions)
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day14, part1)]
pub fn part1(input: &(String, Vec<(String,char)>)) -> usize {
    let (polymer_template, pair_rules) = &input;
    let mut polymer = polymer_template.to_string();
    if cfg!(test) {println!("{}", polymer);}
    let pair_rules: HashMap<(char,char),char>= pair_rules.iter().map(|(pair_str,e_out)|{
        let mut pair = pair_str.chars();
        let e1 = pair.next().unwrap();
        let e2 = pair.next().unwrap();
        assert!(pair.next().is_none());
        ((e1,e2), *e_out)
    }).collect();
    for i in 1..=10 {
        let last = polymer.chars().last().unwrap();
        polymer = polymer.chars().zip(polymer.chars().skip(1))
        .map(|(e1,e2)| {
            let e_middle = *pair_rules.get(&(e1,e2)).unwrap();
            vec![e1,e_middle]
        }).flatten().collect();
        polymer.push(last);
        if cfg!(test) {
            if i <= 4 {println!("{}", polymer);}
            if i==5 {assert_eq!(polymer.len(),97);}
            if i==10 {assert_eq!(polymer.len(),3073);}
        }
    }
    let mut element_counts: HashMap<char,usize> = HashMap::new();
    for e in polymer.chars() {
        let e_cnt = element_counts.entry(e).or_default();
        *e_cnt += 1;
    }
    let max_cnt = element_counts.values().max().unwrap();
    let min_cnt = element_counts.values().min().unwrap();
    max_cnt - min_cnt
}

#[aoc(day14, part2)]
pub fn part2(input: &(String, Vec<(String,char)>)) -> usize {
    let (polymer_template, pair_rules) = &input;
    let polymer = polymer_template.to_string();
    let last = polymer.chars().last().unwrap(); // This will always be the last char
    let mut pair_expander: HashMap<String,[String;2]> = HashMap::new();
    for (pair_str,e_mid) in pair_rules.into_iter() {
        let mut pair = pair_str.chars();
        let e1 = pair.next().unwrap();
        let e2 = pair.next().unwrap();
        assert_eq!(pair.next(), None);
        let mut new_pair1 = String::new();
        new_pair1.push(e1);
        new_pair1.push(*e_mid);
        let mut new_pair2 = String::new();
        new_pair2.push(*e_mid);
        new_pair2.push(e2);        
        pair_expander.insert(pair_str.to_string(),[new_pair1,new_pair2]).is_none(); // is_none() validates inserted key was new
    }

    let mut present: HashMap<String,usize> = HashMap::new();
    polymer_template.chars().zip(polymer_template.chars().skip(1))
    .for_each(|(e1,e2)|{
        let mut existing_pair = String::new();
        existing_pair.push(e1);
        existing_pair.push(e2);
        *present.entry(existing_pair).or_default() += 1;
    });

    for i in 1..=40 {
        if cfg!(test) && i == 10 {
            println!("{:?}", &present)
        }
        // accumulate chars
        // find next row
        let mut next_row: HashMap<String,usize> = HashMap::new();
        for (pair, pair_cnt) in present {
            let new_pairs = pair_expander.get(&pair).unwrap();
            let p1 = new_pairs[0].clone();
            let p2 = new_pairs[1].clone();
            *next_row.entry(p1).or_default() += pair_cnt;
            *next_row.entry(p2).or_default() += pair_cnt;
        }
        present = next_row;
    }
    // Let's tally them up by counting 1st char in each pair
    let mut element_counts:HashMap<char,usize> = HashMap::new();
    for (pair, pair_cnt) in present {
        *element_counts.entry(pair.chars().nth(0).unwrap()).or_default() += pair_cnt;
    }
    // Don't forget to add in last char
    *element_counts.entry(last).or_default() += 1;

    let max_cnt = element_counts.values().max().unwrap();
    let min_cnt = element_counts.values().min().unwrap();
    println!("{}", max_cnt);
    println!("{}", min_cnt);
    max_cnt-min_cnt
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        let (in1,v) = gen1(EX1);
        assert_eq!(&in1,"NNCB");
        assert_eq!(v.len(), 16);
    }

    #[test]
    fn test_ex1_part1() {
        let (in1,v) = gen1(EX1);
        let p1 = part1(&(in1,v));
        assert_eq!(p1, 1588);
    }

    #[test]
    fn test_ex1_part2() {
        let (in1,v) = gen1(EX1);
        let p1 = part2(&(in1,v));
        assert_eq!(p1, 2188189693529);
    }

const EX1: &'static str =
r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

const EX2: &'static str =
r"
";

}