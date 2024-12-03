#![feature(iter_map_windows)]
#![feature(ascii_char)]
#![allow(dead_code, special_module_name)]
#![allow(clippy::needless_range_loop)]

mod day_01;
mod lib;

fn main() {
    println!("day 1: p1 {} | p2 {}", day_01::part_1(), day_01::part_2());
}
