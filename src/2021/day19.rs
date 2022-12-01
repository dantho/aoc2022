/// https://adventofcode.com/2021/day/19
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 
use std::ops::{Add,Sub};
use std::collections::{HashMap,HashSet};

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day19)]
pub fn gen1(input: &str) -> Vec<Vec<Coord>> {
    input.split("\n\n")
    .map(|scannerinput|{
        scannerinput.lines()
        .skip(1)
        .map(|coords|{
            let mut pts = coords.split(",").map(|num|num.parse().unwrap());
            Coord (pts.next().unwrap(),pts.next().unwrap(),pts.next().unwrap())
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day19, part1)]
pub fn part1(input: &[Vec<Coord>]) -> usize {
    // Strategy:
    // Given Scanner 0 as reference data AND orientation
    // Iterate through all ref data, subtracting EACH of target data
    // Aggregate diffs in a hashmap, counting each unique diff
    // Find/store max and associated diff for this rotation (of target)
    // Rotate target and repeat
    // Find max max of rotations -- this should match the reference rotation
    let mut scanners = input.iter().enumerate();
    // Start collection of known beacons with scanner 0 -- good by definition
    let mut beacons_relative_to_0 = scanners.next().unwrap().1.into_iter()
    .map(|ptr|*ptr).collect::<HashSet<_>>();
    let mut processed_scanners = vec![false; input.len()];
    processed_scanners[0] = true;
    let rots = find_unique_rotations();

    // loop until all scanners are processed (see 'break' condition at bottom of loop)
    loop {
        let scanner_rots = input.into_iter().enumerate()
        .filter(|(i,_)| !processed_scanners[*i])
        .map(|(scanner_id,target_scanner)| {
            let diff_matrix = beacons_relative_to_0.iter()
            .map(|ref_beacon|{
                target_scanner.iter()
                .map(|target_beacon|{
                    rots.iter()
                    .map(|rot| {
                        let diff = target_beacon.rotate(*rot)-*ref_beacon;
                        (*rot, diff)
                    }).collect::<Vec<_>>()
                }).flatten().collect::<Vec<_>>()
            }).flatten().collect::<Vec<(Coord,Coord)>>();
            let mut unique_count = HashMap::new();
            for (rot,diff) in diff_matrix {
                *unique_count.entry((rot,diff)).or_default() += 1;
            }
            let ((best_rot, best_diff),count_of_best) = unique_count.iter()
            .fold(((Coord (0,0,0),Coord (0,0,0)), isize::MIN),|(bestrotdiff,maxsofar), (&rot_diff,&count)|
                if count > maxsofar {(rot_diff, count)} else {(bestrotdiff,maxsofar)}
            );
            if cfg!(test) { println!("For scanner #{}, found {} uniques with rotation {:?}", scanner_id, count_of_best, best_rot);}
            (scanner_id, best_rot, best_diff, count_of_best)
        }).collect::<Vec<_>>();
        if cfg!(test) { println!("Scanner Rotations: {:?}", scanner_rots);}
        for (scanner_id, rot, diff, cnt) in scanner_rots {
            if cnt >= 12 {
                if cfg!(test) {println!("Processing Scanner ID# {} of 0 to {}", scanner_id, processed_scanners.len()-1);}
                for beacon in &input[scanner_id] {
                    beacons_relative_to_0.insert(beacon.rotate(rot)-diff);
                    if cfg!(test) {println!("beacons HashSet size: {}", beacons_relative_to_0.len());}
                }
                processed_scanners[scanner_id] = true;
            }
        }
        // Are we done yet?
        if processed_scanners.iter().fold(true,|prev,&b|prev&&b) {break;} // All Done!
        if cfg!(test) {println!("{:?}",processed_scanners);}
    }
    beacons_relative_to_0.len()
}

#[aoc(day19, part2)]
pub fn part2(input: &[Vec<Coord>]) -> isize {
    let mut scanners = input.iter().enumerate();
    // Start collection of known beacons with scanner 0 -- good by definition
    let mut beacons_relative_to_0 = scanners.next().unwrap().1.into_iter()
    .map(|ptr|*ptr).collect::<HashSet<_>>();
    let mut processed_scanners = vec![false; input.len()];
    processed_scanners[0] = true;
    let mut scanner_locs = vec![Coord (0,0,0)];
    let rots = find_unique_rotations();

    // loop until all scanners are processed (see 'break' condition at bottom of loop)
    loop {
        let scanner_rots = input.into_iter().enumerate()
        .filter(|(i,_)| !processed_scanners[*i])
        .map(|(scanner_id,target_scanner)| {
            let diff_matrix = beacons_relative_to_0.iter()
            .map(|ref_beacon|{
                target_scanner.iter()
                .map(|target_beacon|{
                    rots.iter()
                    .map(|rot| {
                        let diff = target_beacon.rotate(*rot)-*ref_beacon;
                        (*rot, diff)
                    }).collect::<Vec<_>>()
                }).flatten().collect::<Vec<_>>()
            }).flatten().collect::<Vec<(Coord,Coord)>>();
            let mut unique_count = HashMap::new();
            for (rot,diff) in diff_matrix {
                *unique_count.entry((rot,diff)).or_default() += 1;
            }
            let ((best_rot, best_diff),count_of_best) = unique_count.iter()
            .fold(((Coord (0,0,0),Coord (0,0,0)), isize::MIN),|(bestrotdiff,maxsofar), (&rot_diff,&count)|
                if count > maxsofar {(rot_diff, count)} else {(bestrotdiff,maxsofar)}
            );
            if cfg!(test) { println!("For scanner #{}, found {} uniques with rotation {:?}", scanner_id, count_of_best, best_rot);}
            (scanner_id, best_rot, best_diff, count_of_best)
        }).collect::<Vec<_>>();
        if cfg!(test) { println!("Scanner Rotations: {:?}", scanner_rots);}
        for (scanner_id, rot, diff, cnt) in scanner_rots {
            if cnt >= 12 {
                scanner_locs.push(diff);
                if cfg!(test) {println!("Processing Scanner ID# {} of 0 to {}", scanner_id, processed_scanners.len()-1);}
                if cfg!(test) {println!("Before processing len {}", beacons_relative_to_0.len());}
                for beacon in &input[scanner_id] {
                    beacons_relative_to_0.insert(beacon.rotate(rot)-diff);
                }
                if cfg!(test) {println!("After processing len {}", beacons_relative_to_0.len());}
                processed_scanners[scanner_id] = true;
            }
        }
        // Are we done yet?
        if processed_scanners.iter().fold(true,|prev,&b|prev&&b) {break;} // All Done!
        if cfg!(test) {println!("{:?}",processed_scanners);}
    }
    // Find manhattan distances between scanners
    let ans = scanner_locs.iter().map(|s1| {
        scanner_locs.iter().map(|s2| {
            s1.manhattan(*s2)
        }).max().unwrap()
    }).max().unwrap();

    if !cfg!(test) {
        assert!(ans != 14868);
        assert!(ans < 14868);
    }
    ans
}

fn find_unique_rotations() -> Vec<Coord> {
    let original = Coord (1,2,3);
    let mut keep_only_first = HashMap::new();
    for x in 0..4 {
        for y in 0..4 {
            for z in 0..4 {
                let trial = Coord (x,y,z);
                let result = original.rotate(trial);
                if !keep_only_first.contains_key(&result) {
                    keep_only_first.insert(result,trial);
                }
            }
        }
    }
    let mut v:Vec<_> = keep_only_first.iter().map(|(_,rot)|*rot).collect();
    v.sort();
    v
}

#[derive(Copy,Clone,PartialEq,Eq,Ord,PartialOrd,Hash,Debug)]
pub struct Coord (isize, isize, isize);
impl Coord {
    pub fn manhattan(self, p2: Self) -> isize {
        let diff = self-p2;
        diff.0.abs() + diff.1.abs() + diff.2.abs()
    }
    pub fn rotate(self, rot: Self) -> Self {
        let Coord (mut x, mut y, mut z) = self;
        match rot.0 % 4 {
            0 => (),
            1 => {let zz = z; z = y; y = -zz},
            2 => {z = -z; y = -y},
            3 => {let zz = z; z = -y; y = zz},
            _ => panic!("Why do we even HAVE that lever?"),
        } 
        match rot.1 % 4 {
            0 => (),
            1 => {let xx = x; x = z; z = -xx},
            2 => {x = -x; z = -z},
            3 => {let xx = x; x = -z; z = xx},
            _ => panic!("Why do we even HAVE that lever?"),
        } 
        match rot.2 % 4 {
            0 => (),
            1 => {let yy = y; y = x; x = -yy},
            2 => {x = -x; y = -y},
            3 => {let yy = y; y = -x; x = yy},
            _ => panic!("Why do we even HAVE that lever?"),
        }
        Coord (x,y,z) 
    }    
}
impl Sub for Coord {
    type Output = Self;

    fn sub(self, other:Self) -> Self::Output {
        Self (self.0-other.0, self.1-other.1, self.2-other.2)
    }
}
impl Add for Coord {
    type Output = Self;

    fn add(self, other:Self) -> Self::Output {
        Self (self.0+other.0, self.1+other.1, self.2+other.2)
    }
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
        assert_eq!(g.len(), 5);
        assert_eq!(g[0].len(), 6);

        let g = gen1(EX2);
        assert_eq!(g.len(), 5);
        assert_eq!(g[0].len(), 25);
    }

    #[test]
    fn test_rot_scanner() {
        let g = gen1(EX1);
        assert_eq!(g.len(), 5);
        assert_eq!(g[0].len(), 6);
        // The following known rotations were determined using Part 1 with some added debug code
        let known_rotations = [Coord (0,0,0), Coord (1, 2, 0), Coord (0, 1, 0), Coord (0, 3, 2), Coord (1, 0, 3)];
        let reference_scanner = &g[0];
        for (scanner, rot) in g.iter().zip(&known_rotations) {
            let rotated_scanner = scanner.iter().map(|c|c.rotate(*rot)).collect::<Vec<_>>();
            assert_eq!(&rotated_scanner, reference_scanner);
        }
    }

    #[test]
    fn test_manhattan() {
        let p1 = Coord (1,1,1);
        let p2 = Coord (5,6,7);
        let dist = p1.manhattan(p2);
        assert_eq!(dist,15);
    }
    #[test]
    fn test_sub() {
        let diff = Coord (1,1,1) - Coord (5,6,7);
        assert_eq!(diff,Coord (-4,-5,-6));
    }
    #[test]
    fn test_add() {
        let diff = Coord (1,1,1) + Coord (5,6,7);
        assert_eq!(diff,Coord (6,7,8));
    }
    #[test]
    fn test_rot() {
        let p = Coord (1,2,3);
        let rot = Coord (0,0,0);
        assert_eq!(p.rotate(rot),p);
        let rot = Coord (2,0,0);
        assert_eq!(p.rotate(rot),Coord (1,-2,-3));
        let rot = Coord (0,2,0);
        assert_eq!(p.rotate(rot),Coord (-1,2,-3));
        let rot = Coord (0,0,2);
        assert_eq!(p.rotate(rot),Coord (-1,-2,3));
        let rot = Coord (3,0,0);
        assert_eq!(p.rotate(rot),Coord (1,3,-2));
        let rot = Coord (0,3,0);
        assert_eq!(p.rotate(rot),Coord (-3,2,1));
        let rot = Coord (0,0,3);
        assert_eq!(p.rotate(rot),Coord (2,-1,3));
        let rot = Coord (4,0,0);
        assert_eq!(p.rotate(rot),p);
        let rot = Coord (0,4,0);
        assert_eq!(p.rotate(rot),p);
        let rot = Coord (0,0,4);
        assert_eq!(p.rotate(rot),p);
        let rot = Coord (1,1,1);
        assert_eq!(p.rotate(rot),Coord (3,2,-1));
    }
    #[test]
    fn test_ex2_part1() {
        let g = gen1(EX2);
        let p1 = part1(&g);
        assert_eq!(p1, 79);
    }

    #[test]
    fn test_ex2_part2() {
        let g = gen1(EX2);
        let p2 = part2(&g);
        assert_eq!(p2, 3621);
    }

const EX1: &'static str =
r"--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7

--- scanner 0 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0

--- scanner 0 ---
-1,-1,-1
-2,-2,-2
-3,-3,-3
-1,-3,-2
4,6,5
-7,0,8

--- scanner 0 ---
1,1,-1
2,2,-2
3,3,-3
1,3,-2
-4,-6,5
7,0,8

--- scanner 0 ---
1,1,1
2,2,2
3,3,3
3,1,2
-6,-4,-5
0,7,-8";

const EX2: &'static str =
r"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

}