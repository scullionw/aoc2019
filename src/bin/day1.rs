#![feature(test)]

use benchtest::benchtest;

const INPUT: &str = include_str!("data/day1.txt");

fn puzzle_a(input: &str) -> u64 {
    let masses = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap());

    masses.map(|m| m / 3 - 2).sum()
}

fn puzzle_b(input: &str) -> u64 {
    let masses = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap());

    masses.map(|m| fuel(m)).sum()
}

fn fuel(mass: u64) -> u64 {
    match (mass / 3).checked_sub(2) {
        Some(0) | None => 0,
        Some(fuel_mass) => fuel_mass + fuel(fuel_mass),
    }
}

fn main() {
    println!("{}", puzzle_a(INPUT));
    println!("{}", puzzle_b(INPUT));
}

benchtest! {
    puzzle_a_test: puzzle_a(INPUT) => 3405721,
    puzzle_b_test: puzzle_b(INPUT) => 5105716
}
