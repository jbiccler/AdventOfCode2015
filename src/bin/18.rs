advent_of_code::solution!(18);

pub fn parse_input(input: &str) -> (Vec<bool>, usize, usize) {
    let grid: Vec<Vec<bool>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim().chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect();
    let nx = grid[0].len();
    let ny = grid.len();
    (grid.into_iter().flatten().collect(), nx, ny)
}

#[derive(PartialEq)]
enum Part {
    One,
    Two,
}

#[inline(always)]
fn idx(x: usize, y: usize, nx: usize) -> usize {
    y * nx + x
}

#[inline(always)]
fn count_neighbors(grid: &[bool], x: usize, y: usize, nx: usize, ny: usize) -> u8 {
    // Manually unrolled for speed
    let mut c = 0u8;

    let y0 = y > 0;
    let y1 = y + 1 < ny;
    let x0 = x > 0;
    let x1 = x + 1 < nx;

    if y0 {
        let yy = y - 1;
        if x0 && grid[idx(x - 1, yy, nx)] {
            c += 1;
        }
        if grid[idx(x, yy, nx)] {
            c += 1;
        }
        if x1 && grid[idx(x + 1, yy, nx)] {
            c += 1;
        }
    }

    {
        let yy = y;
        if x0 && grid[idx(x - 1, yy, nx)] {
            c += 1;
        }
        if x1 && grid[idx(x + 1, yy, nx)] {
            c += 1;
        }
    }

    if y1 {
        let yy = y + 1;
        if x0 && grid[idx(x - 1, yy, nx)] {
            c += 1;
        }
        if grid[idx(x, yy, nx)] {
            c += 1;
        }
        if x1 && grid[idx(x + 1, yy, nx)] {
            c += 1;
        }
    }

    c
}

fn animate(grid: &mut Vec<bool>, nx: usize, ny: usize, n_iter: usize, part: Part) -> usize {
    let corners = [0, nx - 1, (ny - 1) * nx, (ny - 1) * nx + (nx - 1)];
    if part == Part::Two {
        // Fix corners always on
        for i in corners {
            grid[i] = true;
        }
    }
    let mut next = vec![false; nx * ny];

    for _ in 0..n_iter {
        for x in 0..nx {
            for y in 0..ny {
                let idx = idx(x, y, nx);
                // count number of neighbors that are on
                let n = count_neighbors(grid, x, y, nx, ny);
                // Toggle on/off
                let on = grid[idx];
                next[idx] = (on && (n == 2 || n == 3)) || (!on && n == 3);
            }
        }
        if part == Part::Two {
            // Fix corners always on
            for i in corners {
                next[i] = true;
            }
        }
        std::mem::swap(grid, &mut next);
    }
    grid.iter().filter(|&&x| x).count()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut grid, nx, ny) = parse_input(input);
    Some(animate(&mut grid, nx, ny, 100, Part::One))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut grid, nx, ny) = parse_input(input);
    Some(animate(&mut grid, nx, ny, 100, Part::Two))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let (mut grid, nx, ny) = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let result = animate(&mut grid, nx, ny, 4, Part::One);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_two() {
        let (mut grid, nx, ny) = parse_input(&advent_of_code::template::read_file("examples", DAY));
        let result = animate(&mut grid, nx, ny, 5, Part::Two);
        assert_eq!(result, 17);
    }
}
