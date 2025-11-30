use std::collections::HashMap;
advent_of_code::solution!(7);

#[derive(Debug, Clone)]
enum Expr {
    Val(u16),
    Wire(String),
    Not(String),
    And(String, String),
    Or(String, String),
    LShift(String, u8),
    RShift(String, u8),
}

fn parse_input(input: &str) -> HashMap<String, Expr> {
    let mut map = HashMap::new();
    input.lines().filter(|l| !l.is_empty()).for_each(|l| {
        let (k, v) = parse_line(l);
        map.insert(k, v);
    });
    map
}

fn parse_line(line: &str) -> (String, Expr) {
    let (left, target) = line.trim().split_once(" -> ").unwrap();
    let expr: Expr = if let Ok(v) = left.parse::<u16>() {
        Expr::Val(v)
    } else {
        let parts: Vec<&str> = left.split_whitespace().collect();
        match parts.as_slice() {
            [w] => Expr::Wire(w.to_string()),
            ["NOT", a] => Expr::Not(a.to_string()),
            [a, "AND", b] => Expr::And(a.to_string(), b.to_string()),
            [a, "OR", b] => Expr::Or(a.to_string(), b.to_string()),
            [a, "LSHIFT", n] => Expr::LShift(a.to_string(), n.parse().unwrap()),
            [a, "RSHIFT", n] => Expr::RShift(a.to_string(), n.parse().unwrap()),
            _ => panic!(),
        }
    };
    (target.to_string(), expr)
}

fn eval(wire: &str, map: &HashMap<String, Expr>, memo: &mut HashMap<String, u16>) -> u16 {
    // Numeric case
    if let Ok(v) = wire.parse::<u16>() {
        return v;
    }

    // Check memo
    if let Some(&v) = memo.get(wire) {
        return v;
    }

    // Look up instruction
    let expr = map.get(wire).unwrap_or_else(|| panic!("Unknown wire"));
    let result = match expr {
        Expr::Val(v) => *v,
        Expr::Wire(a) => eval(a, map, memo),
        Expr::Not(a) => !eval(a, map, memo),
        Expr::And(a, b) => eval(a, map, memo) & eval(b, map, memo),
        Expr::Or(a, b) => eval(a, map, memo) | eval(b, map, memo),
        Expr::LShift(a, n) => eval(a, map, memo) << n,
        Expr::RShift(a, n) => eval(a, map, memo) >> n,
    };
    memo.insert(wire.to_string(), result);
    result
}

pub fn part_one(input: &str) -> Option<u16> {
    let map = parse_input(input);
    let mut memo = HashMap::new();
    Some(eval("a", &map, &mut memo))
}

pub fn part_two(input: &str) -> Option<u16> {
    let mut map = parse_input(input);
    let mut memo = HashMap::new();
    let wire_a_signal = eval("a", &map, &mut memo);

    // Overwrite b signal with a
    map.insert("b".to_string(), Expr::Val(wire_a_signal));
    // Reset memo
    let mut memo = HashMap::new();
    Some(eval("a", &map, &mut memo))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65412));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65412));
    }
}
