advent_of_code::solution!(14);

#[derive(Debug, Clone)]
struct Reindeer {
    _name: String,
    speed: u64,
    travel_time: u64,
    rest_time: u64,
}

#[derive(Debug, Clone)]
struct State {
    r: Reindeer,
    resting: bool,
    travel_time_left: u64,
    rest_time_left: u64,
    distance_covered: u64,
    score: u64,
}

fn parse_input(input: &str) -> Vec<Reindeer> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim())
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            let name = parts[0].to_string();
            let speed = parts[3].parse().unwrap();
            let travel_time = parts[6].parse().unwrap();
            let rest_time = parts[parts.len() - 2].parse().unwrap();
            Reindeer {
                _name: name,
                speed,
                travel_time,
                rest_time,
            }
        })
        .collect()
}

fn calc_distance(r: &Reindeer, target: u64) -> u64 {
    let total_time = r.travel_time + r.rest_time;
    let n_whole = target / total_time;
    let remainder = target % total_time;
    let mut result = r.speed * n_whole * r.travel_time;
    if remainder <= r.travel_time {
        result += r.speed * remainder;
    } else {
        result += r.travel_time * r.speed;
    }
    result
}

fn solve_max_distance(v: &[Reindeer], target: u64) -> Option<u64> {
    v.iter().map(|r| calc_distance(r, target)).max()
}

pub fn part_one(input: &str) -> Option<u64> {
    let reindeer = parse_input(input);
    solve_max_distance(&reindeer, 2503)
}

fn score_reindeer(v: &[Reindeer], target: u64) -> Option<u64> {
    let mut states: Vec<State> = v
        .iter()
        .map(|r| State {
            r: r.clone(),
            travel_time_left: r.travel_time,
            rest_time_left: r.rest_time,
            distance_covered: 0,
            resting: false,
            score: 0,
        })
        .collect();

    for _ in 0..target {
        let mut max_dist = 0;
        for s in states.iter_mut() {
            // progress the reindeer state by 1 second
            match s.resting {
                false => {
                    s.travel_time_left = s.travel_time_left.saturating_sub(1);
                    s.distance_covered += s.r.speed;
                    // reset time and toggle resting
                    if s.travel_time_left == 0 {
                        s.resting = true;
                        s.rest_time_left = s.r.rest_time;
                    }
                }
                true => {
                    s.rest_time_left = s.rest_time_left.saturating_sub(1);
                    // reset time and toggle resting
                    if s.rest_time_left == 0 {
                        s.resting = false;
                        s.travel_time_left = s.r.travel_time;
                    }
                }
            }
            // Track max distance
            max_dist = max_dist.max(s.distance_covered);
        }
        // Find reindeer that have covered max distance and increment score by one.
        // If multiple reindeer share same max distance covered, each should get one point.
        states
            .iter_mut()
            .filter(|s| s.distance_covered == max_dist)
            .for_each(|s| s.score += 1);
    }
    states.iter().map(|s| s.score).max()
}

pub fn part_two(input: &str) -> Option<u64> {
    let reindeer = parse_input(input);
    score_reindeer(&reindeer, 2503)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let reindeer = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let result = solve_max_distance(&reindeer, 1000);
        assert_eq!(result, Some(1120));
    }

    #[test]
    fn test_part_two() {
        let reindeer = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let result = score_reindeer(&reindeer, 1000);
        assert_eq!(result, Some(689));
    }
}
