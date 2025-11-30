advent_of_code::solution!(10);

fn look_and_say(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 2);

    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        let mut counter = 1;
        while let Some(&next) = chars.peek() {
            if next == c {
                counter += 1;
                chars.next();
            } else {
                break;
            }
        }
        out.push_str(&counter.to_string());
        out.push(c);
    }
    out
}

fn solve(input: &str, n: usize) -> String {
    let mut out = input.trim().to_owned();
    for _ in 0..n {
        out = look_and_say(&out);
    }
    out
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, 40).len())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, 50).len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_look_say() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 1);
        assert_eq!(result, "312211".to_string());
    }
}
