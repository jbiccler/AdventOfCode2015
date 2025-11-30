use std::collections::HashMap;

use itertools::Itertools;
advent_of_code::solution!(13);

fn parse_input(input: &str) -> (HashMap<(&str, &str), i64>, Vec<&str>) {
    let mut map = HashMap::new();
    let mut names = vec![];
    for line in input.lines() {
        let line = line.trim().trim_end_matches(".");
        let parts: Vec<&str> = line.split_whitespace().collect();
        // names
        let a = parts[0];
        let b = parts[parts.len() - 1];
        // gain / lose
        let mult: i64 = match parts[2] {
            "gain" => 1,
            "lose" => -1,
            _ => panic!(),
        };
        let amount = parts[3].parse::<i64>().unwrap();
        map.insert((a, b), amount * mult);
        if !names.contains(&a) {
            names.push(a)
        }
        if !names.contains(&b) {
            names.push(b)
        }
    }
    (map, names)
}

fn eval(map: &HashMap<(&str, &str), i64>, names: &[&str]) -> i64 {
    let mut hap = 0;
    let n = names.len();

    for i in 0..n {
        let a = names[i];
        let b = names[(i + 1) % n];
        // Add happiness scoe in both directions
        hap += map.get(&(a, b)).unwrap();
        hap += map.get(&(b, a)).unwrap();
    }
    hap
}

fn solve(map: &HashMap<(&str, &str), i64>, names: &[&str]) -> i64 {
    let mut max_hap = i64::MIN;
    // keep one element fixed to due to symmetry for performance gains
    let first = names[0];
    let rest = &names[1..];
    for perm in rest.iter().copied().permutations(rest.len()) {
        let mut full = Vec::with_capacity(names.len());
        full.push(first);
        full.extend(perm);
        let hap = eval(map, &full);
        max_hap = max_hap.max(hap);
    }
    max_hap
}

pub fn part_one(input: &str) -> Option<i64> {
    let (map, names) = parse_input(input);
    Some(solve(&map, &names))
}

pub fn part_two(input: &str) -> Option<i64> {
    let (mut map, mut names) = parse_input(input);
    // Add "You"
    for n in names.iter() {
        map.insert(("You", n), 0);
        map.insert((n, "You"), 0);
    }
    names.push("You");
    Some(solve(&map, &names))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(330));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(286));
    }
}
