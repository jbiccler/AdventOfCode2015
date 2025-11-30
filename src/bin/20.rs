advent_of_code::solution!(20);

fn parse_input(input: &str) -> usize {
    input.trim().parse().unwrap()
}

fn solve_part1(target: usize) -> Option<usize> {
    // Heuristic bound loop
    for heuristic in [40, 20, 10] {
        // Heuristic for max house
        let max = target / heuristic;
        let mut houses = vec![0usize; max + 1];

        for elf in 1..=max {
            // houses visited by this elf
            let mut house = elf;
            while house <= max {
                houses[house] += elf * 10;
                // next house to visit by this elf
                house += elf;
            }
        }

        let res = houses.iter().position(|&h| h >= target);
        if res.is_some() {
            return res;
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let target = parse_input(input);
    solve_part1(target)
}

fn solve_part2(target: usize, max_visits: usize) -> Option<usize> {
    // Heuristic bound loop
    for heuristic in [40, 20, 11] {
        // Heuristic for max house
        let max = target / heuristic;
        let mut houses = vec![0usize; max + 1];

        for elf in 1..=max {
            // houses visited by this elf
            let mut house = elf;
            let mut visited = 0;
            while house <= max && visited < max_visits {
                houses[house] += elf * 11;
                // next house to visit by this elf
                house += elf;
                visited += 1;
            }
        }

        let res = houses.iter().position(|&h| h >= target);
        if res.is_some() {
            return res;
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let target = parse_input(input);
    solve_part2(target, 50)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
}
