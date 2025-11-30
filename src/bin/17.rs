advent_of_code::solution!(17);

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim().parse::<i32>().unwrap())
        .collect()
}

/// Solves both part one and two in one loop
fn solve(containers: &[i32], target: i32) -> (usize, usize) {
    let n = containers.len();
    let mut total_valid = 0usize;
    let mut min_used = usize::MAX;
    let mut min_used_count = 0;
    // Max number of combinations is 2^n
    let limit = 1usize << n;

    // Bit sets represent chosen subset of containers
    for mask in 0..limit {
        let mut m = mask;
        let mut sum = 0;
        let mut used = 0;

        // Iterate over the lowest set bits, iteratively turning off the lowest bit checked
        while m != 0 {
            // Get index of lowest set bit
            let idx = m.trailing_zeros() as usize;
            sum += containers[idx];
            if sum > target {
                break;
            }
            used += 1;
            // Bit trick to remove lowest bit
            // m - 1 flips all bit starting from the lowest 1 bit
            // m & (m = 1) then sets it to zero
            m &= m - 1;
        }
        if sum == target {
            total_valid += 1;
            if used < min_used {
                min_used = used;
                min_used_count = 1;
            } else if used == min_used {
                min_used_count += 1;
            }
        }
    }
    (total_valid, min_used_count)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut containers = parse_input(input);
    containers.sort_unstable_by(|a, b| b.cmp(a));
    let (total, _) = solve(&containers, 150);
    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut containers = parse_input(input);
    containers.sort_unstable_by(|a, b| b.cmp(a));
    let (_, count) = solve(&containers, 150);
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let containers = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let (result, _) = solve(&containers, 25);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_two() {
        let containers = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let (_, result) = solve(&containers, 25);
        assert_eq!(result, 3);
    }
}
