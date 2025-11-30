#![feature(int_roundings)]
advent_of_code::solution!(21);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

#[derive(Debug, Clone, Copy)]
struct Player {
    health: i32,
    damage: i32,
    armor: i32,
    cost: i32,
}

impl Player {
    fn new(health: i32, items: &[Item]) -> Self {
        let mut cost = 0;
        let mut damage = 0;
        let mut armor = 0;
        items.iter().for_each(|i| {
            cost += i.cost;
            damage += i.damage;
            armor += i.armor;
        });
        Player {
            health,
            cost,
            damage,
            armor,
        }
    }
}

#[derive(Debug, Clone)]
struct Shop {
    weapons: [Item; 5],
    armor: [Item; 6],
    rings: [Item; 8],
}

fn parse_input(input: &str) -> (Player, Shop) {
    let lines: Vec<&str> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.trim())
        .collect();
    let mut params: [i32; 3] = [0, 0, 0];
    for l in lines {
        let (p, val) = l.split_once(": ").unwrap();
        let val = val.parse::<i32>().unwrap();
        match p {
            "Hit Points" => params[0] = val,
            "Damage" => params[1] = val,
            "Armor" => params[2] = val,
            _ => panic!(),
        }
    }
    let boss = Player {
        cost: 0,
        health: params[0],
        damage: params[1],
        armor: params[2],
    };
    let weapons = [
        Item {
            cost: 8,
            damage: 4,
            armor: 0,
        },
        Item {
            cost: 10,
            damage: 5,
            armor: 0,
        },
        Item {
            cost: 25,
            damage: 6,
            armor: 0,
        },
        Item {
            cost: 40,
            damage: 7,
            armor: 0,
        },
        Item {
            cost: 74,
            damage: 8,
            armor: 0,
        },
    ];

    let armor = [
        Item {
            cost: 13,
            damage: 0,
            armor: 1,
        },
        Item {
            cost: 31,
            damage: 0,
            armor: 2,
        },
        Item {
            cost: 53,
            damage: 0,
            armor: 3,
        },
        Item {
            cost: 75,
            damage: 0,
            armor: 4,
        },
        Item {
            cost: 102,
            damage: 0,
            armor: 5,
        },
        // Empty items
        Item {
            cost: 0,
            damage: 0,
            armor: 0,
        },
    ];

    let rings = [
        Item {
            cost: 25,
            damage: 1,
            armor: 0,
        },
        Item {
            cost: 50,
            damage: 2,
            armor: 0,
        },
        Item {
            cost: 100,
            damage: 3,
            armor: 0,
        },
        Item {
            cost: 20,
            damage: 0,
            armor: 1,
        },
        Item {
            cost: 40,
            damage: 0,
            armor: 2,
        },
        Item {
            cost: 80,
            damage: 0,
            armor: 3,
        },
        // Empty items
        Item {
            cost: 0,
            damage: 0,
            armor: 0,
        },
        Item {
            cost: 0,
            damage: 0,
            armor: 0,
        },
    ];

    let shop = Shop {
        weapons,
        armor,
        rings,
    };
    (boss, shop)
}

fn solve(boss: &Player, shop: &Shop) -> (Option<i32>, Option<i32>) {
    // Mandatory to have one
    let weapons = shop.weapons;
    // Armor 0 -1
    let armor = shop.armor;
    // between 0 - 2
    let rings = shop.rings;

    let mut min_win = i32::MAX;
    let mut max_loss = i32::MIN;

    for w in weapons.iter() {
        for a in armor.iter() {
            for r1 in rings.iter() {
                for r2 in &rings[0..rings.len() - 1] {
                    if r1 != r2 {
                        let player = Player::new(100, &[*w, *a, *r1, *r2]);
                        if player_wins(&player, boss) {
                            min_win = min_win.min(player.cost);
                        } else {
                            max_loss = max_loss.max(player.cost);
                        }
                    }
                }
            }
        }
    }
    let mut res_min = None;
    let mut res_max = None;
    if max_loss != i32::MIN {
        res_max = Some(max_loss);
    }
    if min_win != i32::MAX {
        res_min = Some(min_win);
    }
    (res_min, res_max)
}

fn player_wins(player: &Player, boss: &Player) -> bool {
    let player_dmg = (player.damage - boss.armor).max(1);
    let boss_dmg = (boss.damage - player.armor).max(1);

    let player_turns = boss.health.div_ceil(player_dmg);
    let boss_turns = player.health.div_ceil(boss_dmg);

    player_turns <= boss_turns
}

pub fn part_one(input: &str) -> Option<i32> {
    let (boss, shop) = parse_input(input);
    let (min_win, _) = solve(&boss, &shop);
    min_win
}

pub fn part_two(input: &str) -> Option<i32> {
    let (boss, shop) = parse_input(input);
    let (_, max_loss) = solve(&boss, &shop);
    max_loss
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(28));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
