use crate::lib::parse_input;

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: i32,
    fly_time: i32,
    rest_time: i32,
    distance_covered: i32,
    fly_time_left: i32,
    resting_time_left: i32,
    points: i32,
}

pub fn part_1() -> i32 {
    let reindeers = generate_reindeers();
    let mut max = 0;
    for reindeer in reindeers {
        let mut distance = 0;
        let mut time = 2503;
        while time > 0 {
            if time >= reindeer.fly_time {
                distance += reindeer.speed * reindeer.fly_time;
                time -= reindeer.fly_time;
            } else {
                distance += reindeer.speed * time;
                time = 0;
            }
            if time >= reindeer.rest_time {
                time -= reindeer.rest_time;
            } else {
                time = 0;
            }
        }
        max = std::cmp::max(max, distance);
    }
    max
}

pub fn part_2() -> i32 {
    let mut reindeers = generate_reindeers();
    for _ in 0..2503 {
        for reindeer in reindeers.iter_mut() {
            if reindeer.fly_time_left > 0 {
                reindeer.distance_covered += reindeer.speed;
                reindeer.fly_time_left -= 1;
                if reindeer.fly_time_left == 0 {
                    reindeer.resting_time_left = reindeer.rest_time;
                }
            } else if reindeer.fly_time_left == 0 {
                reindeer.resting_time_left -= 1;
                if reindeer.resting_time_left == 0 {
                    reindeer.fly_time_left = reindeer.fly_time;
                }
            }
        }
        let max_distance = reindeers
            .iter()
            .max_by_key(|r| r.distance_covered)
            .unwrap()
            .distance_covered;

        for reindeer in reindeers.iter_mut() {
            if reindeer.distance_covered == max_distance {
                reindeer.points += 1;
            }
        }
    }

    return reindeers.iter().map(|r| r.points).max().unwrap();
}

fn generate_reindeers() -> Vec<Reindeer> {
    let input = parse_input("inputs/day_14.txt")
        .iter()
        .map(|s| {
            s.to_string()
                .replace("can fly ", "")
                .replace(" km/s for ", " ")
                .replace(" seconds, but then must rest for ", " ")
                .replace(" seconds.", " ")
        })
        .collect::<Vec<String>>();
    let mut reindeers: Vec<Reindeer> = Vec::new();
    for line in input {
        let mut split = line.split_whitespace();
        let name = split.next().unwrap().to_string();
        let speed = split.next().unwrap().parse::<i32>().unwrap();
        let fly_time = split.next().unwrap().parse::<i32>().unwrap();
        let rest_time = split.next().unwrap().parse::<i32>().unwrap();
        reindeers.push(Reindeer {
            name,
            speed,
            fly_time,
            rest_time,
            distance_covered: 0,
            fly_time_left: fly_time,
            resting_time_left: 0,
            points: 0,
        });
    }
    reindeers
}
