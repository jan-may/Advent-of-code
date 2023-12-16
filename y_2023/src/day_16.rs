use std::collections::{HashSet};
use crate::lib::parse_input;

#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Beam {
    position: (usize,usize),
    direction: Direction
}

impl Beam {
    fn new(position: (usize,usize), direction: Direction) -> Beam {
        Beam { position, direction }
    }
    fn move_beam(beam: &Beam, grid: &[Vec<char>]) -> Option<Vec<Beam>> {
        let mut new_beams = Vec::new();
        let (x,y) = beam.position;
        let (width, height) = (grid[0].len(), grid.len());

        let new_pos = match beam.direction {
            Direction::Up if y > 0 => (x, y - 1),
            Direction::Down if y < height - 1 => (x, y + 1),
            Direction::Left if x > 0 => (x - 1, y),
            Direction::Right if x < width - 1 => (x + 1, y),
            _ => return None
        };

        match beam.direction {
            Direction::Up => {
                match grid[y - 1][x] {
                    '|' | '.' => {
                        new_beams.push(Beam::new(new_pos, Direction::Up));
                    }
                    '\\' => new_beams.push(Beam::new(new_pos, Direction::Left)),
                    '/' => new_beams.push(Beam::new(new_pos, Direction::Right)),
                    '-' => {
                        if x > 0 {
                            new_beams.push(Beam::new(new_pos, Direction::Left))
                        }
                        if x < width - 1 {
                            new_beams.push(Beam::new(new_pos, Direction::Right));
                        }
                    },
                    _ => return None
                }
            },
            Direction::Down => {
                match grid[y + 1][x] {
                    '|' | '.' => {
                        new_beams.push(Beam::new(new_pos, Direction::Down));
                    }
                    '\\' => new_beams.push(Beam::new(new_pos, Direction::Right)),
                    '/' => new_beams.push(Beam::new(new_pos, Direction::Left)),
                    '-' => {
                        if x > 0 {
                            new_beams.push(Beam::new(new_pos, Direction::Left))
                        }
                        if x < width - 1 {
                            new_beams.push(Beam::new(new_pos, Direction::Right));
                        }
                    },
                    _ => return None
                }
            },
            Direction::Left => {
                match grid[y][x - 1] {
                    '-' | '.' => {
                        new_beams.push(Beam::new(new_pos, Direction::Left));
                    }
                    '\\' => new_beams.push(Beam::new(new_pos, Direction::Up)),
                    '/' => new_beams.push(Beam::new(new_pos, Direction::Down)),
                    '|' => {
                        if y > 0 {
                            new_beams.push(Beam::new(new_pos, Direction::Up))
                        }
                        if y < height - 1 {
                            new_beams.push(Beam::new(new_pos, Direction::Down));
                        }
                    },
                    _ => return None
                }
            },
            Direction::Right => {
                match grid[y][x + 1] {
                    '-' | '.' => {
                        new_beams.push(Beam::new(new_pos, Direction::Right));
                    }
                    '\\' => new_beams.push(Beam::new(new_pos, Direction::Down)),
                    '/' => new_beams.push(Beam::new(new_pos, Direction::Up)),
                    '|' => {
                        if y > 0 {
                            new_beams.push(Beam::new(new_pos, Direction::Up))
                        }
                        if y < height - 1 {
                            new_beams.push(Beam::new(new_pos, Direction::Down));
                        }
                    },
                    _ => return None
                }
            },
        }
        Some(new_beams)
    }
}

pub fn part_1() -> usize {
    let grid = generate_grid();
    let start_direction = match grid[0][0] {
        '\\' | '/' | '|' => Direction::Down,
        '.' | '-' => Direction::Right,
        _ => panic!("Invalid start")
    };
    let first_beam = Beam::new((0,0), start_direction);
    calc_energy_level(&grid, first_beam)
}

pub fn part_2() -> usize {
    let grid = generate_grid();
    let mut result = 0;

    // every tile in top and bottom row
    for x in 0..grid[0].len() {
        let beam_top = Beam::new((x, 0), Direction::Down);
        let bem_bottom = Beam::new((x, grid.len() - 1), Direction::Up);
        result = result.max(calc_energy_level(&grid, beam_top)).max(calc_energy_level(&grid, bem_bottom));
    }

    // every tile in left and right column
    for y in 0..grid.len() {
        let beam_left = Beam::new((0, y), Direction::Right);
        let beam_right = Beam::new((grid[0].len() - 1, y), Direction::Left);
        result = result.max(calc_energy_level(&grid, beam_left)).max(calc_energy_level(&grid, beam_right));
    }

    result
}

fn generate_grid() -> Vec<Vec<char>> {
    parse_input("inputs/day_16.txt")
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn calc_energy_level(grid: &[Vec<char>], first_beam: Beam) -> usize {
    let mut visited: HashSet<Beam> = HashSet::new();
    let mut energized = grid.to_owned();
    let mut beams = vec![first_beam];
    while !beams.is_empty() {
        let mut new_beams: Vec<Vec<Beam>> = Vec::new();
        for beam in beams {
            if visited.contains(&beam) { continue; }
            energized[beam.position.1][beam.position.0] = 'X';
            visited.insert(beam);
            let new_beam = Beam::move_beam(&beam, grid);
            new_beams.push(new_beam.unwrap_or_default());
        }
        beams = new_beams.into_iter().flatten().collect();
    }
    energized.iter().map(|line| line.iter().filter(|c| **c == 'X').count()).sum()
}