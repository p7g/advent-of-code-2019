use std::fs::read_to_string;
use std::ops::{Add, Mul};
use std::io::Write;

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

fn main() -> Result<(), String> {
    let contents = read_to_string("input.txt").expect("Failed to read file");
    let stdin = std::io::stdin();

    let input = contents
        .trim()
        .split(',')
        .map(|num| num.parse::<i64>().expect("Failed to parse int"))
        .collect::<Vec<_>>();

    let code = input.clone();

    let _result = VM::new(stdin, code).evaluate()?;

    Ok(())
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
struct VM {
    input: std::io::Stdin,
    code: Vec<i64>,
    ip: usize,
    len: usize,
}

impl VM {
    fn new(input: std::io::Stdin, code: Vec<i64>) -> Self {
        Self {
            input,
            len: code.len(),
            code,
            ip: 0,
        }
    }

    fn evaluate(&mut self) -> Result<i64, String> {
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
                    let mut buf = String::new();
                    print!("> ");
                    std::io::stdout().flush().unwrap();
                    self.input.read_line(&mut buf).expect("Failed to read line");
                    let val = buf.trim().parse::<i64>().expect("Failed to parse input");

                    self.code[dest] = val;
                    self.ip += 2;
                }

                OpCode::Output => {
                    println!("{}", param!(0));
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

                OpCode::Halt => return Ok(self.code[0]),
            }
        }

        Err("Didn't encounter Halt".into())
    }
}
