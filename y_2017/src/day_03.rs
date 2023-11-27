use crate::lib::parse_input;

enum Direction {
    Right,
    Up,
    Left,
    Down
}

pub fn part_1() -> i32 {
    let input = parse_input("inputs/day_03.txt")[0].parse::<i32>().unwrap();
    let grid = generate_spiral_grid(input, false);
    calculate_manhattan_distance(&grid, input)
}

pub fn part_2() -> i32 {
    let input = parse_input("inputs/day_03.txt")[0].parse::<i32>().unwrap();
    let grid = generate_spiral_grid(input, true );
    let mut result = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] > input {
                result = grid[i][j];
            }
        }
    }
    result
}

fn generate_spiral_grid(input: i32, fill_adjacent: bool) -> Vec<Vec<i32>> {
    let width = (input as f32).sqrt().ceil() as i32;
    let height = width;
    let mut grid = vec![vec![0; width as usize]; width as usize];
    let mut x = width / 2;
    let mut y = height / 2;
    let mut direction = Direction::Right;
    let mut current = 1;
    let mut steps = 1;
    let mut step_count = 0;
    grid[y as usize][x as usize] = current;
    current += 1;
    while current <= input && grid[y as usize][x as usize] < input {
        match direction {
            Direction::Right => {
                x += 1;
                grid[y as usize][x as usize] = match fill_adjacent {
                    true => calculate_adjacent_sum(width, height, &mut grid, x, y),
                    false => current
                };
                step_count += 1;
                if step_count == steps {
                    direction = Direction::Up;
                    step_count = 0;
                }
            },
            Direction::Up => {
                y -= 1;
                grid[y as usize][x as usize] = match fill_adjacent {
                    true => calculate_adjacent_sum(width, height, &mut grid, x, y),
                    false => current
                };
                step_count += 1;
                if step_count == steps {
                    direction = Direction::Left;
                    step_count = 0;
                    steps += 1;
                }
            },
            Direction::Left => {
                x -= 1;
                grid[y as usize][x as usize] = match fill_adjacent {
                    true => calculate_adjacent_sum(width, height, &mut grid, x, y),
                    false => current
                };
                step_count += 1;
                if step_count == steps {
                    direction = Direction::Down;
                    step_count = 0;
                }
            },
            Direction::Down => {
                y += 1;
                grid[y as usize][x as usize] = match fill_adjacent {
                    true => calculate_adjacent_sum(width, height, &mut grid, x, y),
                    false => current
                };
                step_count += 1;
                if step_count == steps {
                    direction = Direction::Right;
                    step_count = 0;
                    steps += 1;
                }
            }
        }
        current += 1;
    }
    grid
}

fn calculate_adjacent_sum(width: i32, height: i32, grid: &Vec<Vec<i32>>, x: i32, y: i32) -> i32 {
    let mut sum: i32 = 0;
    if x > 0 { sum += grid[y as usize][x as usize - 1]; }
    if x < width - 1 { sum += grid[y as usize][x as usize + 1]; }
    if y > 0 { sum += grid[y as usize - 1][x as usize]; }
    if y < height - 1 { sum += grid[y as usize + 1][x as usize]; }
    if x > 0 && y > 0 { sum += grid[y as usize - 1][x as usize - 1] ; }
    if x > 0 && y < height - 1 { sum += grid[y as usize + 1][x as usize - 1]; }
    if x < width - 1 && y > 0 { sum += grid[y as usize - 1][x as usize + 1]; }
    if x < width - 1 && y < height - 1 { sum += grid[y as usize + 1][x as usize + 1]; }
    sum
}

fn calculate_manhattan_distance(grid: &Vec<Vec<i32>>, target: i32) -> i32 {
    let mut x = 0;
    let mut y = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == target {
                x = j;
                y = i;
            }
        }
    }
    let center = grid.len() / 2;
    let x_diff = (x as i32 - center as i32).abs();
    let y_diff = (y as i32 - center as i32).abs();
    x_diff + y_diff
}


// tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let grid = generate_spiral_grid(2000, false);
        assert_eq!(calculate_manhattan_distance(&grid, 1), 0);
        assert_eq!(calculate_manhattan_distance(&grid, 12), 3);
        assert_eq!(calculate_manhattan_distance(&grid, 23), 2);
        assert_eq!(calculate_manhattan_distance(&grid, 1024), 31);
    }

    #[test]
    fn test_part_2() {
        let grid = generate_spiral_grid(25, true);
        assert_eq!(grid, vec![
            vec![147, 142, 133, 122, 59],
            vec![304, 5, 4, 2, 57],
            vec![330, 10, 1, 1, 54],
            vec![351, 11, 23, 25, 26],
            vec![362, 747, 806, 880, 931]
        ]);
    }
}