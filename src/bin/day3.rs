#![feature(test)]

use benchtest::benchtest;
use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("data/day3.txt");

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug, Default)]
struct Position {
    x: i32,
    y: i32,
}

fn puzzle_a(input: &str) -> i32 {
    let (first, second) = input.trim().lines().collect_tuple().unwrap();
    let (first_path, second_path) = (wire_path(first), wire_path(second));

    first_path
        .keys()
        .filter(|&&pos| pos != Position::default())
        .filter_map(|pos| second_path.get(pos).map(|_| pos.x.abs() + pos.y.abs()))
        .min()
        .unwrap()
}

fn puzzle_b(input: &str) -> u32 {
    let (first, second) = input.trim().lines().collect_tuple().unwrap();
    let (first_path, second_path) = (wire_path(first), wire_path(second));

    first_path
        .iter()
        .filter_map(|(pos, s1)| second_path.get(&pos).map(|s2| s1 + s2))
        .min()
        .unwrap()
}

fn wire_path(instructions: &str) -> HashMap<Position, u32> {
    let mut path = HashMap::new();
    let mut pos = Position::default();
    let mut step = 0;

    fn trace(
        schematic: &mut HashMap<Position, u32>,
        amt: i32,
        pos: &mut Position,
        step: &mut u32,
        mut inc: impl FnMut(&mut Position),
    ) {
        for _ in 0..amt {
            inc(pos);
            *step += 1;
            schematic.entry(*pos).or_insert(*step);
        }
    }

    for code in instructions.split(",") {
        let direction = code.get(0..1).unwrap();
        let amt = code.get(1..).unwrap().parse::<i32>().unwrap();
        match direction {
            "R" => trace(&mut path, amt, &mut pos, &mut step, |p| p.x += 1),
            "L" => trace(&mut path, amt, &mut pos, &mut step, |p| p.x -= 1),
            "U" => trace(&mut path, amt, &mut pos, &mut step, |p| p.y += 1),
            "D" => trace(&mut path, amt, &mut pos, &mut step, |p| p.y -= 1),
            _ => unreachable!(),
        };
    }
    path
}

fn main() {
    println!("{}", puzzle_a(INPUT));
    println!("{}", puzzle_b(INPUT));
}

benchtest! {
    puzzle_a_test: puzzle_a(INPUT) => 221,
    puzzle_b_test: puzzle_b(INPUT) => 18542
}
