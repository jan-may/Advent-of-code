use crate::lib::parse_input;
use serde_json::{Value};

pub fn part_1() -> i32 {
    let input = &parse_input("inputs/day_12.txt")[0];
    let v: Value = serde_json::from_str(input).unwrap();
    sum_json_value(&v)
}

pub fn part_2() -> i32 {
    let input = &parse_input("inputs/day_12.txt")[0];
    let v: Value = serde_json::from_str(input).unwrap();
    sum_json_value_without_red(&v)
}

fn sum_json_value(v: &Value) -> i32 {
    match v {
        Value::Number(n) => {
            n.as_i64().unwrap() as i32
        },
        Value::Array(a) => {
            a.iter().map(|v| sum_json_value(v)).sum()
        },
        Value::Object(o) => {
            let mut sum = 0;
            for (_, v) in o {
                sum += sum_json_value(v);
            }
            sum
        },
        // String, Bool, Null
        _ => 0,
    }
}

fn sum_json_value_without_red(v: &Value) -> i32 {
    match v {
        Value::Number(n) => {
            n.as_i64().unwrap() as i32
        },
        Value::Array(a) => {
            a.iter().map(|v| sum_json_value(v)).sum()
        },
        Value::Object(o) => {
            let mut sum = 0;
            let mut has_red = false;
            for (k, v) in o {
                if let Some(Value::String(ref s)) = o.get(k) {
                    if s == "red" {
                        has_red = true;
                    }
                }
                sum += sum_json_value_without_red(v);
            }
            match has_red {
                true => 0,
                false => sum,
            }
        },
        // String, Bool, Null
        _ => 0,
    }
}