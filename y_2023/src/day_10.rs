use crate::lib::parse_input;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Null,
}

pub fn part_1() -> i32 {
   let mut grid = generate_grid();
    let start = find_start_coords(&grid);
    let mut area = 0; // not used in part 1
    let mut current = start;
    let mut prev_direction = Direction::Left; // have to be filled manually depending on the input
    grid[start.0][start.1] = '7'; // have to be filled manually depending on the input

    (prev_direction, current) = check_next_pipe(&mut grid, &prev_direction, current, &mut area);

    let circ = calc_circ(&mut grid, start, current, &mut prev_direction, &mut area);

    circ / 2 + 1
}

pub fn part_2() -> i32 {
    let mut grid = generate_grid();
    let start = find_start_coords(&grid);
    let mut current = start;
    let mut area = 0;
    let mut direction = Direction::Left; // have to be filled manually depending on the input
    grid[start.0][start.1] = '7'; // have to be filled manually depending on the input

    (direction, current) = check_next_pipe(&mut grid, &direction, current, &mut area);

    let circ = calc_circ(&mut grid, start, current, &mut direction, &mut area);

    // based on Pick’s Theorem: A = i + b/2 - 1
    // where A is the area of the polygon, i is the number of interior points, and b is the number of boundary points
    // https://arxiv.org/ftp/arxiv/papers/1707/1707.04808.pdf
    (i32::abs(area) - circ) / 2 + 1
}

fn generate_grid() -> Vec<Vec<char>> {
    parse_input("inputs/day_10.txt").iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()
}

fn find_start_coords(grid: &[Vec<char>]) -> (usize, usize) {
    grid.iter().enumerate()
        .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, &c)| (y, x, c)))
        .find(|&(_, _, c)| c == 'S')
        .map(|(y, x, _)| (y, x))
        .unwrap_or((0, 0))
}

fn calc_circ(grid: &mut [Vec<char>], start: (usize, usize), mut current: (usize, usize), direction: &mut Direction, area: &mut i32) -> i32 {
    let mut c = 0;
    while current != start {
        (*direction, current) = check_next_pipe(grid, direction, current, area);
        c += 1;
    }
    c
}

fn check_next_pipe(grid: &mut [Vec<char>], direction: &Direction, current_pos: (usize,usize), area: &mut i32)-> (Direction, (usize,usize)) {
    let x = current_pos.1;
    let y = current_pos.0;
    let next_pos;
    let new_direction;
    match grid[y][x] {
        '|' => match direction {
            Direction::Down => { next_pos = (y - 1, x); new_direction = Direction::Down; *area -= x as i32},
            Direction::Up => { next_pos = (y + 1, x); new_direction = Direction::Up; *area += x as i32},
            _ => panic!("Invalid direction"),
        },
        '-' => match direction {
            Direction::Left => { next_pos = (y, x + 1); new_direction = Direction::Left; *area -= y as i32},
            Direction::Right => { next_pos = (y, x - 1); new_direction = Direction::Right; *area += y as i32},
            _ => panic!("Invalid direction"),
        },
        '7' => match direction {
            Direction::Left => { next_pos = (y +1 , x); new_direction = Direction::Up; *area += x as i32},
            Direction::Down => { next_pos = (y, x - 1); new_direction = Direction::Right; *area += y as i32},
            _ => panic!("Invalid direction"),
        },
        'L' => match direction {
            Direction::Right => { next_pos = (y - 1, x); new_direction = Direction::Down; *area -= x as i32},
            Direction::Up => { next_pos = (y, x + 1); new_direction = Direction::Left; *area -= y as i32},
            _ => panic!("Invalid direction"),
        },
        'J' => match direction {
            Direction::Left => { next_pos = (y - 1, x); new_direction = Direction::Down; *area -= x as i32},
            Direction::Up => { next_pos = (y, x - 1); new_direction = Direction::Right; *area += y as i32},
            _ => panic!("Invalid direction"),
        },
        'F' => match direction {
            Direction::Right => { next_pos = (y + 1, x); new_direction = Direction::Up; *area += x as i32},
            Direction::Down => { next_pos = (y, x + 1); new_direction = Direction::Left; *area -= y as i32}
            _ => panic!("Invalid direction"),
        },
        _ => panic!("Invalid character"),
    };
    (new_direction, next_pos)
}