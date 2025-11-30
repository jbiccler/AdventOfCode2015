use std::collections::HashMap;

advent_of_code::solution!(5);

// Part one rules

fn three_vowels(s: &str) -> bool {
    s.chars()
        .map(|c| match c {
            'a' | 'u' | 'i' | 'e' | 'o' => 1,
            _ => 0,
        })
        .sum::<i32>()
        >= 3
}

fn duplicates(s: &str) -> bool {
    // Assumes only ASCII
    s.as_bytes().windows(2).any(|w| w[0] == w[1])
}

fn doesnt_contain_sets(s: &str) -> bool {
    let sets = ["ab", "cd", "pq", "xy"];
    for set in sets {
        if s.contains(set) {
            return false;
        }
    }
    true
}

// Part 2 rules

fn duplicates_split_by_one(s: &str) -> bool {
    // Assumes only ASCII
    s.as_bytes().windows(3).any(|w| w[0] == w[2])
}

fn pairs_non_overlapping(s: &str) -> bool {
    let mut map: HashMap<(u8, u8), usize> = HashMap::new();

    for (i, w) in s.as_bytes().windows(2).enumerate() {
        let pair = (w[0], w[1]);
        if let Some(prev_idx) = map.insert(pair, i)
            && prev_idx + 2 <= i
        {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut count = 0;
    for line in input.lines() {
        let line = line.trim();
        if !duplicates(line) {
            continue;
        }
        if !three_vowels(line) {
            continue;
        }
        if !doesnt_contain_sets(line) {
            continue;
        }
        count += 1
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut count = 0;
    for line in input.lines() {
        let line = line.trim();
        if !duplicates_split_by_one(line) {
            continue;
        }
        if !pairs_non_overlapping(line) {
            continue;
        }
        count += 1
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
