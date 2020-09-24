use std::io;
use std::io::Write;
use std::process;
use std::convert::TryInto;


extern crate colored;
use colored::*;

use crate::util::TResult;

mod stack;
use stack::Stack;

mod op;

mod obj;
use obj::Obj;

mod vm_util;
use vm_util::*;

pub struct VM {
    run: bool,
    err: bool,
    err_msg: String,
    mem: Vec<Obj>,
    instructions: Vec<u8>,

    ip: usize,
    opcode: u8,
    operand: u32,

    stack: Stack<Obj>,
}

impl VM {
    pub fn new(data: &Vec<u8>) -> TResult<Self> {
        if !watermark_ok(data) {
            return Err("watermark check failed");
        }

        Ok(Self{
            run: true,
            err: false,
            err_msg: String::from(""),
            mem: read_mem(data)?,
            instructions: read_instructions(data)?,
            ip: 0,
            opcode: 0,
            operand: 0,
            stack: Stack::new(),
        })
    }
}

struct InstructionData {
    opcode_method: fn(&mut VM),
    operand_offset: usize,
}

/// INSTRUCTION_SET contains opcode instruction data for each available opcode
/// in the VM.
const INSTRUCTION_SET: [InstructionData; 10] = [
    InstructionData { opcode_method: VM::end, operand_offset: 0 },
    InstructionData { opcode_method: VM::push, operand_offset: 4 },
    InstructionData { opcode_method: VM::pop, operand_offset: 4 },
    InstructionData { opcode_method: VM::drop, operand_offset: 0 },
    InstructionData { opcode_method: VM::ini, operand_offset: 0 },
    InstructionData { opcode_method: VM::ins, operand_offset: 0 },
    InstructionData { opcode_method: VM::out, operand_offset: 0 },
    InstructionData { opcode_method: VM::nl, operand_offset: 0 },
    InstructionData { opcode_method: VM::sti, operand_offset: 0 },
    InstructionData { opcode_method: VM::r#bool, operand_offset: 0 },
];

impl VM {
    pub fn boot(&mut self) {
        while self.run && !self.err {
            self.tick();
        }
        self.exit();
    }

    fn opcode_is_unknown(&self) -> bool {
        self.opcode as usize >= INSTRUCTION_SET.len()
    }

    fn opcode_method(&self) -> fn(&mut Self) {
        INSTRUCTION_SET[self.opcode as usize].opcode_method
    }

    fn operand_offset(&self) -> usize {
        INSTRUCTION_SET[self.opcode as usize].operand_offset
    }

    fn exit(&self) {
        if self.err {
            let panic_msg = format!("Rick panicked at #{}!", self.ip).yellow();
            let error_msg = format!("Error: {}.", self.err_msg).red();
            println!("{}\n{}", panic_msg, error_msg);
        }
        process::exit(self.err as i32);
    }

    fn error(&mut self, msg: &str) {
        self.err = true;
        self.err_msg = String::from(msg);
    }

    pub fn tick(&mut self) {
        self.fetch();
        if self.err {
            return;
        }

        self.decode();
        if self.err {
            return;
        }

        self.execute();
        if self.err {
            return;
        }
    }

    pub fn fetch(&mut self) {
        if self.ip >= self.instructions.len() {
            self.error("instruction pointer out of bouds");
        } else {
            self.opcode = self.instructions[self.ip];
            self.ip += 1;
        }
    }

    pub fn decode(&mut self) {
        if self.opcode_is_unknown() {
            self.error("unknown opcode");
            return;
        }

        let operand_offset = self.operand_offset();
        if operand_offset == 0 {
            return;
        }

        let ip_with_offset = self.ip + operand_offset;

        let operand_bytes = &self
            .instructions[self.ip..ip_with_offset]
            .try_into()
            .expect("slice of incorrect length");
        self.operand = u32::from_be_bytes(*operand_bytes);

        self.ip = ip_with_offset;
    }

    pub fn execute(&mut self) {
        let method = self.opcode_method();
        method(self);
    }

    /* Opcode methods follow. */

    fn end(&mut self) {
        self.run = false;
    }

    fn push(&mut self) {
        let mp = self.operand as usize;
        if mp >= self.mem.len() {
            self.error("[push] memory pointer out of bounds");
        } else {
            self.stack.push(self.mem[mp].clone());
        }
    }

    fn pop(&mut self) {
        let mp = self.operand as usize;
        if mp >= self.mem.len() {
            self.error("[pop] memory pointer out of bounds");
        } else {
            match self.stack.pop() {
                None => self.error("[pop] pop attempt on an empty stack"),
                Some(obj) => self.mem[mp] = obj
            }
        }
    }

    fn drop(&mut self) {
        match self.stack.pop() {
            None => self.error("[drop] pop attempt on an empty stack"),
            Some(_) => ()
        }
    }

    fn ini(&mut self) {
        io::stdout().flush().unwrap();
        let result: Result<i64, _> = try_read!();
        match result {
            Err(_) => self.error("[ini] invalid string literal for conversion"),
            Ok(i) => self.stack.push(Obj::Int(i))
        }
    }

    fn ins(&mut self) {
        io::stdout().flush().unwrap();
        let s: String = read!();
        self.stack.push(Obj::Str(s));
    }

    fn out(&mut self) {
        match self.stack.peek() {
            None => self.error("[out] peek attempt on an empty stack"),
            Some(obj) => print!("{}", obj)
        }
    }

    fn nl(&mut self) {
        println!();
    }

    fn sti(&mut self) {
        let top = self.stack.pop();
        let obj: Obj;

        if let Some(o) = top {
            obj = o;
        } else {
            self.error("[sti] pop attempt on an empty stack");
            return;
        }

        if let Obj::Str(string) = obj { 
            match string.parse::<i64>() {
                Ok(integer) => self.stack.push(Obj::Int(integer)),
                Err(_) => self.error("[sti] failed to convert to int")
            }
        } else {
            self.error("[sti] invalid stack top type (str expected)");
        }
    }

    fn bool(&mut self) {
        let top = self.stack.pop();
        let obj: Obj;

        if let Some(o) = top {
            obj = o;
        } else {
            self.error("[bool] pop attempt on an empty stack");
            return;
        }

        match obj {
            Obj::Int(i) => self.stack.push(Obj::Int((i != 0) as i64)),
            Obj::Str(s) => self.stack.push(Obj::Int((s.len() != 0) as i64))
        }
    }
}

#[cfg(test)]
mod watermark_ok_tests {
    use super::*;

    #[test]
    fn fails_on_invalid_watermark() {
        let data = "Rock\0{}\0\0".as_bytes().to_vec();
        assert!(!watermark_ok(&data));
    }

    #[test]
    fn works_on_valid_watermark() {
        let data = "Rick\0{}\0\0".as_bytes().to_vec();
        assert!(watermark_ok(&data));
    }
}

#[cfg(test)]
mod read_mem_tests {
    use super::*;

    #[test]
    fn fails_on_wrong_data_format() {
        let data = "Rick\0{}\0\0".as_bytes().to_vec();
        if let Ok(_) = read_mem(&data) {
            panic!("expected Err");
        }
    }

    #[test]
    fn fails_for_unexpected_values() {
        let data = "Rick\0[[\"hello\", 32], 42]\0\0".as_bytes().to_vec();
        if let Ok(_) = read_mem(&data) {
            panic!("expected Err");
        }
    }

    #[test]
    fn works_on_right_data() {
        let data = "Rick\0[\"magic\", 42, null, true]\0\0".as_bytes().to_vec();
        let mem = read_mem(&data);
        match mem {
            Err(_) => panic!("expected Ok"),
            Ok(v) => assert_eq!(v, vec![
                Obj::Str(String::from("magic")),
                Obj::Int(42),
                Obj::Int(0),
                Obj::Int(1),
            ]),
        }
    }
}

#[cfg(test)]
mod read_instructions_tests {
    use super::*;

    #[test]
    fn fails_with_no_instructions() {
        let data = "Rick\0[]\0".as_bytes().to_vec();
        assert_eq!(Err("empty instructions list"), read_instructions(&data));
    }

    #[test]
    fn works_with_good_instructions() {
        let data = "Rick\0[]\0\0".as_bytes().to_vec();
        assert_eq!(Ok("\0".as_bytes().to_vec()), read_instructions(&data));
    }
}

#[cfg(test)]
mod vm_tests {
    use super::*;

    #[test]
    fn can_create_new_instance() {
        let data = "Rick\0[]\0\0".as_bytes().to_vec();
        if let Err(_) = VM::new(&data) {
            panic!("expected Ok");
        }
    }

    #[test]
    fn fails_on_unknown_instruction() {
        let data: Vec<u8> = "Rick\0[]\0~".as_bytes().to_vec();
        if let Ok(mut vm) = VM::new(&data) {
            vm.tick();
            assert!(vm.err);
        } else {
            panic!("expected Ok");
        }
    }
}

#[cfg(test)]
mod opcode_tests {
    use super::*;
    use op::Op::*;
    use op::op;

    #[test]
    fn push() {
        let data: Vec<u8> = vec![
            b'R', b'i', b'c', b'k', 0,
            // mem: [42]
            b'[', b'4', b'2', b']', 0,
            op(Push), 0, 0, 0, 0,
            op(Push), 0, 0, 0, 1,
        ];
        let vm = VM::new(&data);
        if let Err(_) = vm {
            panic!("expected Ok");
        }

        let mut vm = vm.unwrap();
        vm.tick();
        assert_eq!(Some(Obj::Int(42)), vm.stack.pop());
        vm.tick();
        assert!(vm.err);
    }

    #[test]
    fn pop() {
        let data: Vec<u8> = vec![
            b'R', b'i', b'c', b'k', 0,
            // mem: [0]
            b'[', b'0', b']', 0,
            op(Pop), 0, 0, 0, 0,
            op(Pop), 0, 0, 0, 0,
        ];
        let vm = VM::new(&data);
        if let Err(_) = vm {
            panic!("expected Ok");
        }

        let mut vm = vm.unwrap();
        vm.stack.push(Obj::Int(42));
        vm.tick();
        assert_eq!(Obj::Int(42), vm.mem[0]);
        vm.tick();
        assert!(vm.err);
    }

    #[test]
    fn drop() {
        let data: Vec<u8> = vec![
            b'R', b'i', b'c', b'k', 0,
            // mem: []
            b'[', b']', 0,
            op(Drop),
            op(Drop),
        ];
        let vm = VM::new(&data);
        if let Err(_) = vm {
            panic!("expected Ok");
        }

        let mut vm = vm.unwrap();
        vm.stack.push(Obj::Int(42));
        vm.tick();
        assert!(vm.stack.empty());
        vm.tick();
        assert!(vm.err);
    }

    #[test]
    fn sti() {
        let data: Vec<u8> = vec![
            b'R', b'i', b'c', b'k', 0,
            // mem: []
            b'[', b']', 0,
            op(Sti),
            op(Sti),
        ];
        let vm = VM::new(&data);
        if let Err(_) = vm {
            panic!("expected Ok");
        }

        let mut vm = vm.unwrap();
        vm.stack.push(Obj::Str(String::from("42")));
        vm.tick();
        assert_eq!(Some(Obj::Int(42)), vm.stack.pop());
        vm.stack.push(Obj::Str(String::from("invalid")));
        vm.tick();
        assert!(vm.err);
    }

    #[test]
    fn bool() {
        let data: Vec<u8> = vec![
            b'R', b'i', b'c', b'k', 0,
            // mem: []
            b'[', b']', 0,
            op(Bool),
            op(Bool),
            op(Bool),
            op(Bool),
            op(Bool),
        ];
        let vm = VM::new(&data);
        if let Err(_) = vm {
            panic!("expected Ok");
        }

        let mut vm = vm.unwrap();
        
        vm.stack.push(Obj::Str(String::from("hello world")));
        vm.tick();
        assert_eq!(Some(Obj::Int(1)), vm.stack.pop());

        vm.stack.push(Obj::Str(String::from("")));
        vm.tick();
        assert_eq!(Some(Obj::Int(0)), vm.stack.pop());

        vm.stack.push(Obj::Int(42));
        vm.tick();
        assert_eq!(Some(Obj::Int(1)), vm.stack.pop());

        vm.stack.push(Obj::Int(0));
        vm.tick();
        assert_eq!(Some(Obj::Int(0)), vm.stack.pop());

        vm.tick();
        assert!(vm.err);
    }
}
