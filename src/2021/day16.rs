/// https://adventofcode.com/2021/day/16
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 
use self::{Operator::*};

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day16)]
pub fn gen1(input: &str) -> Packet {
    Packet::parse_hex(input)
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day16, part1)]
pub fn part1(input: &Packet) -> usize {
    input.flatten().iter().map(|p|p.version as usize).sum()
}

#[aoc(day16, part2)]
pub fn part2(input: &Packet) -> u64 {
    let ans = input.eval();
    if !cfg!(test) {
        assert!(ans != 27588);
        assert!(ans > 27588);
    }
    ans
}

#[derive(Eq,PartialEq,Copy,Clone,Debug)]
pub enum Operator {
    Literal = 4,
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7,
    OpUnknown = isize::MAX,
}
impl Operator {
    pub fn parse_u8(v: u8) -> Self {
        match v {
            0 => Sum,
            1 => Product,
            2 => Minimum,
            3 => Maximum,
            4 => Literal,
            5 => GreaterThan,
            6 => LessThan,
            7 => EqualTo,
            _ => OpUnknown,
        }
    }
}
pub struct Packet {
    version: u8,
    typeid: Operator,
    value: Option<u64>,
    packets: Vec<Packet>,
}
impl Packet {
    pub fn eval(&self) -> u64 {
        match self.typeid {
            Literal => self.value.unwrap(),
            Sum => self.packets.iter().fold(0,|sum,p|sum+p.eval()),
            Product => self.packets.iter().fold(1,|product,p|product*p.eval()),
            Minimum => self.packets.iter().map(|p|p.eval()).min().unwrap(),
            Maximum => self.packets.iter().map(|p|p.eval()).max().unwrap(),
            GreaterThan => if self.packets[0].eval()
                            > self.packets[1].eval() {1} else {0},
            LessThan => if self.packets[0].eval()
                         < self.packets[1].eval() {1} else {0},
            EqualTo => if self.packets[0].eval()
                       == self.packets[1].eval() {1} else {0},
            OpUnknown => panic!("Opcode 'OpUnknown' encountered!"),                }
    }
    pub fn is_literal(&self) -> bool {
        self.typeid == Literal && self.value.is_some()
    }
    pub fn is_leaf_node(&self) -> bool {
        self.packets.len() == 0
    }
    fn hex2u8(hex_digit: char) -> u8 {
        match hex_digit {
            d if d>='0' && d<='9' => d as u8 - '0' as u8,
            h if h>='A' && h<='F' => h as u8 - 'A' as u8 + 10,
            _ => panic!("Not a Hex digit"),
        }
    }
    fn u8_to_u64(nibbles: &[u8]) -> u64 {
        let padlen = 16-nibbles.len();
        let mut val_u64 = nibbles.iter()
        .fold(0,|val,&nib|val*16+nib as u64);
        // pad with zeros to 16 nibbles
        for _ in 0..padlen {
            val_u64 *= 16;
        }
        val_u64
    }
    pub fn parse_hex(hex_str:&str) -> Self {
        let v_char: Vec<u8> = hex_str.chars()
        .map(|h|Self::hex2u8(h)).collect();
        let mut v_u64: Vec<u64> = v_char.chunks(16)
        .map(|chunk|Self::u8_to_u64(&chunk.iter().map(|ptr|*ptr).collect::<Vec<u8>>()))
        .collect();
        if cfg!(test) {println!("{:?}",v_u64)};
        Self::parse_u64s(&mut v_u64)
    }
    fn parse_u64s(mut v: &mut Vec<u64>) -> Packet {
        let version = Self::left_shift(&mut v, 3) as u8;
        let typeidnum = Self::left_shift(&mut v, 3) as u8;
        let typeid = Operator::parse_u8(typeidnum);
        match typeid {
            Literal => {
                let mut val = 0;
                loop {
                    val *= 16;
                    let is_last = Self::left_shift(&mut v, 1) == 0;
                    let nibble = Self::left_shift(&mut v, 4);
                    val += nibble;
                    if is_last {break;}
                };
                Packet {version, typeid, value: Some(val), packets: Vec::new()}
            },
            _ => {
                let mut packets: Vec<Packet> = Vec::new();
                let length_type_id = Self::left_shift(&mut v, 1);
                match length_type_id {
                    0 => {
                        let total_bit_length = Self::left_shift(&mut v, 15);
                        let word_cnt = total_bit_length / 64;
                        let remainder_bit_length = total_bit_length % 64;
                        let mut vv = Vec::new();
                        if word_cnt > 0 {
                            println!("bits: {} words: {} remainder: {}", total_bit_length, word_cnt, remainder_bit_length);
                            v.reverse();
                            for _ in 0..word_cnt {
                                vv.push(v.pop().unwrap());
                            }
                            v.reverse();
                        }
                        if remainder_bit_length > 0 {
                            let remainder_bits = Self::left_shift(&mut v, remainder_bit_length);
                            let remainder_bits = remainder_bits<<(64-remainder_bit_length); // MSB justified
                            vv.push(remainder_bits);
                        }
                        loop {
                            packets.push(Self::parse_u64s(&mut vv));
                            if vv.is_empty() {break;}
                            if vv[0] == 0 {break;}
                        }
                    },
                    1 => {
                        let sub_packet_cnt = Self::left_shift(&mut v, 11);
                        for _sp in 0..sub_packet_cnt {
                            packets.push(Self::parse_u64s(&mut v));
                            if v.is_empty() {break;}
                            if v[0] == 0 {break;}
                        }
                    },
                    _ => panic!("This can't happen with 1 bit")
                }
                // clean up remainder of input vector
                loop {
                    if v.is_empty() {break;}
                    if v[v.len()-1] > 0 {break;}
                    v.pop(); // Discard trailing word full of zeros
                }
                Packet {version, typeid, value: None, packets}
            }

        }
    }
    fn left_shift(v: &mut Vec<u64>, bit_cnt: u64) -> u64 {
        assert!(bit_cnt > 0);
        assert!(bit_cnt < 64);
        v.reverse(); // LSWord first
        let shifted_out_bits = v.iter_mut()
        .fold(0,|carry_bits, word|{
            let shifted = *word >> (64-bit_cnt);
            *word = (*word << bit_cnt) + carry_bits;
            shifted
        });
        v.reverse();
        shifted_out_bits
    }
}
impl<'a> Packet {
    pub fn flatten(&'a self) -> Vec<&'a Self> {
        let mut vecp = vec![self];
        for p in &self.packets {
            vecp.extend_from_slice(&p.flatten());
        }
        vecp
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
        assert_eq!(g.version, 6);
        assert_eq!(g.typeid, Literal);
        assert_eq!(g.typeid as u8, 4);
    }

    #[test]
    fn test_ex1_part1() {
        let g = gen1(EX1);
        let p1 = part1(&g);
        assert_eq!(p1, 6);
    }
    #[test]
    fn test_ex2_part1() {
        let g = gen1(EX2);
        let p1 = part1(&g);
        assert_eq!(p1, 9);
    }
    #[test]
    fn test_ex3_part1() {
        let g = gen1(EX3);
        let p1 = part1(&g);
        assert_eq!(p1, 14);
    }
    #[test]
    fn test_ex4_part1() {
        let g = gen1(EX4);
        let p1 = part1(&g);
        assert_eq!(p1, 16);
    }
    #[test]
    fn test_ex5_part1() {
        let g = gen1(EX5);
        let p1 = part1(&g);
        assert_eq!(p1, 12);
    }
    #[test]
    fn test_ex6_part1() {
        let g = gen1(EX6);
        let p1 = part1(&g);
        assert_eq!(p1, 23);
    }
    #[test]
    fn test_ex7_part1() {
        let g = gen1(EX7);
        let p1 = part1(&g);
        assert_eq!(p1, 31);
    }
    // Part2
    #[test]
    fn test_sum_part2() {
        let g = gen1("C200B40A82");
        let p2 = part2(&g);
        assert_eq!(p2, 3);
    }
    #[test]
    fn test_product_part2() {
        let g = gen1("04005AC33890");
        let p2 = part2(&g);
        assert_eq!(p2, 54);
    }
    #[test]
    fn test_minof3_part2() {
        let g = gen1("880086C3E88112");
        let p2 = part2(&g);
        assert_eq!(p2, 7);
    }
    #[test]
    fn test_maxof3_part2() {
        let g = gen1("CE00C43D881120");
        let p2 = part2(&g);
        assert_eq!(p2, 9);
    }
    #[test]
    fn test_lessthan_part2() {
        let g = gen1("D8005AC2A8F0");
        let p2 = part2(&g);
        assert_eq!(p2, 1);
    }
    #[test]
    fn test_greaterthan_part2() {
        let g = gen1("F600BC2D8F");
        let p2 = part2(&g);
        assert_eq!(p2, 0);
    }
    #[test]
    fn test_equalto_part2() {
        let g = gen1("9C005AC2F8F0");
        let p2 = part2(&g);
        assert_eq!(p2, 0);
    }
    #[test]
    fn test_expr_part2() {
        let g = gen1("9C0141080250320F1802104A08");
        let p2 = part2(&g);
        assert_eq!(p2, 1);
    }

// 110100101111111000101000
// VVVTTTAAAAABBBBBCCCCC
const EX1: &'static str =
r"D2FE28";
const EX2: &'static str =
r"38006F45291200";
const EX3: &'static str =
r"EE00D40C823060";
const EX4: &'static str =
r"8A004A801A8002F478";
const EX5: &'static str =
r"620080001611562C8802118E34";
const EX6: &'static str =
r"C0015000016115A2E0802F182340";
const EX7: &'static str =
r"A0016C880162017C3686B18A3D4780";

}