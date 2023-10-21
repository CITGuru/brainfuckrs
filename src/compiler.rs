/*
 Brainfuckrs ( http://github.com/citguru/brainfuckrs)
 Copyright (c) 2023 Oyetoke Toby

 Permission is hereby granted, free of charge, to any person obtaining a copy
 of this software and associated documentation files (the "Software"), to deal
 in the Software without restriction, including without limitation the rights
 to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 copies of the Software, and to permit persons to whom the Software is
 furnished to do so, subject to the following conditions:

 The above copyright notice and this permission notice shall be included in
 all copies or substantial portions of the Software.

 THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 THE SOFTWARE.
*/

use std::fs::File;
use std::io::{self, Read};
use std::process;

#[derive(Clone, Copy)]
pub struct Instruction {
    operator: u16,
    operand: u16,
}

const OP_INC_DP: u16 = 0;
const OP_DEC_DP: u16 = 1;
const OP_INC_VAL: u16 = 2;
const OP_DEC_VAL: u16 = 3;
const OP_OUT: u16 = 4;
const OP_IN: u16 = 5;
const OP_JMP_FWD: u16 = 6;
const OP_JMP_BCK: u16 = 7;

const DATA_SIZE: usize = 65535;

pub fn compile_bf(input: &str) -> Result<Vec<Instruction>, &'static str> {
    let mut program = Vec::new();
    let mut pc: u16 = 0;
    let mut jmp_pc: u16 = 0;
    let mut jmp_stack = Vec::new();
    for c in input.chars() {
        match c {
            '>' => program.push(Instruction {
                operator: OP_INC_DP,
                operand: 0,
            }),
            '<' => program.push(Instruction {
                operator: OP_DEC_DP,
                operand: 0,
            }),
            '+' => program.push(Instruction {
                operator: OP_INC_VAL,
                operand: 0,
            }),
            '-' => program.push(Instruction {
                operator: OP_DEC_VAL,
                operand: 0,
            }),
            '.' => program.push(Instruction {
                operator: OP_OUT,
                operand: 0,
            }),
            ',' => program.push(Instruction {
                operator: OP_IN,
                operand: 0,
            }),
            '[' => {
                program.push(Instruction {
                    operator: OP_JMP_FWD,
                    operand: 0,
                });
                jmp_stack.push(pc);
            }
            ']' => {
                if jmp_stack.is_empty() {
                    return Err("Compilation error.");
                }
                jmp_pc = jmp_stack.pop().unwrap();
                program.push(Instruction {
                    operator: OP_JMP_BCK,
                    operand: jmp_pc,
                });
                program[jmp_pc as usize].operand = pc;
            }
            _ => pc -= 1,
        }
        pc += 1;
    }
    if !jmp_stack.is_empty() {
        return Err("Compilation error.");
    }
    Ok(program)
}

pub fn execute_bf(program: &[Instruction]) -> io::Result<()> {
    let mut data = vec![0i16; DATA_SIZE];
    let mut data_ptr: u16 = 0;
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut pc = 0;
    while pc < program.len() {
        match program[pc].operator {
            OP_INC_DP => data_ptr += 1,
            OP_DEC_DP => data_ptr -= 1,
            OP_INC_VAL => data[data_ptr as usize] += 1,
            OP_DEC_VAL => data[data_ptr as usize] -= 1,
            OP_OUT => print!("{}", data[data_ptr as usize] as u8 as char),
            OP_IN => {
                let mut buffer = [0];
                reader.read_exact(&mut buffer)?;
                data[data_ptr as usize] = buffer[0] as i16;
            }
            OP_JMP_FWD => {
                if data[data_ptr as usize] == 0 {
                    pc = program[pc].operand as usize;
                }
            }
            OP_JMP_BCK => {
                if data[data_ptr as usize] > 0 {
                    pc = program[pc].operand as usize;
                }
            }
            _ => panic!("Unknown operator."),
        }
        pc += 1;
    }
    Ok(())
}

pub fn read_compile_from_file(filename: &str) -> io::Result<()> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let program = match compile_bf(&contents) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    execute_bf(&program)
}

pub fn read_compile_string(contents: &str) {

    let program = match compile_bf(&contents) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    execute_bf(&program);
}
