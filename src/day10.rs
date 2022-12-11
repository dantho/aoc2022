/// https://adventofcode.com/2022/day/10
/// DAN: https://adventofcode.com/2022/leaderboard/private/view/380786
/// TER: https://adventofcode.com/2022/leaderboard/private/view/951754
use self::Op::*;
use std::{convert::{From, TryInto}, fmt::Display, fmt::Formatter};
/*
Start by figuring out the signal being sent by the CPU. The CPU has a single register, X, which starts with the value 1. It supports only two instructions:

addx V takes two cycles to complete. After two cycles, the X register is increased by the value V. (V can be negative.)
noop takes one cycle to complete. It has no other effect.
*/
// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day10)]
pub fn gen1(input: &str) -> String {
    input.to_string()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day10, part1)]
pub fn part1(input: &str) -> isize {
    let mut alu = ALU::load_program(&input);
    alu.eval();
    alu.log.iter().map(|(_,v)|v).sum()
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> isize {
    let mut alu = ALU::load_program(&input);
    alu.eval();
    println!("{}", alu); // Prints pixels
    0
}

// ***************
// *** The ALU ***
// ***************
#[derive(Copy,Clone)]
enum Op {
    Noop,
    Addx(isize),
}

impl From<&str> for Op {
    fn from(input: &str) -> Self {
        let mut tokens = input.split(" ");
        let op_token = tokens.next().unwrap();
        let val_token_maybe = tokens.next();
        match op_token {
            "noop" => Noop,
            "addx" => {
                let val = val_token_maybe.unwrap().parse().unwrap();
                Addx(val)
            },
            bad => panic!("Unrecognized OpCode: '{}'", bad),
        }
    }
}

const REG_COUNT: usize = 1;
const PIX_COUNT: usize = 240;
const PIX_WIDTH: usize = 40;
struct ALU {
    clock: usize,
    regs: [isize;REG_COUNT],  // x
    program: Vec<Op>,
    log: Vec<(usize,isize)>,
    pixels: [bool; PIX_COUNT],
}

impl ALU {
    fn reboot(&mut self) {
        self.clock = 0;
        self.regs = [1;REG_COUNT];
        self.log.clear();
        self.pixels = [false; PIX_COUNT];
        // leave program intact
    }

    fn clock_tic(&mut self) {
        self.clock += 1;
        // Part 1 monitor
        self.monitor();
        // Part 2 pixels
        let clk_ndx = self.clock - 1;
        assert!(clk_ndx < PIX_COUNT);
        let pos: isize = (clk_ndx % PIX_WIDTH).try_into().unwrap();
        let x = self.regs[0];
        self.pixels[clk_ndx] = pos >= x-1 && pos <= x+1;
    }

    fn clock_tic_n(&mut self, n: usize) {
        for _i in 0..n {self.clock_tic()};
    }

    fn monitor(&mut self) {
        // Monitor algo is hardcoded -- could be stored as a closure
        if self.clock >= 20 && (self.clock - 20) % 40 == 0 {
            let clock_as_isize: isize = self.clock.try_into().unwrap();
            self.log.push((self.clock, clock_as_isize * self.regs[0]));
        }
    }

    pub fn eval(&mut self) -> [isize;REG_COUNT] {
        for instruction in self.program.clone() {
            match instruction {
                Noop => {
                    self.clock_tic();
                },
                Addx(v) => {
                    self.clock_tic_n(2);
                    self.regs[0] += v;
                },
            };
        }
        self.regs // Regs array is return value at end
    }

    fn load_program(program: &str) -> Self {
        let clock = 999; // See reboot for proper initialization
        let regs = [999;REG_COUNT]; // See reboot for proper initialization
        let log = Vec::new();
        let pixels = [false;PIX_COUNT];
        let program = program.lines()
        .filter(|line|!line.is_empty())
        .map(|line|{
                Op::from(line)
            }).collect::<Vec<Op>>();
        let mut new_alu = ALU {clock, regs, program, pixels, log};
        new_alu.reboot();
        new_alu
    }
    pub fn pixels_str(&self) -> String {
        let mut out_string = String::new();
        for p in 0..PIX_COUNT {
            if p % PIX_WIDTH == 0 {
                out_string.push('\n');
            }
            out_string.push(if self.pixels[p] {'#'} else {'.'});
        }
        out_string
    }
}

impl Display for ALU {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f,"{}", self.pixels_str())?;
        Ok(())
    }
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let mut alu = ALU::load_program(EX1);
        assert_eq!(alu.eval(), [-1]);
        // part 1
        assert_eq!(part1(&gen1(EX1)), 0);
    }

    #[test]
    fn test_part1_ex2() {
        assert_eq!(part1(&gen1(EX2)), 13140);
    }

    #[test]
    fn test_part1_detail_ex2() {
        let mut alu = ALU::load_program(EX2);
        alu.eval();
        let mut log = alu.log.iter();
        assert_eq!(log.next(),Some(&(20, 420)));
        assert_eq!(log.next(),Some(&(60, 1140)));
        assert_eq!(log.next(),Some(&(100, 1800)));
        assert_eq!(log.next(),Some(&(140, 2940)));
        assert_eq!(log.next(),Some(&(180, 2880)));
        assert_eq!(log.next(),Some(&(220, 3960)));
    }

    #[test]
    fn test_part2_ex2() {
        assert_eq!(part2(&gen1(EX2)), 0);
    }

const EX1: &'static str =
r"noop
addx 3
addx -5";

const EX2: &'static str =
r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

}