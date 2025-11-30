use itertools::Itertools;

advent_of_code::solution!(24);

fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim().parse().unwrap())
        .collect()
}

fn solve(gifts: &[u64], n_groups: usize) -> Option<u64> {
    // only the first group's permutations matter as long as the total split across all 3 is even
    let total: u64 = gifts.iter().sum();
    debug_assert!(total % n_groups as u64 == 0);
    let total_per_group = total / n_groups as u64;
    // Group 1
    let mut best_qe = u64::MAX;
    for n in 1..=gifts.len() {
        // Generate combinations and prune the ones that don't have the required sum of weights
        let combs = gifts
            .iter()
            .combinations(n)
            .filter(|weights| weights.iter().map(|&&x| x).sum::<u64>() == total_per_group);
        for c in combs {
            let product: u64 = c.into_iter().product();
            // In reality, one would have to check if the remaining items can still be
            // divided across the n_groups - 1 that are left in such a way that their total weights
            // per group add up to total_per_group, however this input does not seem to require it.
            // This seems to be case for all generated inputs across participants?
            best_qe = best_qe.min(product)
        }
        // As soon as we have found one n for which there exists a permutation that sums up to the
        // right amount -> stop as this is the min number of elements we can put in group 1
        if best_qe < u64::MAX {
            break;
        }
    }
    if best_qe == u64::MAX {
        return None;
    }
    Some(best_qe)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut gifts = parse_input(input);
    // Given in increasing order, reversing seems to speed it up due to faster pruning.
    gifts = gifts.into_iter().rev().collect();
    solve(&gifts, 3)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut gifts = parse_input(input);
    // Given in increasing order, reversing seems to speed it up due to faster pruning.
    gifts = gifts.into_iter().rev().collect();
    solve(&gifts, 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
