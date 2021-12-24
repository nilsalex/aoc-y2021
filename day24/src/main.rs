use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

enum RegisterOrLiteral {
    Register(usize),
    Literal(isize),
}

impl RegisterOrLiteral {
    fn parse_register(str: &str) -> usize {
        match str {
            "x" => 0,
            "y" => 1,
            "z" => 2,
            "w" => 3,
            _ => unreachable!(),
        }
    }
    fn parse(str: &str) -> Self {
        match str {
            "x" => Self::Register(0),
            "y" => Self::Register(1),
            "z" => Self::Register(2),
            "w" => Self::Register(3),
            lit => Self::Literal(lit.parse().unwrap()),
        }
    }
}

enum Instruction {
    Inp(usize),
    Add(usize, RegisterOrLiteral),
    Mul(usize, RegisterOrLiteral),
    Div(usize, RegisterOrLiteral),
    Mod(usize, RegisterOrLiteral),
    Eql(usize, RegisterOrLiteral),
}

impl Instruction {
    fn parse(str: &str) -> Self {
        let split = str.split_whitespace().collect::<Vec<&str>>();
        match split[0] {
            "inp" => Self::Inp(RegisterOrLiteral::parse_register(split[1])),
            "add" => Self::Add(
                RegisterOrLiteral::parse_register(split[1]),
                RegisterOrLiteral::parse(split[2]),
            ),
            "mul" => Self::Mul(
                RegisterOrLiteral::parse_register(split[1]),
                RegisterOrLiteral::parse(split[2]),
            ),
            "div" => Self::Div(
                RegisterOrLiteral::parse_register(split[1]),
                RegisterOrLiteral::parse(split[2]),
            ),
            "mod" => Self::Mod(
                RegisterOrLiteral::parse_register(split[1]),
                RegisterOrLiteral::parse(split[2]),
            ),
            "eql" => Self::Eql(
                RegisterOrLiteral::parse_register(split[1]),
                RegisterOrLiteral::parse(split[2]),
            ),
            _ => unreachable!(),
        }
    }
    fn eval(&self, registers: &mut [isize], input: isize) {
        match self {
            Instruction::Inp(r) => registers[*r] = input,
            Self::Add(r, RegisterOrLiteral::Literal(l)) => registers[*r] += *l,
            Self::Add(r, RegisterOrLiteral::Register(r2)) => registers[*r] += registers[*r2],
            Self::Mul(r, RegisterOrLiteral::Literal(l)) => registers[*r] *= *l,
            Self::Mul(r, RegisterOrLiteral::Register(r2)) => registers[*r] *= registers[*r2],
            Self::Div(r, RegisterOrLiteral::Literal(l)) => registers[*r] /= *l,
            Self::Div(r, RegisterOrLiteral::Register(r2)) => registers[*r] /= registers[*r2],
            Self::Mod(r, RegisterOrLiteral::Literal(l)) => registers[*r as usize] %= *l,
            Self::Mod(r, RegisterOrLiteral::Register(r2)) => registers[*r] %= registers[*r2],
            Self::Eql(r, RegisterOrLiteral::Literal(l)) => {
                registers[*r] = Self::to_int(registers[*r] == *l)
            }
            Self::Eql(r, RegisterOrLiteral::Register(r2)) => {
                registers[*r] = Self::to_int(registers[*r] == registers[*r2])
            }
        }
    }
    fn to_int(val: bool) -> isize {
        if val {
            1
        } else {
            0
        }
    }
}

fn dfs(
    step: usize,
    register_z: isize,
    instructions: &[Instruction],
    instruction_pointer: usize,
    memo: &mut HashMap<(usize, isize), Option<usize>>,
    ascending: bool,
) -> Option<usize> {
    if step == 14 {
        if register_z == 0 {
            return Some(0);
        }
        return None;
    }

    if let Some(memoized) = memo.get(&(step, register_z)) {
        return *memoized;
    }

    memo.insert((step, register_z), None);

    let range = if ascending {
        [1, 2, 3, 4, 5, 6, 7, 8, 9]
    } else {
        [9, 8, 7, 6, 5, 4, 3, 2, 1]
    };

    for digit in range {
        let mut input_consumed: bool = false;
        let mut registers = vec![0, 0, register_z, 0];
        let mut next_instruction_pointer = instruction_pointer;
        for instruction in instructions.iter().skip(instruction_pointer) {
            if input_consumed && matches!(instruction, Instruction::Inp(_)) {
                break;
            }
            instruction.eval(&mut registers, digit);
            if matches!(instruction, Instruction::Inp(_)) {
                input_consumed = true;
            }
            next_instruction_pointer += 1;
        }
        if let Some(next) = dfs(
            step + 1,
            registers[2],
            instructions,
            next_instruction_pointer,
            memo,
            ascending,
        ) {
            let result = Some(next + digit as usize * 10_usize.pow(14 - 1 - step as u32));
            memo.insert((step, register_z), result);
            return result;
        } else {
            continue;
        }
    }

    *memo.get(&(step, register_z)).unwrap()
}

fn part1() -> usize {
    let file = File::open("day24/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    let instructions = lines
        .map(|l| Instruction::parse(&l))
        .collect::<Vec<Instruction>>();

    let mut memo: HashMap<(usize, isize), Option<usize>> = HashMap::new();
    dfs(0, 0, &instructions, 0, &mut memo, false).unwrap()
}

fn part2() -> usize {
    let file = File::open("day24/input.txt").unwrap();
    let lines = io::BufReader::new(file).lines().flatten();

    let instructions = lines
        .map(|l| Instruction::parse(&l))
        .collect::<Vec<Instruction>>();

    let mut memo: HashMap<(usize, isize), Option<usize>> = HashMap::new();
    dfs(0, 0, &instructions, 0, &mut memo, true).unwrap()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
