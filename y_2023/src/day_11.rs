use crate::lib::parse_input;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    x_expansions: i64,
    y_expansions: i64,
}

const EXPANSION_RATE_P1: i64 = 1;
const EXPANSION_RATE_P2: i64 = 1000000 - 1;

pub fn part_1() -> i64 {
    let input = parse_input("inputs/day_11.txt");
    let galaxies = get_galaxy_data(input, EXPANSION_RATE_P1);
    calc_all_distances(galaxies)
}

pub fn part_2() -> i64 {
    let input = parse_input("inputs/day_11.txt");
    let galaxies = get_galaxy_data(input, EXPANSION_RATE_P2);
    calc_all_distances(galaxies)
}

fn get_galaxy_data(input: Vec<String>, expansion_rate: i64) -> Vec<Point> {
    let col_count = input.first().map_or(0, |row| row.len());

    let empty_rows: Vec<bool> = input
        .iter()
        .map(|row| row.chars().all(|char| char != '#'))
        .collect();

    let empty_cols: Vec<bool> = (0..col_count)
        .map(|col| input.iter().all(|row| row.chars().nth(col).unwrap_or('#') != '#'))
        .collect();

    let mut galaxies = Vec::new();
    for (i, row) in input.iter().enumerate() {
        for (j, char) in row.chars().enumerate() {
            if char == '#' {
                let x_expansions = empty_cols[..j].iter().filter(|&&empty| empty).count() as i64 * expansion_rate;
                let y_expansions = empty_rows[..i].iter().filter(|&&empty| empty).count() as i64 * expansion_rate;

                let galaxy = Point {
                    x: j as i64,
                    y: i as i64,
                    x_expansions,
                    y_expansions,
                };
                galaxies.push(galaxy);
            }
        }
    }
    galaxies
}

fn calc_distance(galaxy: &Point, other_galaxy: &Point) -> i64 {
    let x_distance = i64::abs(galaxy.x + galaxy.x_expansions - other_galaxy.x - other_galaxy.x_expansions);
    let y_distance = i64::abs(galaxy.y + galaxy.y_expansions - other_galaxy.y - other_galaxy.y_expansions);
    x_distance + y_distance
}

fn calc_all_distances(galaxies: Vec<Point>) -> i64 {
    let mut distance: i64 = 0;
    for (i, galaxy) in galaxies.iter().enumerate() {
        for other_galaxy in galaxies.iter().skip(i + 1) {
            distance += calc_distance(galaxy, other_galaxy);
        }
    }
    distance
}
