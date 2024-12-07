use crate::lib::parse_input;

const INPUT_FILE: &str = "inputs/day_06.txt";
const START_SYMBOL: char = '^';
const OBSTACLE_SYMBOL: char = '#';

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn create_grid_from_input() -> Vec<Vec<char>> {
    let input = parse_input(INPUT_FILE);
    let grid = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    grid
}

fn get_starting_point(grid: &Vec<Vec<char>>) -> (i32, i32) {
    let mut starting_point = (0, 0);
    for line in grid {
        for (index, c) in line.iter().enumerate() {
            if *c == START_SYMBOL {
                starting_point = (
                    index as i32,
                    grid.iter().position(|x| x == line).unwrap() as i32,
                );
            }
        }
    }
    starting_point
}

fn rotate_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn get_next_position(direction: Direction, current_position: (i32, i32)) -> (i32, i32) {
    let next_position = match direction {
        Direction::Up => (current_position.0, current_position.1 - 1),
        Direction::Down => (current_position.0, current_position.1 + 1),
        Direction::Left => (current_position.0 - 1, current_position.1),
        Direction::Right => (current_position.0 + 1, current_position.1),
    };
    next_position
}

fn check_if_inside_grid(grid: &Vec<Vec<char>>, position: (i32, i32)) -> bool {
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    position.0 >= 0 && position.0 < width && position.1 >= 0 && position.1 < height
}

pub fn part_1() -> i32 {
    let grid = create_grid_from_input();
    let mut visited_grid = grid.clone();
    let starting_point = get_starting_point(&grid);
    let mut current_position = starting_point;
    let mut direction = Direction::Up;

    // Mark the starting point as visited
    visited_grid[starting_point.1 as usize][starting_point.0 as usize] = 'x';

    loop {
        let next_position = get_next_position(direction, current_position);

        // Check if the next position is outside the grid
        if !check_if_inside_grid(&grid, next_position) {
            break;
        }

        // Check if the next position is an obstacle
        if grid[next_position.1 as usize][next_position.0 as usize] == OBSTACLE_SYMBOL {
            direction = rotate_direction(direction); // Rotate the direction
        } else {
            // Move to the next position and mark it as visited
            visited_grid[next_position.1 as usize][next_position.0 as usize] = 'x';

            current_position = next_position;
        }
    }

    // Count the number of 'x' in visited_grid
    visited_grid.iter().flatten().filter(|&&c| c == 'x').count() as i32
}

use std::collections::HashSet;

pub fn part_2() -> i32 {
    // Count the number of new obstacles that force the guard into a loop
    let grid = create_grid_from_input();
    let starting_point: (i32, i32) = get_starting_point(&grid);
    let mut new_grid: Vec<Vec<char>> = grid.clone();
    let mut loop_count: i32 = 0;

    // Try replacing each valid cell with an obstacle
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == OBSTACLE_SYMBOL || grid[y][x] == START_SYMBOL {
                continue;
            }

            // modify the grid by replacing the cell with an obstacle
            // restore the cell after the simulation
            // no need to create a new grid each time
            new_grid[y][x] = OBSTACLE_SYMBOL;

            let mut current_position = starting_point;
            let mut direction = Direction::Up;
            let mut visited_states: HashSet<((i32, i32), Direction)> = HashSet::new();

            // Simulate guard movement
            loop {
                let state = (current_position, direction);

                // Check if the current state has already been visited
                if visited_states.contains(&state) {
                    loop_count += 1;
                    break;
                }
                visited_states.insert(state);

                let next_position = get_next_position(direction, current_position);

                if !check_if_inside_grid(&grid, next_position) {
                    break;
                }

                // Check if the next position is an obstacle
                if new_grid[next_position.1 as usize][next_position.0 as usize] == OBSTACLE_SYMBOL {
                    direction = rotate_direction(direction); // Rotate the direction
                } else {
                    current_position = next_position;
                }
            }
            // restore the grid state before moving to the next cell
            new_grid[y][x] = grid[y][x];
        }
    }
    loop_count
}
