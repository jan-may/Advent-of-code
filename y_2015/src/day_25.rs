pub fn part_1() -> usize {
    calculate_value(20151125, 2978, 3083)
}

pub fn part_2() -> i32 {
    0
}

fn calculate_value(start: usize, row: usize, col: usize) -> usize {
    let mut current = start;
    let mut code_position = calculate_code_position(row, col);

    for _ in 1..code_position {
        current = (current * 252533) % 33554393;
    }

    current
}

fn calculate_code_position(row: usize, col: usize) -> usize {
    // The diagonal number is the sum of row and column indices minus 1
    let diagonal = row + col - 1;
    (diagonal * (diagonal - 1)) / 2 + col
}
