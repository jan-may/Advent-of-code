use crate::lib::parse_input;

struct Present {
    length: i32,
    width: i32,
    height: i32,
}

impl Present {
    fn new(length: i32, width: i32, height: i32) -> Self {
        Present { length, width, height }
    }

    fn sides(&self) -> [i32; 3] {
        let lwh = [
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ];
        lwh
    }
}

pub fn part_1() -> i32 {
    let input = parse_input("inputs/day_02.txt");
    let mut total = 0;
    for word in input {
        let dimensions: Vec<i32> = word.split('x').map(|s| s.parse().unwrap()).collect();
        let present = Present::new(dimensions[0], dimensions[1], dimensions[2]);
        let sides = present.sides();
        let min_side = *sides.iter().min().unwrap();
        total += 2 * (sides[0] + sides[1] + sides[2]) + min_side;
    }
    total
}

pub fn part_2() -> i32 {
    let input = parse_input("inputs/day_02.txt");
    let mut total = 0;
    for word in input {
        let dimensions: Vec<i32> = word.split('x').map(|s| s.parse().unwrap()).collect();
        let present = Present::new(dimensions[0], dimensions[1], dimensions[2]);
        let bow_length = present.length * present.width * present.height;
        let mut sorted_dimensions = [present.length, present.width, present.height];
        sorted_dimensions.sort();
        let ribbon_length = 2 * sorted_dimensions[0] + 2 * sorted_dimensions[1];
        total += bow_length + ribbon_length;
    }
    total
}
