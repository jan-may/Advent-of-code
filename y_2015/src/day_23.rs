use crate::lib::parse_input;

enum Register {
    A,
    B,
}

enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i32),
    Jie(Register, i32),
    Jio(Register, i32),
}

pub fn part_1() -> i32 {
    let input = parse_input("inputs/day_23.txt");
    let instructions = parse_instructions(&input);
    run_programm(instructions, 0,0)[1]
}

pub fn part_2() -> i32 {
    let input = parse_input("inputs/day_23.txt");
    let instructions = parse_instructions(&input);
    run_programm(instructions, 1,0)[1]
}

fn run_programm(instructions: Vec<Instruction>, a: i32, b: i32) -> [i32; 2] {
    let mut registers = [a, b];
    let mut pc = 0;
    while pc < instructions.len() {
        match &instructions[pc] {
            Instruction::Hlf(reg) | Instruction::Tpl(reg) | Instruction::Inc(reg) => {
                let reg_idx = match reg {
                    Register::A => 0,
                    Register::B => 1,
                };
                match instructions[pc] {
                    Instruction::Hlf(_) => registers[reg_idx] /= 2,
                    Instruction::Tpl(_) => registers[reg_idx] *= 3,
                    Instruction::Inc(_) => registers[reg_idx] += 1,
                    _ => {},
                }
                pc += 1;
            },
            Instruction::Jmp(offset) => {
                pc = (pc as i32 + offset) as usize;
            },
            Instruction::Jie(reg, offset) | Instruction::Jio(reg, offset) => {
                let reg_idx = match reg {
                    Register::A => 0,
                    Register::B => 1,
                };
                let condition = match instructions[pc] {
                    Instruction::Jie(_, _) => registers[reg_idx] % 2 == 0,
                    Instruction::Jio(_, _) => registers[reg_idx] == 1,
                    _ => false,
                };
                if condition {
                    pc = (pc as i32 + offset) as usize;
                } else {
                    pc += 1;
                }
            },
        }
    }
    registers
}

fn parse_instructions(input: &Vec<String>) -> Vec<Instruction> {
    input.iter().map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "hlf" => Instruction::Hlf(parse_register(parts[1])),
            "tpl" => Instruction::Tpl(parse_register(parts[1])),
            "inc" => Instruction::Inc(parse_register(parts[1])),
            "jmp" => Instruction::Jmp(parts[1].parse::<i32>().unwrap()),
            "jie" => Instruction::Jie(parse_register(parts[1].trim_end_matches(',')), parts[2].parse::<i32>().unwrap()),
            "jio" => Instruction::Jio(parse_register(parts[1].trim_end_matches(',')), parts[2].parse::<i32>().unwrap()),
            _ => panic!("Unknown instruction"),
        }
    }).collect()
}

fn parse_register(input: &str) -> Register {
    match input {
        "a" => Register::A,
        "b" => Register::B,
        _ => panic!("Unknown register"),
    }
}