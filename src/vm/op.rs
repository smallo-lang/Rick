#![allow(dead_code)]

use super::VM;

pub struct Opcode {
    pub opcode_method: fn(&mut VM),
    pub operand_offset: usize,
}

/// INSTRUCTION_SET contains opcode instruction data for each available opcode
/// in the VM.
pub const INSTRUCTION_SET: [Opcode; 19] = [
    Opcode { opcode_method: VM::end, operand_offset: 0 },
    Opcode { opcode_method: VM::push, operand_offset: 4 },
    Opcode { opcode_method: VM::pop, operand_offset: 4 },
    Opcode { opcode_method: VM::drop, operand_offset: 0 },
    Opcode { opcode_method: VM::ini, operand_offset: 0 },
    Opcode { opcode_method: VM::ins, operand_offset: 0 },
    Opcode { opcode_method: VM::out, operand_offset: 0 },
    Opcode { opcode_method: VM::nl, operand_offset: 0 },
    Opcode { opcode_method: VM::sti, operand_offset: 0 },
    Opcode { opcode_method: VM::bool, operand_offset: 0 },
    Opcode { opcode_method: VM::add, operand_offset: 0 },
    Opcode { opcode_method: VM::sub, operand_offset: 0 },
    Opcode { opcode_method: VM::mul, operand_offset: 0 },
    Opcode { opcode_method: VM::div, operand_offset: 0 },
    Opcode { opcode_method: VM::r#mod, operand_offset: 0 },
    Opcode { opcode_method: VM::gth, operand_offset: 0 },
    Opcode { opcode_method: VM::lth, operand_offset: 0 },
    Opcode { opcode_method: VM::geq, operand_offset: 0 },
    Opcode { opcode_method: VM::leq, operand_offset: 0 },
];

/// This C-like enum is used to create versatile opcode tests that don't need
/// to get changed every time we alter VM's instruction set.
pub enum Op {
    End,
    Push,
    Pop,
    Drop,
    Ini,
    Ins,
    Out,
    Nl,
    Sti,
    Bool,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Gth,
    Lth,
    Geq,
    Leq,
    Not,
    And,
    Or,
    Eq,
    Neq,
    Con,
    Jum,
    Jmpt,
    Jmpf,
    Br,
    Brt,
    Brf,
    Bac,
    Err,
}

impl Op {
    pub fn op(self) -> u8 {
        self as u8
    }
}
