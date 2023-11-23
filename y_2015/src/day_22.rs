#[derive(Clone, Debug)]
struct Player {
    hp: i32,
    armor: i32,
    damage: i32,
    mana: i32,
}

#[derive(Clone, Debug)]
struct Boss {
    hp: i32,
    damage: i32,
}

#[derive(Clone, Debug)]
struct Effect {
    name: String,
    timer: i32,
    mana: i32,
    damage: i32,
    armor: i32,
    heal: i32,
    mana_back: i32,
}

pub fn part_1() -> usize {
    let mut player = Player { hp: 50, armor: 0, damage: 0, mana: 500 };
    let mut boss = Boss { hp: 71, damage: 10 };
    let spells = generate_spells();
    let mana_spent = 0;
    let mut effects = Vec::new();
    let mut min_mana = usize::MAX;
    simulate(&mut player, &mut boss, &spells, mana_spent, &mut effects, &mut min_mana, false);
    min_mana
}

pub fn part_2() -> usize {
    let mut player = Player { hp: 50, armor: 0, damage: 0, mana: 500 };
    let mut boss = Boss { hp: 71, damage: 10 };
    let spells = generate_spells();
    let mana_spent = 0;
    let mut effects = Vec::new();
    let mut min_mana = usize::MAX;
    simulate(&mut player, &mut boss, &spells, mana_spent, &mut effects, &mut min_mana, true);
    min_mana
}

fn generate_spells() -> Vec<Effect> {
    let mut spells = Vec::new();
    spells.push(Effect { name: "Magic Missile".to_string(), timer: 0, mana: 53, damage: 4, armor: 0, heal: 0, mana_back: 0 });
    spells.push(Effect { name: "Drain".to_string(), timer: 0, mana: 73, damage: 2, armor: 0, heal: 2, mana_back: 0 });
    spells.push(Effect { name: "Shield".to_string(), timer: 6, mana: 113, damage: 0, armor: 7, heal: 0, mana_back: 0});
    spells.push(Effect { name: "Poison".to_string(), timer: 6, mana: 173, damage: 3, armor: 0, heal: 0, mana_back: 0});
    spells.push(Effect { name: "Recharge".to_string(), timer: 5, mana: 229, damage: 0, armor: 0, heal: 0, mana_back: 101});
    spells
}

fn apply_effects(player: &mut Player, boss: &mut Boss, effects: &mut Vec<Effect>) {
    player.armor = 0; // Reset armor at the start of each turn
    for effect in effects.iter_mut() {
        match effect.name.as_str() {
            "Poison" => {
                match boss.hp > effect.damage {
                    true => boss.hp -= effect.damage,
                    false => boss.hp = 0,
                }
            },
            "Recharge" => {
                player.mana += effect.mana_back;
            },
            "Shield" => {
                player.armor += effect.armor;
            },
            _ => {},
        }
        effect.timer -= 1;
    }
    effects.retain(|effect| effect.timer > 0);
}


fn simulate(
    player: &mut Player,
    boss: &mut Boss,
    spells: &Vec<Effect>,
    mana_spent: usize,
    effects: &mut Vec<Effect>,
    min_mana: &mut usize,
    hard_mode: bool,
) {
    for spell in spells.iter() {
        // check preconditions
        if mana_spent + spell.mana as usize >= *min_mana || player.mana < spell.mana || player.hp <= 0 {
            continue;
        }

        let mut new_effects = effects.clone();
        let mut new_player = player.clone();
        let mut new_boss = boss.clone();

        match hard_mode {
            true => {
                new_player.hp -= 1;
                if new_player.hp <= 0 {
                    continue; // Player loses
                }
            }
            false => {},
        }

        // Apply spell effects before player's turn
        apply_effects(&mut new_player, &mut new_boss, &mut new_effects);

        if new_boss.hp <= 0 {
            // Check if boss is defeated
            if mana_spent < *min_mana {
                *min_mana = mana_spent;
            }
            continue;
        }

        // Player's turn
        // Check if spell is already active
        if new_effects.iter().any(|effect| effect.name == spell.name) {
            continue;
        }

        // Apply players spell
        new_player.mana -= spell.mana;
        new_boss.hp -= match spell.name.as_str() {
            "Magic Missile" => spell.damage,
            "Drain" => spell.damage,
            _ => 0,
        };

        // push spell to active effects
        new_effects.push(spell.clone());

        let total_mana_spent = mana_spent + spell.mana as usize;

        // Apply effects before boss's turn
        apply_effects(&mut new_player, &mut new_boss, &mut new_effects);

        if new_boss.hp <= 0 {
            // Check if boss is defeated
            if total_mana_spent < *min_mana {
                *min_mana = total_mana_spent;
            }
            continue;
        }

        // Boss's turn -> at least 1 damage
        let damage = (new_boss.damage - new_player.armor).max(1);
        new_player.hp -= damage;

        if new_player.hp <= 0 {
            continue; // Player loses
        }
        // Recursive call
        simulate(
            &mut new_player,
            &mut new_boss,
            spells,
            total_mana_spent,
            &mut new_effects,
            min_mana,
            hard_mode,
        );
    }
}