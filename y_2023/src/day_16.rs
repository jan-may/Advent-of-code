use std::collections::{HashSet};
use crate::lib::parse_input;
use rayon::prelude::*;


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
    fn move_beam(beam: Beam, queue: &mut Vec<Beam>, grid: &[Vec<char>]) {
        let (x,y) = beam.position;
        let (width, height) = (grid[0].len(), grid.len());
        let new_pos = match beam.direction {
            Direction::Up if y > 0 => Some((x, y - 1)),
            Direction::Down if y < height - 1 => Some((x, y + 1)),
            Direction::Left if x > 0 => Some((x - 1, y)),
            Direction::Right if x < width - 1 => Some((x + 1, y)),
            _ => None
        };
        if new_pos.is_none() { return; }
        let new_pos = new_pos.unwrap();
        match beam.direction {
            Direction::Up => {
                match grid[y - 1][x] {
                    '|' | '.' => {
                        queue.push(Beam::new(new_pos, Direction::Up)); }
                    '\\' => queue.push(Beam::new(new_pos, Direction::Left)),
                    '/' => queue.push(Beam::new(new_pos, Direction::Right)),
                    '-' => {
                        queue.push(Beam::new(new_pos, Direction::Left));
                        queue.push(Beam::new(new_pos, Direction::Right));
                    },
                    _ => {}
                }
            },
            Direction::Down => {
                match grid[y + 1][x] {
                    '\\' => queue.push(Beam::new(new_pos, Direction::Right)),
                    '/' => queue.push(Beam::new(new_pos, Direction::Left)),
                    '-' => {
                        queue.push(Beam::new(new_pos, Direction::Left));
                        queue.push(Beam::new(new_pos, Direction::Right));
                    },
                    _ => queue.push(Beam::new(new_pos, Direction::Down))
                }
            },
            Direction::Left => {
                match grid[y][x - 1] {
                    '\\' => queue.push(Beam::new(new_pos, Direction::Up)),
                    '/' => queue.push(Beam::new(new_pos, Direction::Down)),
                    '|' => {
                        queue.push(Beam::new(new_pos, Direction::Up));
                        queue.push(Beam::new(new_pos, Direction::Down));
                    },
                    _ => queue.push(Beam::new(new_pos, Direction::Left))
                }
            },
            Direction::Right => {
                match grid[y][x + 1] {
                    '\\' => queue.push(Beam::new(new_pos, Direction::Down)),
                    '/' => queue.push(Beam::new(new_pos, Direction::Up)),
                    '|' => {
                        queue.push(Beam::new(new_pos, Direction::Up));
                        queue.push(Beam::new(new_pos, Direction::Down));
                    },
                    _ => queue.push(Beam::new(new_pos, Direction::Right))
                }
            }
        }
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
    let max_energy_top_and_bottom = (0..grid[0].len())
        .into_par_iter()
        .map(|x| {
            let beam_top = Beam::new((x, 0), Direction::Down);
            let beam_bottom = Beam::new((x, grid.len() - 1), Direction::Up);
            calc_energy_level(&grid, beam_top).max(calc_energy_level(&grid, beam_bottom))
        })
        .max()
        .unwrap_or(0);

    let max_energy_left_and_right = (0..grid.len())
        .into_par_iter()
        .map(|y| {
            let beam_left = Beam::new((0, y), Direction::Right);
            let beam_right = Beam::new((grid[0].len() - 1, y), Direction::Left);
            calc_energy_level(&grid, beam_left).max(calc_energy_level(&grid, beam_right))
        })
        .max()
        .unwrap_or(0);

    max_energy_top_and_bottom.max(max_energy_left_and_right)
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
    let mut queue = vec![first_beam];
    while !queue.is_empty() {
        // cant get this to work for mutating first element of queue in place
        // so we remove it and add it back in... still fast enough.
        let beam = queue.remove(0);
            if visited.contains(&beam) { continue; }
            energized[beam.position.1][beam.position.0] = 'X';
            visited.insert(beam);
            Beam::move_beam(beam, &mut queue, grid);
    }
    energized.iter().map(|line| line.iter().filter(|c| **c == 'X').count()).sum()
}