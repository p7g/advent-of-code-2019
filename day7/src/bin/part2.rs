#![allow(dead_code)]

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
    last_output: Option<i64>,
}

impl VM {
    pub(crate) fn new(code: Vec<i64>) -> Self {
        Self {
            len: code.len(),
            code,
            ip: 0,
            last_output: None,
        }
    }

    #[allow(clippy::cognitive_complexity)]
    pub(crate) fn resume(&mut self, mut input: Option<i64>) -> Result<ExecutionStatus, String> {
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
                    if let Some(val) = input.take() {
                        self.code[dest] = val;
                        self.ip += 2;
                    } else {
                        return Ok(ExecutionStatus::AwaitingInput);
                    }
                }

                OpCode::Output => {
                    let val = param!(0);
                    self.ip += 2;
                    self.last_output = Some(val);
                    return Ok(ExecutionStatus::Output(val));
                }

                OpCode::TJmp => {
                    let pred = param!(0);
                    let dest = param!(1);

                    self.ip = if pred != 0 {
                        dest as usize
                    } else {
                        self.ip + 3
                    };
                }

                OpCode::FJmp => {
                    let pred = param!(0);
                    let dest = param!(1);

                    self.ip = if pred == 0 {
                        dest as usize
                    } else {
                        self.ip + 3
                    };
                }

                OpCode::Lt => binop!(|a, b| (a < b) as i64, "lt"),
                OpCode::Eq => binop!(|a, b| (a == b) as i64, "eq"),

                OpCode::Halt => return Ok(ExecutionStatus::Complete),
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
    for permutation in permutations!(5..10) {
        let mut amplifiers = permutation
            .iter()
            .map(|p| {
                let mut v = VM::new(code.clone());
                assert_eq!(v.resume(Some(*p)).unwrap(), ExecutionStatus::AwaitingInput);
                v
            })
            .collect::<Vec<_>>();

        let mut signal = 0;
        let mut last_from_e = None;
        let mut num_completed = 0;

        'outer: loop {
            macro_rules! update_largest {
                ($n:expr) => {{
                    if $n > largest {
                        largest = $n;
                    }
                    break 'outer;
                }};
            }
            for (i, amp) in amplifiers.iter_mut().enumerate() {
                if num_completed == 5 {
                    if let Some(n) = last_from_e {
                        update_largest!(n);
                    }
                }
                match amp.resume(Some(signal))? {
                    ExecutionStatus::AwaitingInput => {}
                    ExecutionStatus::Output(n) => {
                        signal = n;
                        if i == 4 {
                            last_from_e = Some(n);
                        }

                        match amp.resume(None)? {
                            ExecutionStatus::Complete => {
                                num_completed += 1;
                            }
                            ExecutionStatus::AwaitingInput => {}
                            _ => unreachable!(),
                        }
                    }
                    ExecutionStatus::Complete => {
                        num_completed += 1;
                    }
                }
            }
        }
    }

    println!("Part 2: {}", largest);

    Ok(())
}
