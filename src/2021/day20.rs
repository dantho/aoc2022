/// https://adventofcode.com/2021/day/20
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 
use std::collections::VecDeque;

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day20)]
pub fn gen1(input: &str) -> (VecDeque<VecDeque<u8>>,Vec<u8>) {
    let mut lines = input.lines();
    let decoder = lines.next().unwrap().chars()
    .map(|c|char2bit(c).unwrap())
    .collect::<Vec<u8>>();
    lines.next();
    let initial = lines.map(|line|line.chars()
    .map(|c|char2bit(c).unwrap())
    .collect::<VecDeque<u8>>())
    .collect::<VecDeque<_>>();

    (initial,decoder)
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day20, part1)]
pub fn part1(input: &(VecDeque<VecDeque<u8>>, Vec<u8>)) -> usize {
    let decoder = &input.1;
    assert_eq!(decoder.len(), 512);
    let mut image = input.0.clone();

    // Add capacity for the future
    // This may be overkill in Part 1
    for v in &mut image {
        v.reserve(1000);
    }
    image.reserve(1000);

    // Need a 2nd copy for efficient algo
    let mut buffer = image.clone(); // Does this clone capacity, too?  Hope so!

    let mut image_ptr = &mut image;
    let mut buffer_ptr = &mut buffer;

    let (pad_with_even, pad_with_odd) = match (decoder[0], decoder[decoder.len()-1]) {
        (0,0) => (0,0),
        (0,1) => (0,0), // Since we start with zeros... and they never change
        (1,0) => (0,1), // toggle
        (1,1) => (1,1), // only initial iteration is 0, all others are 1
        _ => panic!("Should be dealing only with bits here"),
    };
    // let mut image_ptr;
    // let mut buffer_ptr;
    // execute algo n times
    let n = 2;
    for i in 0..n {
        let pad_with = if i & 1 == 1 {pad_with_odd} else {pad_with_even};
        let (iptr, bptr) = enhance(image_ptr, buffer_ptr, decoder, pad_with);
        image_ptr = iptr;
        buffer_ptr = bptr;
    }
    if cfg!(test) {print_image(&image_ptr)};

    let ans = image_ptr.iter().map(|line|line.iter().filter(|bit|**bit == 1).count()).sum();
    ans
    // 5708 is too high
    // 5661 is too low 
    // 5580 is just right
}

#[aoc(day20, part2)]
pub fn part2(input: &(VecDeque<VecDeque<u8>>,Vec<u8>)) -> usize {
    let decoder = &input.1;
    assert_eq!(decoder.len(), 512);
    let mut image = input.0.clone();

    // Add capacity for the future
    // This may be overkill in Part 1
    for v in &mut image {
        v.reserve(1000);
    }
    image.reserve(1000);

    // Need a 2nd copy for efficient algo
    let mut buffer = image.clone(); // Does this clone capacity, too?  Hope so!

    let mut image_ptr = &mut image;
    let mut buffer_ptr = &mut buffer;

    let (pad_with_even, pad_with_odd) = match (decoder[0], decoder[decoder.len()-1]) {
        (0,0) => (0,0),
        (0,1) => (0,0), // Since we start with zeros... and they never change
        (1,0) => (0,1), // toggle
        (1,1) => (1,1), // only initial iteration is 0, all others are 1
        _ => panic!("Should be dealing only with bits here"),
    };
    // let mut image_ptr;
    // let mut buffer_ptr;
    // execute algo n times
    let n = 50;
    for i in 0..n {
        let pad_with = if i & 1 == 1 {pad_with_odd} else {pad_with_even};
        let (iptr, bptr) = enhance(image_ptr, buffer_ptr, decoder, pad_with);
        image_ptr = iptr;
        buffer_ptr = bptr;
    }
    if cfg!(test) {print_image(&image_ptr)};

    let ans = image_ptr.iter().map(|line|line.iter().filter(|bit|**bit == 1).count()).sum();
    ans
}

