use crate::lib::parse_input;

pub fn part_1() -> i32 {
    let input = parse_input("inputs/day_10.txt")[0].clone();
    calculate(input, 40)
}

pub fn part_2() -> i32 {
    let input = parse_input("inputs/day_10.txt")[0].clone();
    calculate(input, 50)
}

fn calculate(input: String, iterations: usize) -> i32 {
    let mut input = input;
    for _ in 0..iterations {
        let mut output = String::new();
        let mut count = 1;
        let mut last = input.remove(0);
        for c in input.chars() {
            match c == last {
                true => count += 1,
                false => {
                    output.push_str(&format!("{}{}", count, last));
                    count = 1;
                    last = c;
                }
            }
        }
        output.push_str(&format!("{}{}", count, last));
        input = output.clone();
    }
    input.len() as i32
}

