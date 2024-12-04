#![feature(iter_map_windows)]
#![feature(ascii_char)]
#![allow(dead_code, special_module_name)]
#![allow(clippy::needless_range_loop)]

mod day_01;
mod day_02;
mod day_03;
mod lib;

fn main() {
    // println!("day 1: p1 {} | p2 {}", day_01::part_1(), day_01::part_2());
    // println!("day 2: p1 {} | p2 {}", day_02::part_1(), day_02::part_2());
    println!("day 2: p1 {} | p2 {}", day_03::part_1(), day_03::part_2());
}
