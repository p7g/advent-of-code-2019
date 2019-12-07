#![allow(dead_code)]

use std::collections::VecDeque;
use std::fs;
use std::ops::{Add, Mul};

#[allow(dead_code)]
#[derive(Debug)]
#[repr(u8)]
enum OpCode {
    Add = 1,
    Mul = 2,
    Input = 3,
    Output = 4,
    TJmp = 5,
    FJmp = 6,
    Lt = 7,
    Eq = 8,
    Halt = 99,
}

fn digits(n: i64) -> Vec<i64> {
    let mut n = n;
    let mut digits = Vec::new();

    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    digits
}

#[derive(Debug, Clone, Copy)]
enum ParamMode {
    Positional = 0,
    Immediate = 1,
}

impl From<i64> for ParamMode {
    fn from(val: i64) -> ParamMode {
        match val {
            0 => ParamMode::Positional,
            1 => ParamMode::Immediate,
            _ => panic!("Invalid parameter mode {}", val),
        }
    }
}

#[derive(Debug)]
pub(crate) struct VM {
    input: VecDeque<i64>,
    output: Vec<i64>,
    code: Vec<i64>,
    ip: usize,
    len: usize,
}

impl VM {
    pub(crate) fn new(input: VecDeque<i64>, code: Vec<i64>) -> Self {
        Self {
            input,
            output: Vec::new(),
            len: code.len(),
            code,
            ip: 0,
        }
    }

    #[allow(clippy::cognitive_complexity)]
    pub(crate) fn evaluate(&mut self) -> Result<Vec<i64>, String> {
        while self.ip < self.len {
            let raw_instruction = self.code[self.ip];
            let param_modes = digits(raw_instruction / 100)
                .into_iter()
                .map(ParamMode::from)
                .collect::<Vec<_>>();
            let param_mode_count = param_modes.len();

            macro_rules! param {
                ($n:expr) => {{
                    let mode = if $n < param_mode_count {
                        param_modes[$n]
                    } else {
                        ParamMode::Positional
                    };
                    let val = self.code[self.ip + $n + 1];

                    match mode {
                        ParamMode::Immediate => val,
                        ParamMode::Positional => self.code[val as usize],
                    }
                }};
            }

            let instruction = unsafe { std::mem::transmute((raw_instruction % 100) as u8) };

            macro_rules! binop {
                ($op:expr, $name:expr) => {{
                    let left = param!(0);
                    let right = param!(1);
                    let dest = self.code[self.ip + 3] as usize;
                    self.code[dest] = $op(left, right);
                    self.ip += 4;
                }};
            }

            match instruction {
                OpCode::Add => binop!(i64::add, "add"),
                OpCode::Mul => binop!(i64::mul, "mul"),

                OpCode::Input => {
                    let dest = self.code[self.ip + 1] as usize;
                    let val = self.input.pop_front().ok_or("Unexpected end of input")?;

                    self.code[dest] = val;
                    self.ip += 2;
                }

                OpCode::Output => {
                    self.output.push(param!(0));
                    self.ip += 2;
                }

                OpCode::TJmp => {
                    let pred = param!(0);
                    let dest = param!(1);

                    self.ip = if pred != 0 { dest as usize } else { self.ip + 3 };
                }

                OpCode::FJmp => {
                    let pred = param!(0);
                    let dest = param!(1);

                    self.ip = if pred == 0 { dest as usize } else { self.ip + 3 };
                }

                OpCode::Lt => binop!(|a, b| (a < b) as i64, "lt"),
                OpCode::Eq => binop!(|a, b| (a == b) as i64, "eq"),

                OpCode::Halt => return Ok(self.output.to_owned()),
            }
        }

        Err("Didn't encounter Halt".into())
    }
}

macro_rules! permutations {
    ($range:expr) => {{
        let mut permutations = Vec::new();

        for i in $range {
            for j in $range {
                if j == i {
                    continue;
                }
                for k in $range {
                    if k == i || k == j {
                        continue;
                    }
                    for l in $range {
                        if l == i || l == j || l == k {
                            continue;
                        }
                        for m in $range {
                            if m == i || m == j || m == k || m == l {
                                continue;
                            }
                            permutations.push([i, j, k, l, m]);
                        }
                    }
                }
            }
        }

        permutations
    }};
}

#[allow(clippy::cognitive_complexity)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = fs::read_to_string("input.txt")?
        .trim()
        .split(',')
        .map(&str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?;

    let mut largest = i64::min_value();
    for permutation in permutations!(0..5) {
        let mut input = vec![0];
        for phase in &permutation {
            let mut with_phase: VecDeque<_> = input.into();
            with_phase.push_front(*phase);

            let mut v = VM::new(with_phase, code.clone());
            input = v.evaluate()?;

            if input[0] > largest {
                largest = input[0];
            }
        }
    }

    println!("Part 1: {}", largest);

    Ok(())
}
