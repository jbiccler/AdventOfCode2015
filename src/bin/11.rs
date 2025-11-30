advent_of_code::solution!(11);

fn increment_password(pw: &mut [u8]) {
    // base 26
    for i in (0..pw.len()).rev() {
        if pw[i] == b'z' {
            pw[i] = b'a';
        } else {
            pw[i] += 1;
            break;
        }
    }
}

fn straight(pw: &[u8]) -> bool {
    pw.windows(3).any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2])
}

fn valid_letters(pw: &[u8]) -> bool {
    !pw.iter().any(|&c| c == b'i' || c == b'l' || c == b'o')
}

fn pairs(pw: &[u8]) -> bool {
    let mut prev: Option<u8> = None;
    for w in pw.windows(2) {
        if w[0] == w[1] {
            match prev {
                None => prev = Some(w[0]),
                Some(p) => {
                    if p != w[0] {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn validate_password(pw: &[u8]) -> bool {
    valid_letters(pw) && straight(pw) && pairs(pw)
}

pub fn part_one(input: &str) -> Option<String> {
    let mut pw = input.trim().as_bytes().to_owned();
    loop {
        increment_password(&mut pw);
        if validate_password(&pw) {
            return Some(String::from_utf8(pw).unwrap());
        }
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let first = part_one(input);
    part_one(&first.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        let input = "hijklmmz";
        let mut bytes = input.as_bytes().to_owned();
        increment_password(&mut bytes);
        assert_eq!(bytes, "hijklmna".as_bytes().to_owned())
    }
    #[test]
    fn test_valid_pass() {
        let input = "haabcmmz";
        let bytes = input.as_bytes().to_owned();
        let result = validate_password(&bytes);
        assert!(result);
    }
    #[test]
    fn test_invalid_pass() {
        let input = "haabdmmz";
        let bytes = input.as_bytes().to_owned();
        let result = validate_password(&bytes);
        assert!(!result);

        let input = "haabcaaz";
        let bytes = input.as_bytes().to_owned();
        let result = validate_password(&bytes);
        assert!(!result);

        let input = "haabcmmi";
        let bytes = input.as_bytes().to_owned();
        let result = validate_password(&bytes);
        assert!(!result);
    }
}
