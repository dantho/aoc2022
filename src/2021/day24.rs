/// https://adventofcode.com/2021/day/24
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 
use self::Param::*;
use self::Op::*;
use std::convert::From;

/*
The ALU is a four-dimensional processing unit: it has integer variables w, x, y, and z. These variables all start with the value 0. The ALU also supports six instructions:

    inp a - Read an input value and write it to variable a.
    add a b - Add the value of a to the value of b, then store the result in variable a.
    mul a b - Multiply the value of a by the value of b, then store the result in variable a.
    div a b - Divide the value of a by the value of b, truncate the result to an integer, then store the result in variable a. (Here, "truncate" means to round the value toward zero.)
    mod a b - Divide the value of a by the value of b, then store the remainder in variable a. (This is also called the modulo operation.)
    eql a b - If the value of a and b are equal, then store the value 1 in variable a. Otherwise, store the value 0 in variable a.
*/
// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day24)]
pub fn gen1(input: &str) -> String {
    input.to_string()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day24, part1)]
pub fn part1(input: &str) -> isize {
    let mut alu = ALU::load_program(&input);  //..436 is 3, ..873 is 6 inputs
    // for n14 in 111..=999 { // 99_999_999_999_999
    for n14 in 99_999_999_969_999..=99_999_999_969_999 { // 99_999_999_999_999
        let digits = make_digits(n14);
        if digits.contains(&0) {continue};
        let regs = alu.eval(&digits);
        alu.reboot();
        {println!("{} yields {:?}",n14, regs)};
    }
    0
}

fn make_digits(mut n:isize) -> Vec<isize> {
    let mut digits = Vec::new();
    while n > 0 {
        digits.push(n-(n/10)*10);
        n = n/10;
    }
    digits.reverse();
    digits
}

#[aoc(day24, part2)]
pub fn part2(_input: &str) -> isize {
    0
}

#[derive(Copy,Clone)]
enum Param {
    Val(isize),
    Reg(usize),
}

#[derive(Copy,Clone)]
enum Op {
    Inp(usize,Option<Param>),
    Add(usize,Param),
    Mul(usize,Param),
    Div(usize,Param),
    Mod(usize,Param),
    Eql(usize,Param),
}
impl From<&str> for Op {
    fn from(input: &str) -> Self {
        let mut tokens = input.split(" ");
        let op_token = tokens.next().unwrap();
        let reg_token = tokens.next().unwrap();
        let param_token = tokens.next();
        let reg = match reg_token {
            "w" => 0,
            "x" => 1,
            "y" => 2,
            "z" => 3,
            _ => panic!("2nd token in string should be wxyz"),
        };
        let param = match param_token {
            None => None,
            Some("w") => Some(Reg(0)),
            Some("x") => Some(Reg(1)),
            Some("y") => Some(Reg(2)),
            Some("z") => Some(Reg(3)),
            val_str => Some(Val(val_str.unwrap().parse().unwrap())),
        };
        match op_token {
            "inp" => Inp(reg, param),
            "add" => Add(reg, param.unwrap()),
            "mul" => Mul(reg, param.unwrap()),
            "div" => Div(reg, param.unwrap()),
            "mod" => Mod(reg, param.unwrap()),
            "eql" => Eql(reg, param.unwrap()),
            _ => panic!("Unrecognized OpCode"),
        }
    }
}

struct ALU {
    regs: [isize;4],  // w, x, y, z
    program: Vec<Op>,
}

impl ALU {
    pub fn eval(&mut self, inputs: &[isize]) -> [isize;4] {
        self.distribute_inputs(inputs);
        for instruction in &self.program {
            match *instruction {
                Inp(reg,maybe_val) => {
                    if let Some(Val(v)) = maybe_val {
                        self.regs[reg] = v;
                    } else {
                        panic!("Missing input!");
                    }
                },
                Add(reg,b) => self.regs[reg] += match b {
                    Reg(ndx) => self.regs[ndx],
                    Val(v) => v,
                },
                Mul(reg,b) => self.regs[reg] *= match b {
                    Reg(ndx) => self.regs[ndx],
                    Val(v) => v,
                },
                Div(reg,b) => self.regs[reg] /= match b {
                    Reg(ndx) => self.regs[ndx],
                    Val(v) => v,
                },
                Mod(reg,b) => self.regs[reg] %= match b {
                    Reg(ndx) => self.regs[ndx],
                    Val(v) => v,
                },
                Eql(reg,b) => {
                    let b: isize = match b {
                        Reg(ndx) => {
                            // println!("b is regs[{}] is {}", ndx, self.regs[ndx]);
                            self.regs[ndx]
                        },
                        Val(v) => v,
                    };
                    // println!("regs[{}] is {}, b is {}", reg, self.regs[reg],b);
                    self.regs[reg] = if self.regs[reg] == b {1} else {0};
                },
            }
        }
        self.regs // Regs array is return value at end
    }
    #[allow(dead_code)]
    fn reboot(&mut self) {
        self.regs = [0,0,0,0];
        for op in self.program.iter_mut() {
            if let Inp(reg,_) = op {*op = Inp(*reg, None)};
        }
    }
    fn load_program(program: &str) -> Self {
        let regs = [0,0,0,0];
        let program = program.lines()
        .filter(|line|!line.is_empty())
        .map(|line|{
                Op::from(line)
            }).collect::<Vec<Op>>();
        ALU {regs, program}     
    }
    fn distribute_inputs(&mut self, inputs: &[isize]) {
        let input_ops: Vec<&mut Op> = self.program.iter_mut()
        .filter(|op| match op {
            Inp(_, None) => true,
            Inp(_, Some(_)) => panic!("Overwriting prior input"),
            _ => false,
        }).collect();
        if inputs.len() == input_ops.len() {
            input_ops.into_iter()
            .zip(inputs.iter())
            .for_each(|(inp_instr, p)|{
                if let &mut Inp(reg, None) = inp_instr {
                    // println!("Input {}", *p);
                    *inp_instr = Inp(reg,Some(Val(*p)));
                } else {
                    panic!("Shouldn't be here due to filter, above");
                }
            });
        } else {
            eprintln!("Program requires {} inputs, but {} inputs were provided", input_ops.len(), inputs.len());
            panic!("Program input error");
        }
    } 
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1() {
        let mut alu = ALU::load_program(EX1);
        assert_eq!(alu.eval(&[4,12]), [0,12,0,1]);
    }

    #[test]
    fn test_ex2() {
        let mut alu = ALU::load_program(EX2);
        assert_eq!(alu.eval(&[15]), [1,1,1,1]);
        alu.reboot();
        assert_eq!(alu.eval(&[10]), [1,0,1,0]);
        alu.reboot();
        assert_eq!(alu.eval(&[5]), [0,1,0,1]);
        alu.reboot();
        assert_eq!(alu.eval(&[0]), [0,0,0,0]);
    }

const EX1: &'static str =
r"inp z
inp x
mul z 3
eql z x";

const EX2: &'static str =
r"inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2
";

}