// Surround image with bit specified, either 1 or 0
fn pad_image(image: &mut VecDeque<VecDeque<u8>>, pad_with: u8) {
    assert!(pad_with <= 1);
    for line in image.iter_mut() {
        line.push_front(pad_with);
        line.push_back(pad_with);
    }
    // Create whole row of padding:
    let padding_row = image[0].iter().map(|_|pad_with).collect::<VecDeque<u8>>();
    image.push_front(padding_row.clone());
    image.push_back(padding_row.clone());
}

// Remove remnants of prior padding
fn trim_padding(image: &mut VecDeque<VecDeque<u8>>) {
    // trim beginning and end of rows
    for line in image.iter_mut() {
        line.pop_front();
        line.pop_back();
    }
    // trim top and bottom
    image.pop_front();
    image.pop_back();
}

fn print_image(image: &VecDeque<VecDeque<u8>>) {
    for row in image.iter() {
        let line = row.iter()
        .map(|bit| bit2char(*bit).unwrap())
        .collect::<String>();
        println!("     {}", line); 
    }
    println!();
}
pub fn bit2char(bit: u8) -> Result<char,&'static str> {
    match bit {
        0 => Ok('.'),
        1 => Ok('#'),
        _ => Err("Unexpected bit (??) found")
    }
}
pub fn char2bit(c: char) -> Result<u8,&'static str> {
    match c {
        '.' => Ok(0),
        '#' => Ok(1),
        _ => Err("Unexpected char found")
    }
}

fn enhance<'a>(image_ptr: &'a mut VecDeque<VecDeque<u8>>, buffer_ptr: &'a mut VecDeque<VecDeque<u8>>, decoder: &[u8], pad_with: u8) -> (&'a mut VecDeque<VecDeque<u8>>, &'a mut VecDeque<VecDeque<u8>>) {
    pad_image(image_ptr, pad_with);
    pad_image(image_ptr, pad_with);
    pad_image(buffer_ptr, pad_with);
    pad_image(buffer_ptr, pad_with);
    assert_eq!(image_ptr.len(), buffer_ptr.len());
    // Read from image, write to buffer, swap on return at end
    // We are trying to read a 3x3 grid and write to a single bit in buffer
    let image_x3: Vec<Vec<u8>> = image_ptr.iter().map(|line| {
        line.iter()
        .zip(line.iter().skip(1))
        .zip(line.iter().skip(2))
        .map(|((a,b),c)|(a<<2)+(b<<1)+c)
        // line is now 3-bit u8 instead of 1-bit u8
        .collect::<Vec<u8>>()
        }).collect::<Vec<_>>();
    image_x3.iter()
    .zip(image_x3[1..].iter())
    .zip(image_x3[2..].iter())
    .zip(buffer_ptr.iter_mut().skip(1))
    .for_each(|(((abc_row,def_row),ghi_row),target_pixel_row)| {
        abc_row.iter()
        .zip(def_row.iter())
        .zip(ghi_row.iter())
        .zip(target_pixel_row.iter_mut().skip(1))
        .for_each(|(((abc,def),ghi),target_pixel)| {
            let abcdefghi = ((*abc as usize)<<6)+((*def as usize)<<3)+(*ghi as usize);
            *target_pixel = decoder[abcdefghi];
        });
    });
    trim_padding(image_ptr);
    trim_padding(buffer_ptr);
    (buffer_ptr, image_ptr)
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        let (image, decode) = gen1(EX1);
        assert_eq!(decode.len(), 512);
        assert_eq!(image.len(), 5);
    }

    #[test]
    fn test_ex1_part1() {
        let g = gen1(EX1);
        let p1 = part1(&g);
        assert_eq!(p1, 35);
    }

    #[test]
    fn test_ex1_part2() {
        let g = gen1(EX1);
        let p2 = part2(&g);
        assert_eq!(p2, 3351);
    }

const EX1: &'static str =
r"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

}