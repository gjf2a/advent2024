use std::collections::{BTreeMap, VecDeque};

use advent2024::{advent_main, all_lines, Part};
use num::Integer;

fn main() -> anyhow::Result<()> {
    advent_main(|filename, part, options| {
        let mut program = Program::new(filename)?;
        match part {
            Part::One => {
                if options.contains(&"-program") {
                    program.print_program_listing();
                }
                let outputs = program.by_ref().map(|n| n.to_string()).collect::<Vec<_>>();
                println!("{}", outputs.join(","));
                if options.contains(&"-numinst") {
                    println!("Executed {} instructions", program.instructions_executed);
                }
            }
            Part::Two => {
                if options.contains(&"-mina") {
                    lowest_a_for_each(&program);
                } else {
                    part2(program);
                }
            }
        }
        Ok(())
    })
}

fn part2(program: Program) {
    let a_start = RegisterAFinder::new(&program).last().unwrap();
    let outputs = program.with_a(a_start).collect::<Vec<_>>();
    assert_eq!(outputs, program.program);
    println!("{a_start}");
}

#[derive(Debug, Clone)]
struct Program {
    a: u64,
    b: u64,
    c: u64,
    pc: usize,
    program: Vec<u8>,
    instructions_executed: usize,
}

impl Program {
    fn new(filename: &str) -> anyhow::Result<Self> {
        let mut lines = all_lines(filename)?;
        let a = num_from_end(lines.next().unwrap())?;
        let b = num_from_end(lines.next().unwrap())?;
        let c = num_from_end(lines.next().unwrap())?;
        let program = nums_from_end(lines.skip(1).next().unwrap());

        Ok(Self {
            a,
            b,
            c,
            pc: 0,
            program,
            instructions_executed: 0,
        })
    }

    fn with_a(&self, a: u64) -> Self {
        let mut result = self.clone();
        result.a = a;
        result
    }

    fn finished(&self) -> bool {
        self.pc >= self.program.len()
    }

    fn combo(&self, op: u8) -> u64 {
        match op {
            0..=3 => op as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Undefined op: {op}"),
        }
    }

    fn div(&self, op: u64) -> u64 {
        self.a / 2_u64.pow(op as u32)
    }

    fn execute_one_instr(&mut self) -> Option<u8> {
        assert!(!self.finished());
        let operand = self.program[self.pc + 1];
        let mut pc = self.pc + 2;
        let mut output = None;
        match self.program[self.pc] {
            0 => self.a = self.div(self.combo(operand)),
            1 => self.b = self.b ^ operand as u64,
            2 => self.b = self.combo(operand).mod_floor(&8),
            3 => {
                if self.a != 0 {
                    pc = operand as usize;
                }
            }
            4 => self.b = self.b ^ self.c,
            5 => output = Some(self.combo(operand).mod_floor(&8) as u8),
            6 => self.b = self.div(self.combo(operand)),
            7 => self.c = self.div(self.combo(operand)),
            _ => panic!("unrecognized instruction: {}", self.program[self.pc]),
        }
        self.pc = pc;
        self.instructions_executed += 1;
        output
    }

    fn print_program_listing(&self) {
        for i in (0..self.program.len()).step_by(2) {
            let literal = format!("{}", self.program[i + 1]);
            let combo = format!("{}", match self.program[i + 1] {
                0..=3 => char::from_digit(self.program[i + 1] as u32, 10).unwrap(),
                4 => 'a',
                5 => 'b',
                6 => 'c',
                _ => panic!("Unrecognized"),
            });
            let (opcode, operand) = match self.program[i] {
                0 => ("adv", combo),
                1 => ("bxl", literal),
                2 => ("bst", combo),
                3 => ("jnz", literal),
                4 => ("bxc", String::new()),
                5 => ("out", combo),
                6 => ("bdv", combo),
                7 => ("cdv", combo),
                _ => panic!("Should never happen."),
            };
            println!("{opcode}\t{operand}");
        }
    }
}

impl Iterator for Program {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.finished() {
            if let Some(output) = self.execute_one_instr() {
                return Some(output);
            }
        }
        None
    }
}

fn num_from_end(s: String) -> anyhow::Result<u64> {
    Ok(s.split_whitespace()
        .skip(2)
        .next()
        .unwrap()
        .parse::<u64>()?)
}

fn nums_from_end(s: String) -> Vec<u8> {
    s.split_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .map(|ns| ns.parse::<u8>().unwrap())
        .collect()
}

struct RegisterAFinder {
    table: BTreeMap<Vec<u8>, u64>,
    pending: VecDeque<u8>,
    incremental_a: Option<u64>,
}

impl RegisterAFinder {
    fn new(program: &Program) -> Self {
        let mut table = BTreeMap::new();
        for potential_a in 0..(2_u64.pow(24)) {
            let outputs = program.with_a(potential_a).collect::<Vec<_>>();
            if !table.contains_key(&outputs) {
                if outputs.len() <= 2 || outputs.contains(&4) && outputs.len() <= 4 {
                println!("{outputs:?}");}
                table.insert(outputs, potential_a);
            }
        }
        let pending = program.program.iter().copied().collect();
        let incremental_a = Some(0);
        Self {table, pending, incremental_a}
    }
}

impl Iterator for RegisterAFinder {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.incremental_a;
        self.incremental_a = match self.pending.pop_front() {
            None => None,
            Some(front) => {
                let mut pending_key = vec![front];
                while !self.table.contains_key(&pending_key) {
                    println!("pending_key: {pending_key:?} pending: {:?}", self.pending);
                    pending_key.push(self.pending.pop_front().unwrap());
                }
                self.table.get(&pending_key).copied()
            }
        };
        result
    }
}

fn lowest_a_for_each(program: &Program) {
    let mut lowest_as = BTreeMap::new();
    let mut a = 0;
    while lowest_as.len() < 8 {
        let output = program.with_a(a).collect::<Vec<_>>();
        for target in 0..8 {
            if !lowest_as.contains_key(&target) && output.contains(&target) {
                lowest_as.insert(target, (a, output));
                break;
            }
        }
        a += 1;
    }
    println!("{lowest_as:?}");
}