#![feature(test)]

use benchtest::benchtest;

const INPUT: &str = include_str!("data/day2.txt");

fn puzzle_a(input: &str) -> u64 {
    let mut intcodes: Vec<_> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    intcodes[1] = 12;
    intcodes[2] = 2;

    compute(&mut intcodes)
}

fn puzzle_b(input: &str) -> u64 {
    let intcodes: Vec<_> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut intcodes = intcodes.clone();
            intcodes[1] = noun;
            intcodes[2] = verb;

            if compute(&mut intcodes) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    unreachable!()
}

fn operation(tape: &mut [u64], pos: usize, op: impl Fn(u64, u64) -> u64) {
    let a = tape[tape[pos + 1] as usize];
    let b = tape[tape[pos + 2] as usize];
    tape[tape[pos + 3] as usize] = op(a, b);
}

fn compute(tape: &mut [u64]) -> u64 {
    let mut pos = 0;

    loop {
        match tape[pos] {
            1 => operation(tape, pos, |a, b| a + b),
            2 => operation(tape, pos, |a, b| a * b),
            99 => break,
            _ => unreachable!(),
        }
        pos += 4;
    }

    tape[0]
}

fn main() {
    println!("{}", puzzle_a(INPUT));
    println!("{}", puzzle_b(INPUT));
}

benchtest! {
    puzzle_a_test: puzzle_a(INPUT) => 3654868,
    puzzle_b_test: puzzle_b(INPUT) => 7014
}
