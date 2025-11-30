advent_of_code::solution!(1);

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!(),
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<i64> {
    let v = parse_input(input);
    Some(v.iter().sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let v = parse_input(input);
    let mut counter: i64 = 0;
    for (i, &x) in v.iter().enumerate() {
        counter += x;
        if counter == -1 {
            // Indexing starts from 1
            return Some(i + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(-1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
}
