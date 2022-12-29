/// https://adventofcode.com/2022/day/1
/// DAN AoC: https://adventofcode.com/2022/leaderboard/private/view/380786
/// HLOTYAK: https://adventofcode.com/2022/leaderboard/private/view/951754


use std::collections::HashSet;

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day18)]
pub fn gen1(input: &str) -> Vec<[usize;3]> {
    input.lines()
    .map(|line|line.split(",")
        .map(|s|s.parse().unwrap()).collect::<Vec<_>>())
    .map(|v| {
        [v[0], v[1], v[2]]
    }).collect()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day18, part1)]
pub fn part1(input: &[[usize;3]]) -> usize {
    // Shift coords in 3-space by (1,1,1) eliminate usize underflow.  Algo needs only relative coords.
    let occupied = input.iter().map(|[x,y,z]|[*x+1,*y+1,*z+1]).collect::<HashSet<_>>();
    occupied.iter()
    .map(|d3| [
        [d3[0]+1, d3[1], d3[2]],
        [d3[0]-1, d3[1], d3[2]],
        [d3[0], d3[1]+1, d3[2]],
        [d3[0], d3[1]-1, d3[2]],
        [d3[0], d3[1], d3[2]+1],
        [d3[0], d3[1], d3[2]-1],
    ]).flatten()
    .filter(|d3| !occupied.contains(d3))
    .count()
}

#[aoc(day18, part2)]
pub fn part2(input: &[[usize;3]]) -> usize {
    // Shift coords in 3-space by (2,2,2) to half-eliminate future boundary checks.  Algo needs only relative coords.
    let occupied = input.iter().map(|[x,y,z]|[*x+2,*y+2,*z+2]).collect::<HashSet<_>>();
    // To find all adjacents that can be reached from any point in space NOT inside a bubble...
    // Construct a map in 3-space...
    // Find 3d maxima
    let mmm = occupied.iter()
        .fold([usize::MIN,usize::MIN,usize::MIN], |mmm, pt| {
             [mmm[0].max(pt[0]),mmm[1].max(pt[1]),mmm[2].max(pt[2])]
        });
    // Add 2 to maxima for the 2nd half of the boundary check elimination
    let mmm = [mmm[0]+2,mmm[1]+2,mmm[2]+2];
    // Create a 3-space map, initially all unknown (None)
    let mut air = (0..=mmm[0])
        .map(|_x| (0..=mmm[1])
            .map(|_y| (0..=mmm[2])
                .map(|_z| None
                ).collect::<Vec<_>>()
            ).collect::<Vec<_>>()
        ).collect::<Vec<_>>();
    // Now fill in all 6 boundary planes
    for x in [0, mmm[0]] {
        for y in 0..=mmm[1] {
            for z in 0..=mmm[2] {
                air[x][y][z] = Some('B'); // 'B' means Boundary -- we won't count these later
            }
        }
    }
    for y in [0, mmm[1]] {
        for x in 0..=mmm[0] {
            for z in 0..=mmm[2] {
                air[x][y][z] = Some('B'); // 'B' means Boundary
            }
        }
    }
    for z in [0, mmm[2]] {
        for x in 0..=mmm[0] {
            for y in 0..=mmm[1] {
                air[x][y][z] = Some('B'); // 'B' means Boundary
            }
        }
    }
    // Now add in all known occupied
    occupied.iter().for_each(|[x,y,z]| {
        assert!(air[*x][*y][*z] == None);
        air[*x][*y][*z] = Some('O'); // 'O' means droplet, not air.
    });
    // Now search for unknowns and mark all we can reach as air '.'
    // The key to the whole algo is that we search incrementally from known air, and never beyond an occupied spot,
    // so we can't get _inside_ a closed space
    let mut unknowns_to_search = HashSet::from([[1,1,1]]);
    while !unknowns_to_search.is_empty() {
        let mut new_unknowns = HashSet::new();
        for &[x,y,z] in &unknowns_to_search {
            if air[x][y][z] == None {
                air[x][y][z] = Some('.'); // known free air!
                for chk in [[x-1,y,z],[x+1,y,z],[x,y-1,z],[x,y+1,z],[x,y,z-1],[x,y,z+1]] {
                    if air[chk[0]][chk[1]][chk[2]] == None {
                        new_unknowns.insert(chk);
                    }
                }
            }
        }
        unknowns_to_search = new_unknowns;
    }
    // Debug
    #[cfg(test)]
    {
        let none_cnt = {
            air.iter().map(|v|v.iter()).flatten().map(|a|a.iter()).flatten()
                .filter(|maybe_air| &&None == maybe_air)
                .count()
        };
        let air_cnt = {
            air.iter().map(|v|v.iter()).flatten().map(|a|a.iter()).flatten()
                .filter(|maybe_air| &&Some('.') == maybe_air)
                .count()
        };
        let occupied_cnt = {
            air.iter().map(|v|v.iter()).flatten().map(|a|a.iter()).flatten()
                .filter(|maybe_air| &&Some('O') == maybe_air)
                .count()
        };
        let boundary_cnt = {
            air.iter().map(|v|v.iter()).flatten().map(|a|a.iter()).flatten()
                .filter(|maybe_air| &&Some('B') == maybe_air)
                .count()
        };
        let [x,y,z] = [mmm[0]+1,mmm[1]+1,mmm[2]+1];
        println!("3-space size: {:?}", &[x,y,z]);
        println!("Boundary count: {}", boundary_cnt);
        println!("Occupied count: {}", occupied_cnt);
        println!("Air count: {}", air_cnt);
        println!("Remaining None after search: {}", none_cnt);
        assert_eq!(boundary_cnt,x*y*2+x*(z-2)*2+(y-2)*(z-2)*2);
    }
    // Finally, we can get the answer we've been searching for...
    // By repeating the Part 1 algo but replacing !occupied filter with an "air filter" (air == Some('.'))
    occupied.iter()
    .map(|d3| [
        [d3[0]+1, d3[1], d3[2]],
        [d3[0]-1, d3[1], d3[2]],
        [d3[0], d3[1]+1, d3[2]],
        [d3[0], d3[1]-1, d3[2]],
        [d3[0], d3[1], d3[2]+1],
        [d3[0], d3[1], d3[2]-1],
    ]).flatten()
    .filter(|&[x,y,z]| air[x][y][z] == Some('.'))
    .count()
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_part1() {
        assert_eq!(part1(&gen1(SPHERE)), 36);
    }
    #[test]
    fn test_ex1_part1() {
        assert_eq!(part1(&gen1(EX1)), 10);
    }
    #[test]
    fn test_ex2_part1() {
        assert_eq!(part1(&gen1(EX2)), 64);
    }

    #[test]
    fn test_sphere_part2() {
        assert_eq!(part2(&gen1(SPHERE)), 30);
    }
    #[test]
    fn test_ex1_part2() {
        assert_eq!(part2(&gen1(EX2)), 58);
    }

    const SPHERE: &'static str = 
r"0,1,1
2,1,1
1,0,1
1,2,1
1,1,0
1,1,2";
    const EX1: &'static str = r"1,1,1
2,1,1";
    const EX2: &'static str = r"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

}
