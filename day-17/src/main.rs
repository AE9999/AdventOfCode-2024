use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input =  &args[1];

    let problem = read_input(input)?;

    solve(problem.clone());

    Ok(())
}

fn solve(mut problem: Problem) {
    problem.run();
    problem.dump_output();
}

#[derive(Debug, Clone)]
struct Problem {
    registers: Vec<usize>,
    program: Vec<usize>,
    output: Vec<usize>,
    pc: usize,
}

impl Problem {
    fn new(registers: Vec<usize>,
           program: Vec<usize>) -> Self {
        Problem { registers,
                  program,
                  output: Vec::new(),
                  pc: 0 }
    }

    fn dump_output(&self) {
        println!("{:?}", self.output.iter()
                                     .map(|num| num.to_string())
                                     .collect::<Vec<_>>().join(","));
    }

    fn run(&mut self) {
        while self.pc < self.program.len() {
            self.step()
        }
    }

    fn step(&mut self) {
        let opcode = self.program[self.pc];
        let operation = Operation::from_opcode(opcode);
        let operand = self.program[self.pc + 1];
        match operation {
            Operation::Adv => {
                let num = self.registers[0];
                let demoniator = 2_usize.pow( self.combo_operand_value(operand) as u32);
                self.registers[0] = num / demoniator;
                self.pc += 2
            },
            Operation::Bxl => {
                let l = self.registers[1];
                self.registers[1] = l ^ operand;
                self.pc += 2
            },
            Operation::Bst => {
                self.registers[1] = self.combo_operand_value(operand)  % 8;
                self.pc += 2
            },
            Operation::Jnz => {
                if self.registers[0] != 0 {
                    self.pc = operand
                } else {
                    self.pc += 2
                }
            },
            Operation::Bxc => {
                self.registers[1] = self.registers[1] ^ self.registers[2];
                self.pc += 2
            },
            Operation::Out => {
                let res=   self.combo_operand_value(operand ) % 8;
                self.output.push(res);
                self.pc += 2
            },
            Operation::Bdv => {
                let num = self.registers[0];
                let demoniator = 2_usize.pow( self.combo_operand_value(operand) as u32);
                self.registers[1] = num / demoniator;
                self.pc += 2
            },
            Operation::Cdv => {
                let num = self.registers[0];
                let demoniator = 2_usize.pow( self.combo_operand_value(operand) as u32);
                self.registers[2] = num / demoniator;
                self.pc += 2
            },
        }
    }

    fn combo_operand_value(&self, operand: usize) -> usize {
        match operand {
            0|1|2|3 => { operand }
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            7 => { panic!("reserved") }
            _ => { panic!("unexpected operand") }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Operation {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Operation {
    fn from_opcode(opcode: usize) -> Self {
        match opcode {
            0 => Operation::Adv,
            1 => Operation::Bxl,
            2 => Operation::Bst,
            3 => Operation::Jnz,
            4 => Operation::Bxc,
            5 => Operation::Out,
            6 => Operation::Bdv,
            7 => Operation::Cdv,
            _ => panic!("invalid opcode"),
        }
    }
}


fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;


    let mut it = BufReader::new(file_in).lines();
    let mut registers: Vec<usize> = Vec::new();
    registers.push(it.next().unwrap()?.split_whitespace().last().unwrap().parse::<usize>().unwrap());
    registers.push(it.next().unwrap()?.split_whitespace().last().unwrap().parse::<usize>().unwrap());
    registers.push(it.next().unwrap()?.split_whitespace().last().unwrap().parse::<usize>().unwrap());
    it.next();

    let program: Vec<usize> =
        it.next().unwrap()?.split_whitespace().last().unwrap().split(',')
                 .map(|x| x.parse::<usize>().unwrap())
                 .collect();

    Ok(Problem::new(registers, program))
}
