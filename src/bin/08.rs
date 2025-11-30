use advent_of_code::utils::input::lines;
use regex::Regex;
use std::sync::LazyLock;
advent_of_code::solution!(8);

static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"\\x[0-9a-fA-F]{2}|\\."#).unwrap());

fn calc_length(line: &str) -> usize {
    let code_len = line.len();
    let inner = &line[1..line.len() - 1]; // strip quotes
    let mut mem_len = code_len - 2;

    for m in RE.find_iter(inner) {
        let tok = m.as_str();
        if tok.starts_with(r"\x") {
            mem_len = mem_len.saturating_sub(3); // hex escape  4 chars are actually 1
        } else if tok.starts_with('\\') {
            mem_len = mem_len.saturating_sub(1); // escaped slash or quote -> 1 char
        }
    }
    mem_len
}

pub fn part_one(input: &str) -> Option<usize> {
    let lines = lines(input);
    Some(
        lines
            .iter()
            .map(|l| l.len().saturating_sub(calc_length(l)))
            .sum(),
    )
}

fn encoded_length(line: &str) -> usize {
    // start from the two outside quotes
    let mut count = 2;
    for c in line.chars() {
        match c {
            '"' | '\\' => count += 2,
            _ => count += 1,
        }
    }
    count
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines = lines(input);
    Some(
        lines
            .iter()
            .map(|l| encoded_length(l).saturating_sub(l.len()))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19));
    }
}
