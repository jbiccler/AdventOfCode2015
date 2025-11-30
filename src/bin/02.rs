use advent_of_code::utils::input::split_parse_lines;
advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    split_parse_lines(input, 'x')
}

fn surface_area(v: Vec<u64>) -> (u64, u64) {
    assert!(v.len() == 3);
    let (l, w, h) = (v[0], v[1], v[2]);
    let sides = [l * w, w * h, h * l];
    (
        sides.into_iter().fold(0, |acc, e| acc + e * 2),
        sides.into_iter().min().unwrap(),
    )
}

fn ribbon(v: Vec<u64>) -> (u64, u64) {
    assert!(v.len() == 3);
    let (l, w, h) = (v[0], v[1], v[2]);
    let sides = [l + w, w + h, h + l];
    (l * w * h, sides.into_iter().min().unwrap())
}

pub fn part_one(input: &str) -> Option<u64> {
    let v = parse_input(input);
    let mut sum = 0;
    for x in v {
        let (sf, m) = surface_area(x);
        sum += sf + m;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let v = parse_input(input);
    let mut sum = 0;
    for x in v {
        let (vol, r) = ribbon(x);
        sum += vol + r * 2;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(58 + 43));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34 + 14));
    }
}
