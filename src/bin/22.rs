use std::cmp::Ordering;
use std::collections::BinaryHeap;

advent_of_code::solution!(22);

#[derive(Debug, Clone, Copy)]
struct Boss {
    health: i32,
    damage: i32,
}

fn parse_input(input: &str) -> Boss {
    let lines: Vec<i32> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (_, val) = l.trim().split_once(": ").unwrap();
            val.parse::<i32>().unwrap()
        })
        .collect();
    Boss {
        health: lines[0],
        damage: lines[1],
    }
}

#[derive(Debug, Clone, Copy)]
enum Spell {
    Missile,
    Drain,
    Shield,
    Poison,
    Recharge,
}
// ----- Spell Costs -----
const MAGIC_MISSILE: i32 = 53;
const DRAIN: i32 = 73;
const SHIELD: i32 = 113;
const POISON: i32 = 173;
const RECHARGE: i32 = 229;

const AVAILABLE_SPELLS: [(Spell, i32); 5] = [
    (Spell::Missile, MAGIC_MISSILE),
    (Spell::Drain, DRAIN),
    (Spell::Shield, SHIELD),
    (Spell::Poison, POISON),
    (Spell::Recharge, RECHARGE),
];

#[derive(Debug, Clone, Copy, Eq)]
struct State {
    player_hp: i32,
    mana: i32,
    boss_hp: i32,
    boss_damage: i32,

    timer_shield: i32,
    timer_poison: i32,
    timer_recharge: i32,

    mana_spent: i32,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.mana_spent == other.mana_spent
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse, order by lowest mana first
        other.mana_spent.cmp(&self.mana_spent)
    }
}

impl State {
    fn apply_effects(&mut self) {
        if self.timer_shield > 0 {
            self.timer_shield -= 1;
        }
        if self.timer_poison > 0 {
            self.boss_hp -= 3;
            self.timer_poison -= 1;
        }
        if self.timer_recharge > 0 {
            self.mana += 101;
            self.timer_recharge -= 1;
        }
    }
    fn armor(&self) -> i32 {
        if self.timer_shield > 0 { 7 } else { 0 }
    }
    fn is_boss_dead(&self) -> bool {
        self.boss_hp <= 0
    }
    fn is_player_dead(&self) -> bool {
        self.player_hp <= 0
    }
    fn cast_spell(&mut self, sp: Spell, cost: i32) {
        match sp {
            Spell::Missile => {
                self.boss_hp -= 4;
                self.mana -= cost;
                self.mana_spent += cost;
            }
            Spell::Drain => {
                self.boss_hp -= 2;
                self.player_hp += 2;
                self.mana -= cost;
                self.mana_spent += cost;
            }
            Spell::Shield => {
                self.timer_shield = 6;
                self.mana -= cost;
                self.mana_spent += cost;
            }
            Spell::Poison => {
                self.timer_poison = 6;
                self.mana -= cost;
                self.mana_spent += cost;
            }
            Spell::Recharge => {
                self.timer_recharge = 5;
                self.mana -= cost;
                self.mana_spent += cost;
            }
        }
    }
}

fn solve(
    player_hp: i32,
    player_mana: i32,
    boss_hp: i32,
    boss_damage: i32,
    part2: bool,
) -> Option<i32> {
    // Assumes States are ordered by decreasing order of mana_spent.
    // So highest priority = lowest mana_spent.
    let mut pq: BinaryHeap<State> = BinaryHeap::new();

    let init = State {
        player_hp,
        mana: player_mana,
        boss_hp,
        boss_damage,
        timer_shield: 0,
        timer_poison: 0,
        timer_recharge: 0,
        mana_spent: 0,
    };

    pq.push(init);

    while let Some(mut state) = pq.pop() {
        if part2 {
            state.player_hp -= 1;
            if state.player_hp <= 0 {
                // loss
                continue;
            }
        }
        // Apply effects at start of round
        state.apply_effects();
        if state.is_boss_dead() {
            return Some(state.mana_spent);
        }
        // Try casting each spell
        for (spell, cost) in AVAILABLE_SPELLS {
            if state.mana < cost {
                continue;
            }
            // If timer is already running, you can't reapply the effect
            match spell {
                Spell::Shield if state.timer_shield > 0 => continue,
                Spell::Poison if state.timer_poison > 0 => continue,
                Spell::Recharge if state.timer_recharge > 0 => continue,
                _ => {
                    //pass
                }
            }
            // Copy state
            let mut next = state;
            // valid spell to cast
            next.cast_spell(spell, cost);
            if next.is_boss_dead() {
                return Some(next.mana_spent);
            }
            // Start of boss turn
            next.apply_effects();
            if next.is_boss_dead() {
                return Some(next.mana_spent);
            }
            // Boss attacks
            let dmg = (next.boss_damage - next.armor()).max(1);
            next.player_hp -= dmg;
            if !next.is_player_dead() {
                pq.push(next);
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<i32> {
    let boss = parse_input(input);
    solve(50, 500, boss.health, boss.damage, false)
}

pub fn part_two(input: &str) -> Option<i32> {
    let boss = parse_input(input);
    solve(50, 500, boss.health, boss.damage, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let boss = parse_input(input);
        let result = solve(10, 250, boss.health, boss.damage, false);
        assert_eq!(result, Some(250 - 24));
    }
}
