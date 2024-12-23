#![feature(iter_map_windows)]
#![feature(ascii_char)]
#![allow(dead_code, special_module_name)]
#![allow(clippy::needless_range_loop)]

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod lib;

fn main() {
    // println!("day 1: p1 {} | p2 {}", day_01::part_1(), day_01::part_2());
    // println!("day 2: p1 {} | p2 {}", day_02::part_1(), day_02::part_2());
    // println!("day 3: p1 {} | p2 {}", day_03::part_1(), day_03::part_2());
    // println!("day 4: p1 {} | p2 {}", day_04::part_1(), day_04::part_2());
    // println!("day 5: p1 {} | p2 {}", day_05::part_1(), 0);
    // println!("day 6: p1 {} | p2 {}", day_06::part_1(), day_06::part_2());
    println!("day 7: p1 {} | p2 {}", day_07::part_1(), day_07::part_2());
}
