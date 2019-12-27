use std::cmp;
use std::convert::TryFrom;
use std::str::FromStr;

enum Mode {
    Position,
    Immediate,
}

#[derive(Debug)]
struct ModeParseError;

impl TryFrom<char> for Mode {
    type Error = ModeParseError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Mode::Position),
            '1' => Ok(Mode::Immediate),
            _ => Err(ModeParseError),
        }
    }
}

enum Instruction {
    Add(Mode, Mode),
    Mul(Mode, Mode),
    Input,
    Output(Mode),
    Halt,
    JumpNZ(Mode, Mode),
    JumpZ(Mode, Mode),
    LT(Mode, Mode),
    EQ(Mode, Mode),
}

#[derive(Debug, Copy, Clone)]
pub enum Cell<'a> {
    Value(i64),
    Symbol(&'a str),
}

impl<'a> Cell<'_> {
    fn value(&self) -> i64 {
        match self {
            Cell::Value(v) => *v,
            Cell::Symbol(s) => s.parse::<i64>().unwrap(),
        }
    }

    fn instruction(&self) -> Instruction {
        match self {
            Cell::Symbol(s) => s.parse().unwrap(),
            Cell::Value(v) => v.to_string().parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct InstructionParseError;

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        let (modes, opcode) = s.split_at(len - cmp::min(2, len));
        let mut modes = modes.chars().rev().map(|s| Mode::try_from(s).unwrap());
        let mut mode = || modes.next().unwrap_or(Mode::Position);

        match opcode.parse::<i64>().unwrap() {
            1 => Ok(Instruction::Add(mode(), mode())),
            2 => Ok(Instruction::Mul(mode(), mode())),
            3 => Ok(Instruction::Input),
            4 => Ok(Instruction::Output(mode())),
            5 => Ok(Instruction::JumpNZ(mode(), mode())),
            6 => Ok(Instruction::JumpZ(mode(), mode())),
            7 => Ok(Instruction::LT(mode(), mode())),
            8 => Ok(Instruction::EQ(mode(), mode())),
            99 => Ok(Instruction::Halt),
            _ => Err(InstructionParseError),
        }
    }
}

#[derive(Default)]
pub struct IntCodeMachine {
    inputs: Vec<i64>,
    outputs: Vec<i64>,
}

fn load(tape: &[Cell], cursor: usize, offset: usize, mode: Mode) -> i64 {
    match mode {
        Mode::Immediate => tape[cursor + offset].value(),
        Mode::Position => tape[tape[cursor + offset].value() as usize].value(),
    }
}

fn store(tape: &mut [Cell], cursor: usize, offset: usize, value: i64) {
    tape[tape[cursor + offset].value() as usize] = Cell::Value(value);
}

impl IntCodeMachine {
    pub fn add_input(&mut self, input: i64) {
        self.inputs.push(input);
    }

    pub fn diagnostic_code(&self) -> Option<i64> {
        self.outputs.last().copied()
    }

    pub fn errors(&self) -> bool {
        let len = self.outputs.len();
        self.outputs[..len - 1].iter().any(|&output| output != 0)
    }

    pub fn clear(&mut self) {
        self.inputs.clear();
        self.outputs.clear();
    }

    pub fn run(&mut self, tape: &mut [Cell]) -> i64 {
        let mut cursor = 0;

        loop {
            match tape[cursor].instruction() {
                Instruction::Add(mode_1, mode_2) => {
                    let a = load(tape, cursor, 1, mode_1);
                    let b = load(tape, cursor, 2, mode_2);
                    store(tape, cursor, 3, a + b);
                    cursor += 4
                }
                Instruction::Mul(mode_1, mode_2) => {
                    let a = load(tape, cursor, 1, mode_1);
                    let b = load(tape, cursor, 2, mode_2);
                    store(tape, cursor, 3, a * b);
                    cursor += 4
                }
                Instruction::Input => {
                    store(tape, cursor, 1, self.inputs.pop().unwrap());
                    cursor += 2;
                }
                Instruction::Output(mode) => {
                    self.outputs.push(load(tape, cursor, 1, mode));
                    cursor += 2
                }
                Instruction::JumpNZ(mode_1, mode_2) => {
                    if load(tape, cursor, 1, mode_1) != 0 {
                        cursor = load(tape, cursor, 2, mode_2) as usize;
                    } else {
                        cursor += 3;
                    }
                }
                Instruction::JumpZ(mode_1, mode_2) => {
                    if load(tape, cursor, 1, mode_1) == 0 {
                        cursor = load(tape, cursor, 2, mode_2) as usize;
                    } else {
                        cursor += 3
                    }
                }
                Instruction::LT(mode_1, mode_2) => {
                    let a = load(tape, cursor, 1, mode_1);
                    let b = load(tape, cursor, 2, mode_2);
                    store(tape, cursor, 3, if a < b { 1 } else { 0 });
                    cursor += 4;
                }
                Instruction::EQ(mode_1, mode_2) => {
                    let a = load(tape, cursor, 1, mode_1);
                    let b = load(tape, cursor, 2, mode_2);
                    store(tape, cursor, 3, if a == b { 1 } else { 0 });
                    cursor += 4;
                }
                Instruction::Halt => break,
            }
        }

        tape[0].value()
    }
}
