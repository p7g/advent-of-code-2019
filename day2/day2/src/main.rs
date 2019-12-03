use std::convert::TryFrom;
use std::fs::read_to_string;

#[repr(u8)]
enum OpCode {
    Add = 1,
    Mul = 2,
    Halt = 99,
}

impl TryFrom<i64> for OpCode {
    type Error = String;

    fn try_from(n: i64) -> Result<OpCode, Self::Error> {
        Ok(match n {
            1 => OpCode::Add,
            2 => OpCode::Mul,
            99 => OpCode::Halt,

            _ => return Err(format!("Invalid opcode {}", n)),
        })
    }
}

fn main() -> Result<(), String> {
    let contents = read_to_string("../input.txt").expect("Failed to read file");

    let input = contents
        .trim()
        .split(',')
        .map(|num| num.parse::<i64>().expect("Failed to parse int"))
        .collect::<Vec<_>>();

    let mut code = input.clone();

    code[1] = 12;
    code[2] = 2;

    println!("{}", VM::new(code).evaluate()?);

    'outer: for i in 0..100 {
        for j in 0..100 {
            let mut code = input.clone();
            code[1] = i;
            code[2] = j;
            let result = VM::new(code).evaluate()?;

            if result == 19_690_720 {
                println!("{}", 100 * i + j);
                break 'outer;
            }
        }
    }

    Ok(())
}

struct VM {
    code: Vec<i64>,
    ip: usize,
    len: usize,
}

impl VM {
    fn new(code: Vec<i64>) -> Self {
        Self {
            len: code.len(),
            code,
            ip: 0,
        }
    }

    fn evaluate(&mut self) -> Result<i64, String> {
        while self.ip < self.len {
            match OpCode::try_from(self.code[self.ip])? {
                OpCode::Add => {
                    let left = self.code[self.code[self.ip + 1] as usize];
                    let right = self.code[self.code[self.ip + 2] as usize];
                    let dest = self.code[self.ip + 3] as usize;
                    self.code[dest] = left + right;
                    self.ip += 4;
                }
                OpCode::Mul => {
                    let left = self.code[self.code[self.ip + 1] as usize];
                    let right = self.code[self.code[self.ip + 2] as usize];
                    let dest = self.code[self.ip + 3] as usize;
                    self.code[dest] = left * right;
                    self.ip += 4;
                }
                OpCode::Halt => return Ok(self.code[0]),
            }
        }

        Err("Didn't encounter Halt".into())
    }
}
