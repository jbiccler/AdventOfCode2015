advent_of_code::solution!(16);

const TARGET_AUNT: Aunt = Aunt {
    id: 0,
    children: Some(3),
    cats: Some(7),
    samoyeds: Some(2),
    pomeranians: Some(3),
    akitas: Some(0),
    vizslas: Some(0),
    goldfish: Some(5),
    trees: Some(3),
    cars: Some(2),
    perfumes: Some(1),
};

#[derive(Debug, Clone, Copy)]
struct Aunt {
    id: u64,
    children: Option<u64>,
    cats: Option<u64>,
    samoyeds: Option<u64>,
    pomeranians: Option<u64>,
    akitas: Option<u64>,
    vizslas: Option<u64>,
    goldfish: Option<u64>,
    trees: Option<u64>,
    cars: Option<u64>,
    perfumes: Option<u64>,
}

#[inline(always)]
fn check_plausible(a: Option<u64>, target: Option<u64>) -> bool {
    match a {
        None => true,
        Some(v) => v == target.unwrap(),
    }
}

#[inline(always)]
fn check_plausible_gt(a: Option<u64>, target: Option<u64>) -> bool {
    match a {
        None => true,
        Some(v) => v > target.unwrap(),
    }
}

#[inline(always)]
fn check_plausible_lt(a: Option<u64>, target: Option<u64>) -> bool {
    match a {
        None => true,
        Some(v) => v < target.unwrap(),
    }
}

impl PartialEq for Aunt {
    fn eq(&self, other: &Self) -> bool {
        check_plausible(self.children, other.children)
            && check_plausible(self.cats, other.cats)
            && check_plausible(self.samoyeds, other.samoyeds)
            && check_plausible(self.pomeranians, other.pomeranians)
            && check_plausible(self.akitas, other.akitas)
            && check_plausible(self.vizslas, other.vizslas)
            && check_plausible(self.goldfish, other.goldfish)
            && check_plausible(self.trees, other.trees)
            && check_plausible(self.cars, other.cars)
            && check_plausible(self.perfumes, other.perfumes)
    }
}

impl Aunt {
    fn part_two_cmp(&self, other: &Self) -> bool {
        // pomeranians and goldfish shoud be less than other
        // cats and trees should be greate than other
        check_plausible(self.children, other.children)
            && check_plausible_gt(self.cats, other.cats)
            && check_plausible(self.samoyeds, other.samoyeds)
            && check_plausible_lt(self.pomeranians, other.pomeranians)
            && check_plausible(self.akitas, other.akitas)
            && check_plausible(self.vizslas, other.vizslas)
            && check_plausible_lt(self.goldfish, other.goldfish)
            && check_plausible_gt(self.trees, other.trees)
            && check_plausible(self.cars, other.cars)
            && check_plausible(self.perfumes, other.perfumes)
    }
}

fn parse_input(input: &str) -> Vec<Aunt> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim())
        .map(|l| {
            let (sue, parts) = l.split_once(": ").unwrap();
            let (_, id) = sue.split_once(" ").unwrap();
            let id = id.parse::<u64>().unwrap();

            // Initialize variables
            let mut children: Option<u64> = None;
            let mut cats: Option<u64> = None;
            let mut samoyeds: Option<u64> = None;
            let mut pomeranians: Option<u64> = None;
            let mut akitas: Option<u64> = None;
            let mut vizslas: Option<u64> = None;
            let mut goldfish: Option<u64> = None;
            let mut trees: Option<u64> = None;
            let mut cars: Option<u64> = None;
            let mut perfumes: Option<u64> = None;

            for p in parts.split(", ") {
                let (typ, val) = p.split_once(": ").unwrap();
                match typ {
                    "children" => children = val.parse::<u64>().ok(),
                    "cats" => cats = val.parse::<u64>().ok(),
                    "samoyeds" => samoyeds = val.parse::<u64>().ok(),
                    "pomeranians" => pomeranians = val.parse::<u64>().ok(),
                    "akitas" => akitas = val.parse::<u64>().ok(),
                    "vizslas" => vizslas = val.parse::<u64>().ok(),
                    "goldfish" => goldfish = val.parse::<u64>().ok(),
                    "trees" => trees = val.parse::<u64>().ok(),
                    "cars" => cars = val.parse::<u64>().ok(),
                    "perfumes" => perfumes = val.parse::<u64>().ok(),
                    _ => panic!(),
                }
            }
            Aunt {
                id,
                children,
                cats,
                samoyeds,
                pomeranians,
                akitas,
                vizslas,
                goldfish,
                trees,
                cars,
                perfumes,
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let aunts = parse_input(input);
    for a in aunts {
        if a == TARGET_AUNT {
            return Some(a.id);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let aunts = parse_input(input);
    for a in aunts {
        if a.part_two_cmp(&TARGET_AUNT) {
            return Some(a.id);
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
