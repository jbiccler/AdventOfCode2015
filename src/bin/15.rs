advent_of_code::solution!(15);

#[derive(Debug, Clone)]
struct Ingredient {
    _name: String,
    cap: i64,
    dur: i64,
    fla: i64,
    tex: i64,
    cal: i64,
}

#[derive(Debug, Clone, Copy)]
struct Score {
    cap: i64,
    dur: i64,
    fla: i64,
    tex: i64,
    cal: i64,
}

impl Score {
    fn new() -> Self {
        Self {
            cap: 0,
            dur: 0,
            fla: 0,
            tex: 0,
            cal: 0,
        }
    }
    fn add(&mut self, ing: &Ingredient, x: i64) {
        self.cap += ing.cap * x;
        self.dur += ing.dur * x;
        self.fla += ing.fla * x;
        self.tex += ing.tex * x;
        self.cal += ing.cal * x;
    }
    fn score(&self) -> i64 {
        let a = self.cap.max(0);
        let b = self.dur.max(0);
        let c = self.fla.max(0);
        let d = self.tex.max(0);
        a * b * c * d
    }
}

fn parse_input(input: &str) -> Vec<Ingredient> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim())
        .map(|l| {
            let (name, properties) = l.split_once(": ").expect("Invalid syntax");
            let parts: Vec<&str> = properties.split(", ").collect();
            let mut props: Vec<i64> = Vec::with_capacity(5);
            debug_assert!(parts.len() == 5);
            for p in parts {
                let (_prop, val) = p.split_once(" ").expect("Invalid property syntax");
                props.push(val.parse().unwrap());
            }
            Ingredient {
                _name: name.to_owned(),
                cap: props[0],
                dur: props[1],
                fla: props[2],
                tex: props[3],
                cal: props[4],
            }
        })
        .collect()
}

fn upper_bound(remaining: i64, cur: &Score, ings: &[Ingredient]) -> i64 {
    // Loose upper bound, find max value of each ingredient and assign remaining to that
    // Actually not beneficial to precompute and store in vector for such small inputs (benchmarked), so
    // recalculated every time here
    let best_cap = ings.iter().map(|i| i.cap).max().unwrap_or(0).max(0);
    let best_dur = ings.iter().map(|i| i.dur).max().unwrap_or(0).max(0);
    let best_fla = ings.iter().map(|i| i.fla).max().unwrap_or(0).max(0);
    let best_tex = ings.iter().map(|i| i.tex).max().unwrap_or(0).max(0);

    let cap = (cur.cap + best_cap * remaining).max(0);
    let dur = (cur.dur + best_dur * remaining).max(0);
    let fla = (cur.fla + best_fla * remaining).max(0);
    let tex = (cur.tex + best_tex * remaining).max(0);

    cap * dur * fla * tex
}

fn dfs(
    idx: usize,
    remaining: i64,
    cur: Score,
    ings: &[Ingredient],
    best: &mut i64,
    calorie_target: Option<i64>,
) {
    let n = ings.len();

    if idx >= n - 1 {
        // Last ingredient, forced assignment
        let mut res = cur; // make mutable
        let amt = remaining;
        res.add(&ings[idx], amt);
        if let Some(target) = calorie_target {
            if res.cal == target {
                // Part 2 case
                *best = (*best).max(res.score());
            }
        } else {
            // Part 1 case
            *best = (*best).max(res.score());
        }
        return;
    }
    // Branch & Bound
    if upper_bound(remaining, &cur, &ings[idx..]) < *best {
        // Skip remainig branches
        return;
    }
    // Iterate over possible amounts
    for amt in 0..=remaining {
        let mut next = cur;
        next.add(&ings[idx], amt);
        dfs(idx + 1, remaining - amt, next, ings, best, calorie_target);
    }
}

fn solve(ings: &[Ingredient], total: i64, calorie_target: Option<i64>) -> u64 {
    let mut best = 0;
    let cur = Score::new();
    dfs(0, total, cur, ings, &mut best, calorie_target);
    best as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let ingredients = parse_input(input);
    Some(solve(&ingredients, 100, None))
}

pub fn part_two(input: &str) -> Option<u64> {
    let ingredients = parse_input(input);
    Some(solve(&ingredients, 100, Some(500)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62842880));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(57600000));
    }
}
