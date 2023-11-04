use crate::lib::parse_input;

#[derive(Debug ,PartialEq, Default)]
struct Sue {
        sue_number: i32,
        children: Option<i32>,
        cats: Option<i32>,
        samoyeds: Option<i32>,
        pomeranians: Option<i32>,
        akitas: Option<i32>,
        vizslas: Option<i32>,
        goldfish: Option<i32>,
        trees: Option<i32>,
        cars: Option<i32>,
        perfumes: Option<i32>,
}

pub fn part_1() -> i32 {
    let sues = generate_sues();
    let mut result = 0;
    let filtered = sues.iter().filter(|sue| {
        sue.children.map(|val| val == 3).unwrap_or(true)
            && sue.cats.map(|val| val == 7).unwrap_or(true)
            && sue.samoyeds.map(|val| val == 2).unwrap_or(true)
            && sue.pomeranians.map(|val| val == 3).unwrap_or(true)
            && sue.akitas.map(|val| val == 0).unwrap_or(true)
            && sue.vizslas.map(|val| val == 0).unwrap_or(true)
            && sue.goldfish.map(|val| val == 5).unwrap_or(true)
            && sue.trees.map(|val| val == 3).unwrap_or(true)
            && sue.cars.map(|val| val == 2).unwrap_or(true)
            && sue.perfumes.map(|val| val == 1).unwrap_or(true)
    }).collect::<Vec<&Sue>>();
    match filtered.len() {
        1 => result = filtered[0].sue_number,
        _ => panic!("did not found exactly one sue"),
    }
    result
}



pub fn part_2() -> i32 {
    let sues = generate_sues();
    let mut result = 0;
    let filtered = sues.iter().filter(|sue| {
        sue.children.map(|val| val == 3).unwrap_or(true)
            && sue.cats.map(|val| val > 7).unwrap_or(true)
            && sue.samoyeds.map(|val| val == 2).unwrap_or(true)
            && sue.pomeranians.map(|val| val < 3).unwrap_or(true)
            && sue.akitas.map(|val| val == 0).unwrap_or(true)
            && sue.vizslas.map(|val| val == 0).unwrap_or(true)
            && sue.goldfish.map(|val| val < 5).unwrap_or(true)
            && sue.trees.map(|val| val > 3).unwrap_or(true)
            && sue.cars.map(|val| val == 2).unwrap_or(true)
            && sue.perfumes.map(|val| val == 1).unwrap_or(true)
    }).collect::<Vec<&Sue>>();
    match filtered.len() {
        1 => result = filtered[0].sue_number,
        _ => panic!("did not found exactly one sue"),
    }
    result

}

fn generate_sues() -> Vec<Sue> {
    let input = parse_input("inputs/day_16.txt").iter().map(|s| s.to_string().replace("Sue ", "").replace(": ", " ").replace(", ", " ")).collect::<Vec<String>>();
    let mut sues: Vec<Sue> = Vec::new();
    for line in input {
        let mut parts  = line.split(" ");
        let sue_number = parts .next().unwrap();
        let mut sue = Sue {sue_number: sue_number.parse::<i32>().unwrap(), ..Default::default()};
        while let Some(key) = parts .next() {
            let value = parts .next().unwrap().parse::<i32>().unwrap();
            match key {
                "children" => sue.children = Some(value),
                "cats" => sue.cats = Some(value),
                "samoyeds" => sue.samoyeds = Some(value),
                "pomeranians" => sue.pomeranians = Some(value),
                "akitas" => sue.akitas = Some(value),
                "vizslas" => sue.vizslas = Some(value),
                "goldfish" => sue.goldfish = Some(value),
                "trees" => sue.trees = Some(value),
                "cars" => sue.cars = Some(value),
                "perfumes" => sue.perfumes = Some(value),
                _ => panic!("unknown key"),
            }
        }
        sues.push(sue);
    }
    sues
}