advent_of_code::solution!(23);

const NR_REGISTERS: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Instruction {
    Hlf(u8),
    Tpl(u8),
    Inc(u8),
    Jmp(i32),
    Jie(u8, i32),
    Jio(u8, i32),
    NotImplemented,
}

fn map_register(input: &str) -> u8 {
    match input {
        "a" => 0,
        "b" => 1,
        _ => panic!(),
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim())
        .map(|l| {
            let cleaned = l.replace(",", "");
            let parts: Vec<&str> = cleaned.split_whitespace().collect();
            match parts[0] {
                "hlf" => Instruction::Hlf(map_register(parts[1])),
                "tpl" => Instruction::Tpl(map_register(parts[1])),
                "inc" => Instruction::Inc(map_register(parts[1])),
                "jmp" => Instruction::Jmp(parts[1].parse::<i32>().unwrap()),
                "jie" => Instruction::Jie(map_register(parts[1]), parts[2].parse::<i32>().unwrap()),
                "jio" => Instruction::Jio(map_register(parts[1]), parts[2].parse::<i32>().unwrap()),
                _ => Instruction::NotImplemented,
            }
        })
        .collect()
}

fn solve(ins: &[Instruction], start_a: i32) -> i32 {
    let mut registers: [i32; NR_REGISTERS] = [0; NR_REGISTERS];
    registers[0] = start_a;
    let mut idx: usize = 0;
    let mut current = ins[idx];
    while current != Instruction::NotImplemented {
        match current {
            Instruction::Hlf(r) => {
                registers[r as usize] /= 2;
                idx += 1
            }
            Instruction::Tpl(r) => {
                registers[r as usize] *= 3;
                idx += 1
            }
            Instruction::Inc(r) => {
                registers[r as usize] += 1;
                idx += 1
            }
            Instruction::Jmp(off) => {
                let new_idx = idx as i32 + off;
                if new_idx >= 0 {
                    idx = new_idx as usize;
                }
            }
            Instruction::Jie(r, off) => {
                if registers[r as usize] % 2 == 0 {
                    let new_idx = idx as i32 + off;
                    if new_idx >= 0 {
                        idx = new_idx as usize;
                    }
                } else {
                    idx += 1;
                }
            }
            Instruction::Jio(r, off) => {
                if registers[r as usize] == 1 {
                    let new_idx = idx as i32 + off;
                    if new_idx >= 0 {
                        idx = new_idx as usize;
                    }
                } else {
                    idx += 1;
                }
            }
            Instruction::NotImplemented => {
                break;
            }
        }
        if idx < ins.len() {
            current = ins[idx];
        } else {
            break;
        }
    }
    registers[1]
}

pub fn part_one(input: &str) -> Option<i32> {
    let ins = parse_input(input);
    Some(solve(&ins, 0))
}

pub fn part_two(input: &str) -> Option<i32> {
    let ins = parse_input(input);
    Some(solve(&ins, 1))
}
