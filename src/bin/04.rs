advent_of_code::solution!(4);

fn get_hash(secret: &str, digit: usize) -> md5::Digest {
    md5::compute(format!("{secret}{digit}"))
}

fn has_leading_zeros(digest: md5::Digest, n_zeros: usize) -> bool {
    let bytes = digest.0;
    let full_zero_bytes = n_zeros / 2;
    let remaining_half = n_zeros % 2;

    for &b in bytes[0..full_zero_bytes].iter() {
        if b != 0 {
            return false;
        }
    }
    if (remaining_half == 1) & (bytes[full_zero_bytes] >> 4 != 0) {
        return false;
    }
    true
}

pub fn part_one(input: &str) -> Option<usize> {
    let secret = input.trim();
    let mut i = 1;
    loop {
        let digest = get_hash(secret, i);
        if has_leading_zeros(digest, 5) {
            return Some(i);
        }
        i += 1;
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let secret = input.trim();
    let mut i = 1;
    loop {
        let digest = get_hash(secret, i);
        if has_leading_zeros(digest, 6) {
            return Some(i);
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(609043));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6742839));
    }
}
