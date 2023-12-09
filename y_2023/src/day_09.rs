use crate::lib::parse_input;

pub fn part_1() -> i32 {
    let histories = generate_sequences();
    process_histories(histories, true)
}

pub fn part_2() -> i32 {
    let histories = generate_sequences();
    process_histories(histories, false)
}

fn process_histories(histories: Vec<Vec<i32>>, is_part_one: bool) -> i32 {
    let mut total = 0;
    for history in histories {
        let mut predictions = generate_predictions(history);
        let mut diff = 0;
        for i in (0..predictions.len()-1).rev() {
            diff = if is_part_one {
                let line_len = predictions[i].len() - 1;
                let next_line_len = predictions[i+1].len() - 1;
                predictions[i+1][next_line_len] + predictions[i][line_len]
            } else {
                predictions[i][0] - predictions[i+1][0]
            };
            if is_part_one {
                predictions[i].push(diff);
            } else {
                predictions[i].insert(0, diff);
            }
        }
        total += diff;
    }
    total
}

fn generate_sequences() -> Vec<Vec<i32>> {
    parse_input("inputs/day_09.txt").iter().map(|line| {
        line.split_whitespace().filter_map(|num| num.parse::<i32>().ok()).collect()
    }).collect()
}

fn generate_predictions(mut seq: Vec<i32>) -> Vec<Vec<i32>> {
    let mut predictions = vec![seq.clone()];
    while seq.iter().any(|&x| x != 0) {
        let prediction: Vec<i32> = seq.windows(2).map(|window| window[1] - window[0]).collect();
        seq = prediction.clone();
        predictions.push(prediction);
    }
    predictions
}