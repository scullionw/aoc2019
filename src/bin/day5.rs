#![feature(test)]

use aoc2019::IntCodeMachine;
use benchtest::benchtest;

const INPUT: &str = include_str!("data/day2.txt");

fn puzzle_a(input: &str) -> u64 {
    // let mut intcodes: Vec<_> = input
    //     .trim()
    //     .split(',')
    //     .map(|s| s.parse::<u64>().unwrap())
    //     .collect();

    // intcodes[1] = 12;
    // intcodes[2] = 2;

    // IntCodeMachine::run(&mut intcodes)

    todo!()
}

fn puzzle_b(input: &str) -> u64 {
    todo!()
}

fn main() {
    println!("{}", puzzle_a(INPUT));
    // println!("{}", puzzle_b(INPUT));
}

// benchtest! {
//     puzzle_a_test: puzzle_a(INPUT) => 3654868,
//     puzzle_b_test: puzzle_b(INPUT) => 7014
// }
