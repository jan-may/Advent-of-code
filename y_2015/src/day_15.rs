use crate::lib::parse_input;

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

pub fn part_1() -> i32 {
    let ingredients = generate_ingredients();
    let permutations = generate_permutations();
    let need_calories = false;
    return calc_score(ingredients, permutations, need_calories);
}

pub fn part_2() -> i32 {
    let ingredients = generate_ingredients();
    let permutations = generate_permutations();
    let need_calories = true;
    return calc_score(ingredients, permutations, need_calories);
}

fn calc_score(
    ingredients: Vec<Ingredient>,
    permutations: Vec<Vec<i32>>,
    need_calories: bool,
) -> i32 {
    let mut max_score = 0;
    for permutation in permutations {
        let (mut capacity, mut durability, mut flavor, mut texture, mut calories) = (0, 0, 0, 0, 0);
        for (i, ingredient) in ingredients.iter().enumerate() {
            capacity += ingredient.capacity * permutation[i];
            durability += ingredient.durability * permutation[i];
            flavor += ingredient.flavor * permutation[i];
            texture += ingredient.texture * permutation[i];
            calories += ingredient.calories * permutation[i];
        }
        if capacity < 0 || durability < 0 || flavor < 0 || texture < 0 {
            continue;
        }
        if need_calories {
            if calories == 500 {
                let score = capacity * durability * flavor * texture;
                max_score = std::cmp::max(max_score, score);
            }
        } else {
            let score = capacity * durability * flavor * texture;
            max_score = std::cmp::max(max_score, score)
        }
    }
    max_score
}

fn generate_permutations() -> Vec<Vec<i32>> {
    // permutations of 4 ingredients with 100 teaspoons -> the sum of the permutations must be 100
    let mut permutations: Vec<Vec<i32>> = Vec::new();
    for i in 0..=100 {
        for j in 0..=100 {
            for k in 0..=100 {
                let l = 100 - i - j - k;
                if l >= 0 {
                    permutations.push(vec![i, j, k, l]);
                }
            }
        }
    }
    permutations
}

fn generate_ingredients() -> Vec<Ingredient> {
    let input = &parse_input("inputs/day_15.txt")
        .iter()
        .map(|s| s.to_string().replace(":", "").replace(", ", " "))
        .collect::<Vec<String>>();
    input
        .iter()
        .map(|line| {
            let mut split = line.split_whitespace();
            let name = split.next().unwrap().to_string();
            let capacity = split.nth(1).unwrap().parse::<i32>().unwrap();
            let durability = split.nth(1).unwrap().parse::<i32>().unwrap();
            let flavor = split.nth(1).unwrap().parse::<i32>().unwrap();
            let texture = split.nth(1).unwrap().parse::<i32>().unwrap();
            let calories = split.nth(1).unwrap().parse::<i32>().unwrap();
            Ingredient {name, capacity, durability, flavor, texture,calories, }
        })
        .collect()
}
