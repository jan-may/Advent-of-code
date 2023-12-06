use crate::lib::parse_input;

pub fn part_1() -> i64 {
    let input = parse_input("inputs/day_06.txt");
    let times = input[0].split(' ').filter_map(|num| num.parse::<i64>().ok()).collect::<Vec<i64>>();
    let distances = input[1].split(' ').filter_map(|num| num.parse::<i64>().ok()).collect::<Vec<i64>>();
    let mut winning_races = 1;

    for (&max_time, &target_distance) in times.iter().zip(&distances) {
        let mut count = 0;
        for time_btn_pressed in 0..max_time {
            if simulate_race(time_btn_pressed, max_time, target_distance) {
                count += 1;
            }
        }
        if count > 0 {
            winning_races *= count;
        }
    }
    winning_races
}

pub fn part_2() -> i64 {
    let input = parse_input("inputs/day_06.txt");
    let times_arr = input[0].split(' ').filter_map(|num| num.parse::<i32>().ok()).collect::<Vec<i32>>();
    let max_time = times_arr.iter().map(|num| num.to_string()).collect::<Vec<String>>().join("").parse::<i64>().unwrap();
    let distances_arr = input[1].split(' ').filter_map(|num| num.parse::<i32>().ok()).collect::<Vec<i32>>();
    let target_distance = distances_arr.iter().map(|num| num.to_string()).collect::<Vec<String>>().join("").parse::<i64>().unwrap();
    let mut winning_races = 0;

    for j in 0..max_time{
        if simulate_race(j, max_time, target_distance){
            winning_races += 1;
        }
    }
    winning_races
}

fn simulate_race(time_btn_pressed: i64, max_time:i64, target_distance: i64) -> bool {
    let speed = time_btn_pressed;
    let time_left = max_time - time_btn_pressed;
    let distance = speed * time_left;
    distance > target_distance
}