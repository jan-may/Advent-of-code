use crate::lib::parse_input;

#[derive(Debug)]
enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

pub fn part_1_and_2() -> usize {
    let input = parse_input("inputs/day_08.txt");
    let mut grid = [[false; 50]; 6];
    let instructions = parse_instructions(&input);
    for inst in instructions {
        match inst {
            Instruction::Rect(width, height) => {
                for row in grid.iter_mut().take(height) {
                    for item in row.iter_mut().take(width) {
                        *item = true;
                    }
                }
            },
            Instruction::RotateRow(row, amount) => {
                grid[row].rotate_right(amount);
            },
            Instruction::RotateColumn(col, amount) => {
                let mut temp_column: Vec<_> = grid.iter().map(|r| r[col]).collect();
                temp_column.rotate_right(amount);
                for (i, val) in temp_column.iter().enumerate() {
                    grid[i][col] = *val;
                }
            },
        }
    }
    let count = grid.iter().map(|row| row.iter().filter(|item| **item).count()).sum();
    // part2
    for row in grid.iter() {
        for item in row.iter() {
            print!("{}", if *item { '#' } else { '.' });
        }
        println!();
    }
    count
}

fn parse_instructions(input: &Vec<String>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::with_capacity(input.len());
    for line in input{
        let mut words = line.split_whitespace();
        match words.next().unwrap() {
            "rect" => {
                let mut dimensions = words.next().unwrap().split('x');
                let width = dimensions.next().unwrap().parse::<usize>().unwrap();
                let height = dimensions.next().unwrap().parse::<usize>().unwrap();
                instructions.push(Instruction::Rect(width, height));
            },
            "rotate" => {
                let dir = words.next().unwrap();
                let index = words.next().unwrap().split('=').nth(1).unwrap().parse::<usize>().unwrap();
                let _ = words.next().unwrap();
                let amount = words.next().unwrap().parse::<usize>().unwrap();
                match dir {
                    "row" => instructions.push(Instruction::RotateRow(index, amount)),
                    "column" => instructions.push(Instruction::RotateColumn(index, amount)),
                    _ => panic!("Invalid direction"),
                }
            },
            _ => panic!("Invalid instruction"),
        }
    }
    instructions
}