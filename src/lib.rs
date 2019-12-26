use std::str::FromStr;

enum Mode {
    Position,
    Immediate,
}

enum Operation {
    Adder,
    Multiplier,
}

impl Operation {
    fn op(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Adder => a + b,
            Operation::Multiplier => a * b,
        }
    }
}

#[derive(Debug)]
struct ModeParseError;

impl FromStr for Mode {
    type Err = ModeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Mode::Position),
            "1" => Ok(Mode::Immediate),
            _ => Err(ModeParseError),
        }
    }
}

enum Instruction {
    Add { a: Mode, b: Mode, alu: Operation },
    Mul { a: Mode, b: Mode, alu: Operation },
    Input,
    Output { a: Mode },
    Halt,
}

impl Instruction {
    fn add(a: Mode, b: Mode) -> Instruction {
        Instruction::Add {
            a,
            b,
            alu: Operation::Adder,
        }
    }

    fn mul(a: Mode, b: Mode) -> Instruction {
        Instruction::Mul {
            a,
            b,
            alu: Operation::Multiplier,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Cell<'a> {
    Value(u64),
    Symbol(&'a str),
}

// TODO: Default variant should be Symbol
impl<'a> Cell<'_> {
    fn value(&self) -> u64 {
        match self {
            Cell::Value(v) => *v,
            _ => panic!("Cannot load value from a symbol!"),
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

// TODO: Faster parsing if already int?
impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();

        let (modes, opcode) = if len > 1 {
            s.split_at(len - 2)
        } else {
            s.split_at(len - 1)
        };

        // How to reverse without allocating (rev chars?)
        let mut modes = modes
            .split("")
            .filter(|&s| s != "")
            .map(|s| s.parse::<Mode>().unwrap());

        let mut next_mode = || modes.next().unwrap_or(Mode::Position);

        match opcode.parse::<u64>().unwrap() {
            1 => Ok(Instruction::add(next_mode(), next_mode())),
            2 => Ok(Instruction::mul(next_mode(), next_mode())),
            3 => Ok(Instruction::Input),
            4 => Ok(Instruction::Output { a: next_mode() }),
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

// TODO: Replace symbol with value
fn load(tape: &[Cell], cursor: usize, offset: usize, mode: Mode) -> u64 {
    let cell = match mode {
        Mode::Immediate => tape[cursor + offset],
        Mode::Position => tape[tape[cursor + offset].value() as usize],
    };

    match cell {
        Cell::Value(v) => v,
        Cell::Symbol(s) => s.parse::<u64>().unwrap(),
    }
}

impl IntCodeMachine {
    pub fn add_input(&mut self, input: i64) {
        self.inputs.push(input);
    }

    pub fn diagnostic(&self) -> Option<i64> {
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

    pub fn run(&mut self, tape: &mut [Cell]) -> u64 {
        let mut cursor = 0;

        loop {
            let instruction = tape[cursor].instruction();
            match instruction {
                Instruction::Add { a, b, alu } | Instruction::Mul { a, b, alu } => {
                    let a = load(tape, cursor, 1, a);
                    let b = load(tape, cursor, 2, b);
                    tape[tape[cursor + 3].value() as usize] = Cell::Value(alu.op(a, b));
                    cursor += 4
                }
                Instruction::Halt => break,
                _ => unreachable!(),
            }
        }

        tape[0].value()
    }
}
