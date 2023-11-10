use crate::lib::parse_input;

pub fn part_1() -> i32 {
    let input = parse_input("inputs/day_03.txt");
    let mut count = 0;
    for triangle in input {
        let mut sides = triangle.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        sides.sort();
        if sides[0] + sides[1] > sides[2] {
            count += 1;
        }
    }
    count
}

pub fn part_2() -> i32 {
    let input = parse_input("inputs/day_03.txt");
    let mut count = 0;

    for i in 0..input.len() / 3 {
        let mut sides = Vec::new();
        for j in 0..3 {
            let triangle = input[i * 3 + j].split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            sides.push(triangle);
        }
        for j in 0..3 {
            let mut triangle = Vec::new();
            for k in 0..3 {
                triangle.push(sides[k][j]);
            }
            triangle.sort();
            if triangle[0] + triangle[1] > triangle[2] {
                count += 1;
            }
        }
    }
    count
}




