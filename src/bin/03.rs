use std::collections::HashMap;
advent_of_code::solution!(3);

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    let line = input.lines().next().unwrap().trim();
    line.chars()
        .map(|c| match c {
            '>' => (1, 0),
            '<' => (-1, 0),
            '^' => (0, 1),
            'v' => (0, -1),
            _ => panic!(),
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let dirs = parse_input(input);
    let mut pos: HashMap<(i64, i64), usize> = HashMap::new();
    let (mut cx, mut cy) = (0, 0);
    // Add start position
    pos.insert((cx, cy), 1);
    for (dx, dy) in dirs {
        cx += dx;
        cy += dy;
        pos.entry((cx, cy))
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    Some(pos.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let dirs = parse_input(input);
    let mut pos: HashMap<(i64, i64), usize> = HashMap::new();
    // Santa
    let (mut sx, mut sy) = (0, 0);
    // Robot
    let (mut rx, mut ry) = (0, 0);
    // Add start position
    pos.insert((sx, sy), 1);
    // Santa flag
    let mut santa: bool = true;
    for (dx, dy) in dirs {
        if santa {
            sx += dx;
            sy += dy;
            pos.entry((sx, sy))
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        } else {
            rx += dx;
            ry += dy;
            pos.entry((rx, ry))
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        santa = !santa;
    }
    Some(pos.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }
}
