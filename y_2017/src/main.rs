#![feature(iter_map_windows)]
#![allow(dead_code, special_module_name)]
mod day_01;

mod lib;

fn main() {
    println!("day 1: p1 {} | p2 {}", day_01::part_1(), day_01::part_2());
}