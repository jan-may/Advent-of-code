use crate::lib::parse_input;

pub fn part_1() -> i32 {
    let mut grid = generate_grid_from_input();
    for _ in 0..100 {
        grid = update_grid(&grid);
    }
    return sum_active_lights(&mut grid)
}

pub fn part_2() -> i32 {
    let mut grid = generate_grid_from_input();
    set_corners_active(&mut grid);

    for _ in 0..100 {
        grid = step(&grid);
        set_corners_active(&mut grid);
    }
    return sum_active_lights(&mut grid)
}

fn sum_active_lights(grid: &mut Vec<Vec<char>>) -> i32 {
    grid.iter()
        .flatten()
        .filter(|&&c| c == '#')
        .count() as i32
}

fn generate_grid_from_input() -> Vec<Vec<char>> {
    let grid = parse_input("inputs/day_18.txt")
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    grid
}

fn update_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let height = grid.len();
    let width = grid[0].len();

    let mut new_grid = vec![vec!['.'; width]; height];

    for y in 0..height {
        for x in 0..width {
            let count = count_neighbors_on(grid, y, x);
            new_grid[y][x] = match (grid[y][x], count) {
                ('#', 2) | ('#', 3) => '#',
                ('.', 3) => '#',
                _ => '.',
            };
        }
    }

    new_grid
}

fn count_neighbors_on(grid: &Vec<Vec<char>>, y: usize, x: usize) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;
    for &y2 in &[y.wrapping_sub(1), y, y + 1] {
        for &x2 in &[x.wrapping_sub(1), x, x + 1] {
            if y2 == y && x2 == x {
                continue;
            }
            if y2 < height && x2 < width && grid[y2][x2] == '#' {
                count += 1;
            }
        }
    }

    count
}

fn set_corners_active(grid: &mut Vec<Vec<char>>) {
    let height = grid.len();
    let width = grid[0].len();
    grid[0][0] = '#';
    grid[0][width - 1] = '#';
    grid[height - 1][0] = '#';
    grid[height - 1][width - 1] = '#';
}

fn step(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let new_grid = update_grid(grid);
    new_grid
}
