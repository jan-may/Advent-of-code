use crate::lib::parse_input;

#[derive(Debug)]
struct Instruction {
    action: String,
    start: (i32, i32),
    end: (i32, i32),
}

pub fn part_1() -> i32 {
    let mut data = parse_input("inputs/day_06.txt");
    let mut grid = vec![vec![false; 1000]; 1000];

    for line in data.iter() {
        let instruction = parse_instruction(line);

        for x in instruction.start.0..=instruction.end.0 {
            for y in instruction.start.1..=instruction.end.1 {
                match instruction.action.as_str() {
                    "on" => grid[x as usize][y as usize] = true,
                    "of" => grid[x as usize][y as usize] = false,
                    "toggle" => grid[x as usize][y as usize] = !grid[x as usize][y as usize],
                    _ => (),
                }
            }
        }
    }

    let count = grid.iter().flatten().filter(|&x| *x).count() as i32;
    count
}

pub fn part_2() -> i32 {
    let mut data = parse_input("inputs/day_06.txt");
    let mut grid = vec![vec![0; 1000]; 1000];

    for line in data.iter() {
        let instruction = parse_instruction(line);

        for x in instruction.start.0..=instruction.end.0 {
            for y in instruction.start.1..=instruction.end.1 {
                match instruction.action.as_str() {
                    "on" => grid[x as usize][y as usize] = grid[x as usize][y as usize] + 1,
                    "of" => grid[x as usize][y as usize] = if grid[x as usize][y as usize] > 0 { grid[x as usize][y as usize] - 1 } else { 0 },
                    "toggle" => grid[x as usize][y as usize] = grid[x as usize][y as usize] + 2,
                    _ => (),
                }
            }
        }
    }

    let sum = grid.iter().flatten().sum::<i32>();
    sum
}

fn parse_instruction(line: &str) -> Instruction {
    let line = line.replace("turn on", "on").replace("turn off", "of").replace(" through", "");
    let line = line.split(" ").collect::<Vec<&str>>();
    Instruction {
        action: line[0].to_string(),
        start: (line[1].split(",").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(), line[1].split(",").collect::<Vec<&str>>()[1].parse::<i32>().unwrap()),
        end: (line[2].split(",").collect::<Vec<&str>>()[0].parse::<i32>().unwrap(), line[2].split(",").collect::<Vec<&str>>()[1].parse::<i32>().unwrap()),
    }
}