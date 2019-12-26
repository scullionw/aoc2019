#![feature(test)]

use aoc2019::{Cell, IntCodeMachine};
use benchtest::benchtest;

const INPUT: &str = include_str!("data/day2.txt");

fn puzzle_a(input: &str) -> i64 {
    let mut intcodes: Vec<_> = input.trim().split(',').map(Cell::Symbol).collect();

    intcodes[1] = Cell::Value(12);
    intcodes[2] = Cell::Value(2);

    IntCodeMachine::default().run(&mut intcodes)
}

fn puzzle_b(input: &str) -> i64 {
    let intcodes: Vec<_> = input.trim().split(',').map(Cell::Symbol).collect();

    let mut machine = IntCodeMachine::default();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcodes = intcodes.clone();
            intcodes[1] = Cell::Value(noun);
            intcodes[2] = Cell::Value(verb);

            if machine.run(&mut intcodes) == 19690720 {
                return 100 * noun + verb;
            }

            machine.clear();
        }
    }

    unreachable!()
}

fn main() {
    println!("{}", puzzle_a(INPUT));
    println!("{}", puzzle_b(INPUT));
}

benchtest! {
    puzzle_a_test: puzzle_a(INPUT) => 3654868,
    puzzle_b_test: puzzle_b(INPUT) => 7014
}
