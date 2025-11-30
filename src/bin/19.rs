use rand::prelude::*;
use std::collections::HashSet;

advent_of_code::solution!(19);

fn parse_input(input: &str) -> (Vec<(&str, &str)>, &str) {
    let (rules_str, target) = input.split_once("\n\n").unwrap();
    let target = target.trim();
    let mut rules = vec![];
    for rule in rules_str.lines() {
        let rule = rule.trim();
        let (from, to) = rule.split_once(" => ").unwrap();
        rules.push((from, to));
    }

    (rules, target)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (rules, target) = parse_input(input);
    let mut set: HashSet<String> = HashSet::new();
    // Apply each rule once
    for (from, to) in rules {
        for (idx, _) in target.match_indices(from) {
            let mut new = String::with_capacity(target.len() + to.len() - from.len());
            new.push_str(&target[..idx]);
            new.push_str(to);
            new.push_str(&target[idx + from.len()..]);
            set.insert(new);
        }
    }
    Some(set.len())
}

fn greedy_backtrace(rules: &[(&str, &str)], target: &str, start: &str) -> Option<usize> {
    // Go in reverse order from target to start by greedily applying rules in reverse
    let mut n_steps = 0;
    let mut s = target.to_owned();
    while s != start {
        let mut applied_rule = false;
        for (from, to) in rules {
            if let Some(idx) = s.find(to) {
                s.replace_range(idx..idx + to.len(), from);
                n_steps += 1;
                applied_rule = true;
            }
        }
        if !applied_rule {
            return None;
        }
    }
    Some(n_steps)
}

fn length_diff(rule: &(&str, &str)) -> usize {
    rule.1.len() - rule.0.len()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut rules, target) = parse_input(input);
    let start = "e";
    // Sort to start looking at rules with biggest reduction in length
    rules.sort_unstable_by_key(|a| std::cmp::Reverse(length_diff(a)));
    let mut rng = rand::rng();
    loop {
        let n_steps = greedy_backtrace(&rules, target, start);
        // If greedy backtrace didn't find a solution, randomize input order of rules and retry
        if n_steps.is_some() {
            return n_steps;
        }
        rules.shuffle(&mut rng);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(3));
    }
}
