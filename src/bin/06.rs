use regex::Regex;
advent_of_code::solution!(6);

#[derive(Debug)]
enum Operation {
    On,
    Off,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    op: Operation,
    corner1: (usize, usize),
    corner2: (usize, usize),
}
impl Instruction {
    fn new(op: &str, frx: &str, fry: &str, tox: &str, toy: &str) -> Instruction {
        Instruction {
            op: match op {
                "turn on" => Operation::On,
                "turn off" => Operation::Off,
                "toggle" => Operation::Toggle,
                _ => panic!(),
            },
            corner1: (frx.parse().unwrap(), fry.parse().unwrap()),
            corner2: (tox.parse().unwrap(), toy.parse().unwrap()),
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"([\w\s]+) (\d+),(\d+) through (\d+),(\d+)\n").unwrap();
    let mut res: Vec<Instruction> = Vec::with_capacity(input.lines().count());
    for (_, [op, frx, fry, tox, toy]) in re.captures_iter(input).map(|c| c.extract()) {
        res.push(Instruction::new(op, frx, fry, tox, toy))
    }
    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let instructions = parse_input(input);
    let mut grid = vec![vec![false; 1_000]; 1_000];
    for ins in instructions {
        match ins.op {
            Operation::On => {
                for row in grid[ins.corner1.0..=ins.corner2.0].iter_mut() {
                    for el in row[ins.corner1.1..=ins.corner2.1].iter_mut() {
                        *el = true;
                    }
                }
            }
            Operation::Off => {
                for row in grid[ins.corner1.0..=ins.corner2.0].iter_mut() {
                    for el in row[ins.corner1.1..=ins.corner2.1].iter_mut() {
                        *el = false;
                    }
                }
            }
            Operation::Toggle => {
                for row in grid[ins.corner1.0..=ins.corner2.0].iter_mut() {
                    for el in row[ins.corner1.1..=ins.corner2.1].iter_mut() {
                        *el = !*el;
                    }
                }
            }
        }
    }
    Some(
        grid.iter()
            .map(|x| x.iter().map(|&y| if y { 1 } else { 0 }).sum::<u64>())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let instructions = parse_input(input);
    let mut grid: Vec<Vec<u8>> = vec![vec![0; 1_000]; 1_000];
    for ins in instructions {
        match ins.op {
            Operation::On => {
                for row in grid[ins.corner1.0..=ins.corner2.0].iter_mut() {
                    for el in row[ins.corner1.1..=ins.corner2.1].iter_mut() {
                        *el += 1;
                    }
                }
            }
            Operation::Off => {
                for row in grid[ins.corner1.0..=ins.corner2.0].iter_mut() {
                    for el in row[ins.corner1.1..=ins.corner2.1].iter_mut() {
                        *el = el.saturating_sub(1);
                    }
                }
            }
            Operation::Toggle => {
                for row in grid[ins.corner1.0..=ins.corner2.0].iter_mut() {
                    for el in row[ins.corner1.1..=ins.corner2.1].iter_mut() {
                        *el += 2;
                    }
                }
            }
        }
    }
    Some(
        grid.iter()
            .map(|x| x.iter().map(|&x| x as u64).sum::<u64>())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
