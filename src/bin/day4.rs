#![feature(test)]

use benchtest::benchtest;
use itertools::Itertools;
use std::iter;

const INPUT: &str = include_str!("data/day4.txt");

fn puzzle_a(input: &str) -> usize {
    let (low, high) = input
        .trim()
        .split("-")
        .map(|s| s.parse::<u32>().unwrap())
        .collect_tuple()
        .unwrap();

    (low..=high).filter(|&pwd| valid(pwd)).count()
}

fn puzzle_b(input: &str) -> usize {
    let (low, high) = input
        .trim()
        .split("-")
        .map(|s| s.parse::<u32>().unwrap())
        .collect_tuple()
        .unwrap();

    (low..=high).filter(|&pwd| valid_extra(pwd)).count()
}

// Iterates in reverse on the digits, to prevent allocation
fn digits(mut n: u32) -> impl Iterator<Item = u32> {
    iter::from_fn(move || {
        if n == 0 {
            None
        } else {
            let temp_n = n;
            n /= 10;
            Some(temp_n % 10)
        }
    })
}

fn valid(password: u32) -> bool {
    let mut previous = None;
    let mut double_digits = false;

    for digit in digits(password) {
        if let Some(p) = previous {
            if digit == p {
                double_digits = true;
            }
            if digit > p {
                return false;
            }
        }
        previous = Some(digit);
    }

    double_digits
}

fn valid_extra(password: u32) -> bool {
    let mut counts = [0; 10];
    let mut previous = None;

    for digit in digits(password) {
        if let Some(p) = previous {
            if digit > p {
                return false;
            }
        }

        // Repeats are consecutive since sequence is never decreasing
        counts[digit as usize] += 1;
        previous = Some(digit);
    }

    counts.iter().any(|&count| count == 2)
}

fn main() {
    println!("{}", puzzle_a(INPUT));
    println!("{}", puzzle_b(INPUT));
}

benchtest! {
    puzzle_a_test: puzzle_a(INPUT) => 1929,
    puzzle_b_test: puzzle_b(INPUT) => 1306
}
