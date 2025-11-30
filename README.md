<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2015

Solutions for [Advent of Code 2015 (AOC)](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

This repo is based on this [AOC Rust template](https://github.com/fspoettel/advent-of-code-rust), please refer to this page for information on how to run the solutions.

## Benchmarks

As per the benchmarks referenced below, all days can be solved in less than a second on a i7-13700k @ 5.3 GHz.
Part 1 and 2 of each day were implemented to be self contained, meaning that both parts will separately parse the input and can be ran individually.
This however implies redundant input parsing when all days and parts are benchmarked together, as you parse the same input twice per day.
Lastly, when both parts could be solved simultaneously (e.g. day 21), the total time required to solve the problem is effectively duplicated in the table below.

<!--- benchmarking table --->

| Day | Part 1 | Part 2 |
| :---: | :---: | :---:  |
| [Day 1](./src/bin/01.rs) | `11.3µs` | `11.2µs` |
| [Day 2](./src/bin/02.rs) | `46.8µs` | `46.4µs` |
| [Day 3](./src/bin/03.rs) | `205.2µs` | `207.5µs` |
| [Day 4](./src/bin/04.rs) | `12.7ms` | `424.4ms` |
| [Day 5](./src/bin/05.rs) | `57.7µs` | `130.8µs` |
| [Day 6](./src/bin/06.rs) | `1.3ms` | `.3ms` |
| [Day 7](./src/bin/07.rs) | `76.1µs` | `101.7µs` |
| [Day 8](./src/bin/08.rs) | `24.1µs` | `7.0µs` |
| [Day 9](./src/bin/09.rs) | `2.7ms` | `2.7ms` |
| [Day 10](./src/bin/10.rs) | `6.5ms` | `92.1ms` |
| [Day 11](./src/bin/11.rs) | `83.8µs` | `4.0ms` |
| [Day 12](./src/bin/12.rs) | `216.1µs` | `205.1µs` |
| [Day 13](./src/bin/13.rs) | `1.5ms` | `12.6ms` |
| [Day 14](./src/bin/14.rs) | `1.8µs` | `24.5µs` |
| [Day 15](./src/bin/15.rs) | `392.0µs` | `359.7µs` |
| [Day 16](./src/bin/16.rs) | `67.9µs` | `67.7µs` |
| [Day 17](./src/bin/17.rs) | `2.2ms` | `2.3ms` |
| [Day 18](./src/bin/18.rs) | `2.0ms` | `2.0ms` |
| [Day 19](./src/bin/19.rs) | `163.2µs` | `93.0µs` |
| [Day 20](./src/bin/20.rs) | `13.1ms` | `4.0ms` |
| [Day 21](./src/bin/21.rs) | `3.4µs` | `3.4µs` |
| [Day 22](./src/bin/22.rs) | `19.6ms` | `1.5ms` |
| [Day 23](./src/bin/23.rs) | `3.8µs` | `4.3µs` |
| [Day 24](./src/bin/24.rs) | `4.3ms` | `1.0ms` |
| [Day 25](./src/bin/25.rs) | `52.0ms` | `-` |

**Total: 668.42ms**
<!--- benchmarking table --->
