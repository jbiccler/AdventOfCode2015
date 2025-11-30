advent_of_code::solution!(25);

const DIVISOR: usize = 252533;
const MOD: usize = 33554393;
const START_VALUE: usize = 20151125;

fn parse_input(input: &str) -> (usize, usize) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    //15
    let row = parts[15].trim_end_matches(",").parse::<usize>().unwrap();
    let col = parts[parts.len() - 1]
        .trim_end_matches(".")
        .parse::<usize>()
        .unwrap();
    (row, col)
}

#[inline(always)]
fn next_val(prev: usize) -> usize {
    (prev * DIVISOR) % MOD
}

#[inline(always)]
fn next_idx(row: usize, col: usize) -> (usize, usize) {
    // diagonally
    if row == 1 {
        return (col + 1, 1);
    }
    (row.saturating_sub(1), col + 1)
}

fn solve(
    target_row: usize,
    target_col: usize,
    start_row: usize,
    start_col: usize,
    start_value: usize,
) -> usize {
    debug_assert!(target_row + target_col >= start_row + start_col);
    let (mut current_row, mut current_col) = (start_row, start_col);
    let mut current_val = start_value;
    while (current_row != target_row) || (current_col != target_col) {
        (current_row, current_col) = next_idx(current_row, current_col);
        current_val = next_val(current_val);
    }
    current_val
}

pub fn part_one(input: &str) -> Option<usize> {
    let (target_row, target_col) = parse_input(input);
    Some(solve(target_row, target_col, 1, 1, START_VALUE))
}

pub fn part_two(_input: &str) -> Option<u64> {
    // No part two for the final day
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33071741));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
