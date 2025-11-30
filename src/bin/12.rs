use serde_json::Value;
advent_of_code::solution!(12);

fn parse(input: &str) -> Value {
    serde_json::from_str(input).expect("Invalid JSON")
}

// Part 1
fn sum_numbers(v: &Value) -> i64 {
    match v {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(sum_numbers).sum(),
        Value::Object(obj) => obj.values().map(sum_numbers).sum(),
        _ => 0,
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let val = parse(input);
    Some(sum_numbers(&val))
}

// Part 2
fn sum_numbers_wo_red(v: &Value) -> i64 {
    match v {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(sum_numbers_wo_red).sum(),
        Value::Object(obj) => {
            if obj.values().any(|v| v == "red") {
                // skip
                0
            } else {
                obj.values().map(sum_numbers_wo_red).sum()
            }
        }
        _ => 0,
    }
}

pub fn part_two(input: &str) -> Option<i64> {
    let val = parse(input);
    Some(sum_numbers_wo_red(&val))
}

#[cfg(test)]
mod tests {}
