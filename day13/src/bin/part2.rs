#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};
use std::fs;
use std::ops::{Add, Mul};

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
    AdjustRelativeBase = 9,
    Halt = 99,
}

impl OpCode {
    pub fn effect(&self) -> usize {
        match self {
            OpCode::Add | OpCode::Mul => 4,
            OpCode::Input | OpCode::Output => 2,
            OpCode::TJmp | OpCode::FJmp => 3,
            OpCode::Lt | OpCode::Eq => 4,
            OpCode::AdjustRelativeBase => 2,
            OpCode::Halt => 1,
        }
    }
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
    Relative = 2,
}

impl From<i64> for ParamMode {
    fn from(val: i64) -> ParamMode {
        match val {
            0 => ParamMode::Positional,
            1 => ParamMode::Immediate,
            2 => ParamMode::Relative,
            _ => panic!("Invalid parameter mode {}", val),
        }
    }
}

#[derive(Debug, PartialEq)]
enum ExecutionStatus {
    Output(i64),
    AwaitingInput,
    Complete,
}

#[derive(Debug)]
pub(crate) struct VM {
    code: Vec<i64>,
    ip: usize,
    len: usize,
    relative_base: i64,
    extra_memory: HashMap<usize, i64>,
}

impl VM {
    pub(crate) fn new(code: Vec<i64>) -> Self {
        Self {
            len: code.len(),
            code,
            ip: 0,
            relative_base: 0,
            extra_memory: HashMap::new(),
        }
    }

    fn get(&self, idx: usize) -> i64 {
        if idx < self.len {
            self.code[idx]
        } else {
            *self.extra_memory.get(&idx).unwrap_or(&0)
        }
    }

    fn set(&mut self, idx: usize, val: i64) {
        if idx < self.len {
            self.code[idx] = val;
        } else {
            self.extra_memory.insert(idx, val);
        }
    }

    #[allow(clippy::cognitive_complexity)]
    pub(crate) fn resume(&mut self, mut input: Option<i64>) -> Result<ExecutionStatus, String> {
        while self.ip < self.len {
            let raw_instruction = self.get(self.ip);
            let param_modes = digits(raw_instruction / 100)
                .into_iter()
                .map(ParamMode::from)
                .collect::<Vec<_>>();
            let param_mode_count = param_modes.len();

            macro_rules! param_mode {
                ($n:expr) => {{
                    if $n < param_mode_count {
                        param_modes[$n]
                    } else {
                        ParamMode::Positional
                    }
                }};
            }

            macro_rules! param {
                ($n:expr) => {{
                    let mode = param_mode!($n);
                    let val = self.get(self.ip + $n + 1);

                    match mode {
                        ParamMode::Immediate => val,
                        ParamMode::Positional => self.get(val as usize),
                        ParamMode::Relative => self.get((val + self.relative_base) as usize),
                    }
                }};
            }

            macro_rules! param_dest {
                ($n:expr, $val:expr) => {{
                    let mode = param_mode!($n);
                    let val = self.get(self.ip + $n + 1);

                    match mode {
                        ParamMode::Positional => self.set(val as usize, $val),
                        ParamMode::Relative => self.set((val + self.relative_base) as usize, $val),
                        _ => unreachable!(),
                    }
                }};
            }

            macro_rules! binop {
                ($op:expr) => {{
                    let left = param!(0);
                    let right = param!(1);
                    param_dest!(2, $op(left, right));
                }};
            }

            let instruction = unsafe { std::mem::transmute((raw_instruction % 100) as u8) };

            match instruction {
                OpCode::Add => binop!(i64::add),
                OpCode::Mul => binop!(i64::mul),

                OpCode::Input => {
                    if let Some(val) = input.take() {
                        param_dest!(0, val);
                    } else {
                        return Ok(ExecutionStatus::AwaitingInput);
                    }
                }

                OpCode::Output => {
                    let val = param!(0);
                    self.ip += instruction.effect();
                    return Ok(ExecutionStatus::Output(val));
                }

                OpCode::TJmp => {
                    let pred = param!(0);
                    let dest = param!(1);

                    if pred != 0 {
                        self.ip = dest as usize;
                        continue;
                    }
                }

                OpCode::FJmp => {
                    let pred = param!(0);
                    let dest = param!(1);

                    if pred == 0 {
                        self.ip = dest as usize;
                        continue;
                    }
                }

                OpCode::Lt => binop!(|a, b| (a < b) as i64),
                OpCode::Eq => binop!(|a, b| (a == b) as i64),

                OpCode::AdjustRelativeBase => {
                    let adjustment = param!(0);
                    self.relative_base += adjustment;
                }

                OpCode::Halt => return Ok(ExecutionStatus::Complete),
            }

            self.ip += instruction.effect();
        }

        Err("Didn't encounter Halt".into())
    }

    pub fn run_to_completion<T>(&mut self, input: T) -> Result<Vec<i64>, String>
    where
        T: Into<VecDeque<i64>>,
    {
        let mut input: VecDeque<i64> = input.into();
        let mut result = self.resume(input.pop_front())?;
        let mut output = Vec::new();

        while result != ExecutionStatus::Complete {
            match result {
                ExecutionStatus::AwaitingInput => {
                    result = self.resume(input.pop_front())?;
                }
                ExecutionStatus::Output(n) => {
                    output.push(n);
                    result = self.resume(input.pop_front())?;
                }
                ExecutionStatus::Complete => unreachable!(),
            }
        }

        Ok(output)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("input.txt")?;
    let mut code = input
        .trim()
        .split(',')
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?;

    code[0] = 2;

    let mut vm = VM::new(code);
    let mut ball_coords: Option<(i64, i64)> = None;
    let mut paddle_coords: Option<(i64, i64)> = None;
    let mut score = None;
    let mut input = None;

    loop {
        match vm.resume(input.take())? {
            ExecutionStatus::Complete => break,

            ExecutionStatus::AwaitingInput => {
                let ball = ball_coords.unwrap();
                let paddle = paddle_coords.unwrap();

                input = if ball.0 < paddle.0 {
                    Some(-1)
                } else if ball.0 > paddle.0 {
                    Some(1)
                } else {
                    Some(0)
                }
            },

            ExecutionStatus::Output(x) => {
                if let ExecutionStatus::Output(y) = vm.resume(None)? {
                    if let ExecutionStatus::Output(typ) = vm.resume(None)? {
                        if x == -1 && y == 0 {
                            score = Some(typ);
                        } else if typ == 3 {
                            paddle_coords = Some((x, y));
                        } else if typ == 4 {
                            ball_coords = Some((x, y));
                        }
                    } else {
                        unreachable!();
                    }
                } else {
                    unreachable!();
                }
            }
        }
    }

    println!("{}", score.unwrap());

    Ok(())
}
