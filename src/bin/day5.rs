#![feature(test)]

use aoc2019::{Cell, IntCodeMachine};
use benchtest::benchtest;

const INPUT: &str = include_str!("data/day5.txt");

fn puzzle_a(input: &str) -> i64 {
    let mut intcodes: Vec<_> = input.trim().split(',').map(Cell::Symbol).collect();

    let mut machine = IntCodeMachine::default();

    machine.add_input(1);
    machine.run(&mut intcodes);

    if machine.errors() {
        panic!("Errors detected!");
    };

    machine.diagnostic_code().unwrap()
}

fn puzzle_b(input: &str) -> i64 {
    todo!()
}

fn main() {
    println!("{}", puzzle_a(INPUT));
    // println!("{}", puzzle_b(INPUT));
}

benchtest! {
    puzzle_a_test: puzzle_a(INPUT) => 12896948
    // puzzle_b_test: puzzle_b(INPUT) => 7014
}
