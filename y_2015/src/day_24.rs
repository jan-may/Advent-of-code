use crate::lib::parse_input;

// Recursively computes the smallest product of values that add up to the target capacity.
// It's a solution to the multiple subset sum problem.
fn recursive_pack(
    values: &[u128],
    current_index: usize,
    current_sum: u128,
    current_product: u128,
    target_capacity: u128,
    minimum_product: &mut u128,
) {
    if current_sum == target_capacity {
        *minimum_product = std::cmp::min(current_product, *minimum_product);
    } else if current_sum < target_capacity && current_product < *minimum_product && current_index < values.len() {
        recursive_pack(values, current_index + 1, current_sum, current_product, target_capacity, minimum_product);
        let next_value = values[current_index];
        recursive_pack(values, current_index + 1, current_sum + next_value, current_product * next_value, target_capacity, minimum_product);
    }
}

// Finds the quantum entanglement (product of values) of the smallest group of packages that can exactly fill a compartment.
// Returns The minimum product of the weights that reach the target capacity.
fn quantum_entanglement(values: &[u128], target_capacity: u128) -> u128 {
    let mut minimum_product = u128::MAX;
    recursive_pack(values, 0, 0, 1, target_capacity, &mut minimum_product);
    minimum_product
}

pub fn part_1() -> u128 {
    let input = parse_input("inputs/day_24.txt");
    let packages = input
        .iter()
        .filter_map(|line| line.parse::<u128>().ok())
        .collect::<Vec<_>>();
    quantum_entanglement(&packages, packages.iter().sum::<u128>() / 3)
}

pub fn part_2() -> u128 {
    let input = parse_input("inputs/day_24.txt");
    let packages = input
        .iter()
        .filter_map(|line| line.parse::<u128>().ok())
        .collect::<Vec<_>>();
    quantum_entanglement(&packages, packages.iter().sum::<u128>() / 4)
}

