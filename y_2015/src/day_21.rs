use crate::lib::parse_input;

#[derive(Clone)]
struct Item {
    name: String,
    cost: i32,
    damage: i32,
    armor: i32,
}

#[derive(Debug, Clone)]
struct Character {
    name: String,
    hit_points: i32,
    damage: i32,
    armor: i32,
}

impl Item {
    fn new(name: &str, cost: i32, damage: i32, armor: i32) -> Self {
        Item {
            name: name.to_string(),
            cost,
            damage,
            armor,
        }
    }
}

impl Character {
    fn new(name: String, hit_points: i32, damage: i32, armor: i32) -> Character {
        Character {
            name,
            hit_points,
            damage,
            armor,
        }
    }
}

pub fn part_1_and_2() -> (i32, i32) {
    let (weapons, armors, rings) = generate_possible_equipment();
    let boss = parse_character();
    let mut min_cost = i32::MAX;
    let mut max_cost = 0;
    // we need to buy at least one weapon
    // armor and rings are optional
    for weapon in &weapons {
        for armor in &armors {
            for ring1 in &rings {
                for ring2 in &rings {
                    if ring1.name == ring2.name {
                        continue;
                    }
                    let player_damage = weapon.damage + ring1.damage + ring2.damage;
                    let player_armor = armor.armor + ring1.armor + ring2.armor;
                    let cost = weapon.cost + armor.cost + ring1.cost + ring2.cost;
                    let mut player = Character::new("player".to_string(), 100, player_damage, player_armor);

                    if fight(&mut player, &boss) {
                        min_cost = min_cost.min(cost);
                    } else {
                        max_cost = max_cost.max(cost);
                    }
                }
            }
        }
    }
    (min_cost, max_cost)
}

fn fight(player: &mut Character, boss: &Character) -> bool {
    let mut boss = boss.clone();

    loop {
        boss.hit_points -= (player.damage - boss.armor).max(1);
        if boss.hit_points <= 0 {
            return true;
        }
        player.hit_points -= (boss.damage - player.armor).max(1);
        if player.hit_points <= 0 {
            return false;
        }
    }
}

fn parse_character() -> Character {
    let lines = parse_input("inputs/day_21.txt");
    let hit_points = lines[0].split(": ").nth(1).unwrap().parse::<i32>().unwrap();
    let damage = lines[1].split(": ").nth(1).unwrap().parse::<i32>().unwrap();
    let armor = lines[2].split(": ").nth(1).unwrap().parse::<i32>().unwrap();
    Character::new("boss".to_string(), hit_points, damage, armor)
}

fn generate_possible_equipment() -> (Vec<Item>, Vec<Item>, Vec<Item>) {
    let weapons = vec![
        Item::new("Dagger", 8, 4, 0),
        Item::new("Shortsword", 10, 5, 0),
        Item::new("Warhammer", 25, 6, 0),
        Item::new("Longsword", 40, 7, 0),
        Item::new("Greataxe", 74, 8, 0),
    ];
    let armor = vec![
        Item::new("None", 0, 0, 0),
        Item::new("Leather", 13, 0, 1),
        Item::new("Chainmail", 31, 0, 2),
        Item::new("Splintmail", 53, 0, 3),
        Item::new("Bandedmail", 75, 0, 4),
        Item::new("Platemail", 102, 0, 5),
    ];
    let rings = vec![
        Item::new("None", 0, 0, 0),
        Item::new("Damage +1", 25, 1, 0),
        Item::new("Damage +2", 50, 2, 0),
        Item::new("Damage +3", 100, 3, 0),
        Item::new("Defense +1", 20, 0, 1),
        Item::new("Defense +2", 40, 0, 2),
        Item::new("Defense +3", 80, 0, 3),
    ];
    (weapons, armor, rings)
}
