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
use op::INSTRUCTION_SET;

mod obj;
use obj::Obj;

mod vm_util;

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

// Main methods.
// These methods provide VM's basic functionality. Opcode execution is
// impossible without these very important things.
impl VM {
    pub fn new(bytecode: &Vec<u8>) -> TResult<Self> {
        if !vm_util::watermark_ok(bytecode) {
            return Err("watermark check failed");
        }

        Ok(Self{
            run: true,
            err: false,
            err_msg: String::from(""),
            mem: vm_util::read_mem(bytecode)?,
            instructions: vm_util::read_instructions(bytecode)?,
            ip: 0,
            opcode: 0,
            operand: 0,
            stack: Stack::new(),
        })
    }

    pub fn boot(&mut self) {
        while self.run && !self.err {
            self.tick();
        }
        self.exit();
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
    }

    pub fn fetch(&mut self) {
        if self.ip_out_of_bounds() {
            self.error("instruction pointer out of bounds");
        } else {
            self.opcode = self.instructions[self.ip];
            self.ip += 1;
        }
    }

    pub fn decode(&mut self) {
        if self.opcode_is_unknown() {
            self.error("unknown opcode");
        } else {
            self.decode_operand();
        }
    }

    pub fn execute(&mut self) {
        self.opcode_method()(self);
    }

    fn ip_out_of_bounds(&self) -> bool {
        self.ip >= self.instructions.len()
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

    fn decode_operand(&mut self) {
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

    fn binary_pop(&mut self) -> Option<(Obj, Obj)> {
        let top_b = self.stack.pop();
        let top_a = self.stack.pop();

        if self.pop_is_empty(&top_b) || self.pop_is_empty(&top_a) {
            return None;
        }
        Some((top_a.unwrap(), top_b.unwrap()))
    }

    fn pop_is_empty(&self, option: &Option<Obj>) -> bool {
        if let None = option {
            return true
        }
        false
    }

    fn two_obj_as_int(&self, a: &Obj, b: &Obj) -> Option<(i64, i64)> {
        if !a.is_int() || !b.is_int() {
            return None;
        }
        Some((a.as_int().unwrap(), b.as_int().unwrap()))
    }

    fn binary_int_op(&mut self, name: &'static str, op: fn(i64, i64) -> i64) {
        let objects = self.binary_pop();
        if let None = objects {
            self.error(&format!("[{}] not enough values on the stack", name));
            return;
        }

        let (obj_a, obj_b) = objects.unwrap();
        let integers = self.two_obj_as_int(&obj_a, &obj_b);
        if let None = integers {
            self.error(&format!("[{}] type mismatch: {} & {}",
                                name, obj_a, obj_b));
            return;
        }

        let (a, b) = integers.unwrap();
        self.stack.push(Obj::Int(op(a, b)));
    }
}

// Opcode methods.
// All opcode methods must be of type fn(&mut self) -> () since they are kept track
// of by the op::Opcode struct which defines that type.
impl VM {
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
        match self.stack.pop() {
            None => self.error("[out] pop attempt on an empty stack"),
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
            self.error("[sti] invalid stack top type (string expected)");
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

        match obj.to_bool() {
            Some(b) => self.stack.push(b),
            None => self.error("[bool] attempt to convert null"),
        }
    }

    fn add(&mut self) {
        self.binary_int_op("add", |a, b| a + b);
    }

    fn sub(&mut self) {
        self.binary_int_op("sub", |a, b| a - b);
    }

    fn mul(&mut self) {
        self.binary_int_op("mul", |a, b| a * b);
    }

    fn div(&mut self) {
        self.binary_int_op("div", |a, b| a / b);
    }

    fn r#mod(&mut self) {
        self.binary_int_op("mod", |a, b| a % b);
    }

    fn gth(&mut self) { self.binary_int_op("gth", |a, b| (a > b) as i64); }

    fn lth(&mut self) { self.binary_int_op("gth", |a, b| (a < b) as i64); }

    fn geq(&mut self) { self.binary_int_op("gth", |a, b| (a >= b) as i64); }

    fn leq(&mut self) { self.binary_int_op("gth", |a, b| (a <= b) as i64); }
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
    use op::Op;

    #[test]
    fn push() {
        let data: Vec<u8> = vec![
            b'R', b'i', b'c', b'k', 0,
            // mem: [42]
            b'[', b'4', b'2', b']', 0,
            Op::Push.op(), 0, 0, 0, 0,
            Op::Push.op(), 0, 0, 0, 1,
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
            Op::Pop.op(), 0, 0, 0, 0,
            Op::Pop.op(), 0, 0, 0, 0,
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
            Op::Drop.op(),
            Op::Drop.op(),
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
            Op::Sti.op(),
            Op::Sti.op(),
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
            Op::Bool.op(),
            Op::Bool.op(),
            Op::Bool.op(),
            Op::Bool.op(),
            Op::Bool.op(),
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

    #[test]
    fn add() {
        let data: Vec<u8> = vec![
            b'R', b'i', b'c', b'k', 0,
            // mem: []
            b'[', b']', 0,
            Op::Add.op(),
            Op::Add.op(),
            Op::Add.op(),
        ];
        let vm = VM::new(&data);
        if let Err(_) = vm {
            panic!("expected Ok");
        }

        let mut vm = vm.unwrap();

        vm.stack.push(Obj::Int(40));
        vm.stack.push(Obj::Int(2));
        vm.tick();
        assert_eq!(Some(Obj::Int(42)), vm.stack.pop());

        vm.stack.push(Obj::Int(40));
        vm.tick();
        assert!(vm.err);    // not enough elements for binary pop

        vm.err = false;
        vm.stack.push(Obj::Str(String::from("hello world")));
        vm.tick();
        assert!(vm.err);    // type mismatch
    }

    #[test]
    fn sub() {
        let data: Vec<u8> = vec![
            b'R', b'i', b'c', b'k', 0,
            // mem: []
            b'[', b']', 0,
            Op::Sub.op(),
            Op::Sub.op(),
            Op::Sub.op(),
        ];
        let vm = VM::new(&data);
        if let Err(_) = vm {
            panic!("expected Ok");
        }

        let mut vm = vm.unwrap();

        vm.stack.push(Obj::Int(44));
        vm.stack.push(Obj::Int(2));
        vm.tick();
        assert_eq!(Some(Obj::Int(42)), vm.stack.pop());

        vm.stack.push(Obj::Int(40));
        vm.tick();
        assert!(vm.err);    // not enough elements for binary pop

        vm.err = false;
        vm.stack.push(Obj::Str(String::from("hello world")));
        vm.tick();
        assert!(vm.err);    // type mismatch
    }

    #[test]
    fn mul() {
        let data: Vec<u8> = vec![
            b'R', b'i', b'c', b'k', 0,
            // mem: []
            b'[', b']', 0,
            Op::Mul.op(),
            Op::Mul.op(),
            Op::Mul.op(),
        ];
        let vm = VM::new(&data);
        if let Err(_) = vm {
            panic!("expected Ok");
        }

        let mut vm = vm.unwrap();

        vm.stack.push(Obj::Int(21));
        vm.stack.push(Obj::Int(2));
        vm.tick();
        assert_eq!(Some(Obj::Int(42)), vm.stack.pop());

        vm.stack.push(Obj::Int(40));
        vm.tick();
        assert!(vm.err);    // not enough elements for binary pop

        vm.err = false;
        vm.stack.push(Obj::Str(String::from("hello world")));
        vm.tick();
        assert!(vm.err);    // type mismatch
    }

    #[test]
    fn div() {
        let data: Vec<u8> = vec![
            b'R', b'i', b'c', b'k', 0,
            // mem: []
            b'[', b']', 0,
            Op::Div.op(),
            Op::Div.op(),
            Op::Div.op(),
        ];
        let vm = VM::new(&data);
        if let Err(_) = vm {
            panic!("expected Ok");
        }

        let mut vm = vm.unwrap();

        vm.stack.push(Obj::Int(84));
        vm.stack.push(Obj::Int(2));
        vm.tick();
        assert_eq!(Some(Obj::Int(42)), vm.stack.pop());

        vm.stack.push(Obj::Int(40));
        vm.tick();
        assert!(vm.err);    // not enough elements for binary pop

        vm.err = false;
        vm.stack.push(Obj::Str(String::from("hello world")));
        vm.tick();
        assert!(vm.err);    // type mismatch
    }

    #[test]
    fn r#mod() {
        let data: Vec<u8> = vec![
            b'R', b'i', b'c', b'k', 0,
            // mem: []
            b'[', b']', 0,
            Op::Mod.op(),
            Op::Mod.op(),
            Op::Mod.op(),
        ];
        let vm = VM::new(&data);
        if let Err(_) = vm {
            panic!("expected Ok");
        }

        let mut vm = vm.unwrap();

        vm.stack.push(Obj::Int(84));
        vm.stack.push(Obj::Int(2));
        vm.tick();
        assert_eq!(Some(Obj::Int(0)), vm.stack.pop());

        vm.stack.push(Obj::Int(40));
        vm.tick();
        assert!(vm.err);    // not enough elements for binary pop

        vm.err = false;
        vm.stack.push(Obj::Str(String::from("hello world")));
        vm.tick();
        assert!(vm.err);    // type mismatch
    }

    #[test]
    fn gth() {
        let data: Vec<u8> = vec![
            b'R', b'i', b'c', b'k', 0,
            // mem: []
            b'[', b']', 0,
            Op::Gth.op(),
            Op::Gth.op(),
            Op::Gth.op(),
            Op::Gth.op(),
        ];
        let vm = VM::new(&data);
        if let Err(_) = vm {
            panic!("expected Ok");
        }

        let mut vm = vm.unwrap();

        vm.stack.push(Obj::Int(84));
        vm.stack.push(Obj::Int(2));
        vm.tick();
        assert_eq!(Some(Obj::Int(1)), vm.stack.pop());

        vm.stack.push(Obj::Int(0));
        vm.stack.push(Obj::Int(2));
        vm.tick();
        assert_eq!(Some(Obj::Int(0)), vm.stack.pop());

        vm.stack.push(Obj::Int(40));
        vm.tick();
        assert!(vm.err);    // not enough elements for binary pop

        vm.err = false;
        vm.stack.push(Obj::Str(String::from("hello world")));
        vm.tick();
        assert!(vm.err);    // type mismatch
    }

    #[test]
    fn lth() {
        let data: Vec<u8> = vec![
            b'R', b'i', b'c', b'k', 0,
            // mem: []
            b'[', b']', 0,
            Op::Lth.op(),
            Op::Lth.op(),
            Op::Lth.op(),
            Op::Lth.op(),
        ];
        let vm = VM::new(&data);
        if let Err(_) = vm {
            panic!("expected Ok");
        }

        let mut vm = vm.unwrap();

        vm.stack.push(Obj::Int(84));
        vm.stack.push(Obj::Int(2));
        vm.tick();
        assert_eq!(Some(Obj::Int(0)), vm.stack.pop());

        vm.stack.push(Obj::Int(0));
        vm.stack.push(Obj::Int(2));
        vm.tick();
        assert_eq!(Some(Obj::Int(1)), vm.stack.pop());

        vm.stack.push(Obj::Int(40));
        vm.tick();
        assert!(vm.err);    // not enough elements for binary pop

        vm.err = false;
        vm.stack.push(Obj::Str(String::from("hello world")));
        vm.tick();
        assert!(vm.err);    // type mismatch
    }

    #[test]
    fn geq() {
        let data: Vec<u8> = vec![
            b'R', b'i', b'c', b'k', 0,
            // mem: []
            b'[', b']', 0,
            Op::Geq.op(),
            Op::Geq.op(),
            Op::Geq.op(),
            Op::Geq.op(),
            Op::Geq.op(),
        ];
        let vm = VM::new(&data);
        if let Err(_) = vm {
            panic!("expected Ok");
        }

        let mut vm = vm.unwrap();

        vm.stack.push(Obj::Int(84));
        vm.stack.push(Obj::Int(2));
        vm.tick();
        assert_eq!(Some(Obj::Int(1)), vm.stack.pop());

        vm.stack.push(Obj::Int(2));
        vm.stack.push(Obj::Int(2));
        vm.tick();
        assert_eq!(Some(Obj::Int(1)), vm.stack.pop());

        vm.stack.push(Obj::Int(0));
        vm.stack.push(Obj::Int(2));
        vm.tick();
        assert_eq!(Some(Obj::Int(0)), vm.stack.pop());

        vm.stack.push(Obj::Int(40));
        vm.tick();
        assert!(vm.err);    // not enough elements for binary pop

        vm.err = false;
        vm.stack.push(Obj::Str(String::from("hello world")));
        vm.tick();
        assert!(vm.err);    // type mismatch
    }

    #[test]
    fn leq() {
        let data: Vec<u8> = vec![
            b'R', b'i', b'c', b'k', 0,
            // mem: []
            b'[', b']', 0,
            Op::Leq.op(),
            Op::Leq.op(),
            Op::Leq.op(),
            Op::Leq.op(),
            Op::Leq.op(),
        ];
        let vm = VM::new(&data);
        if let Err(_) = vm {
            panic!("expected Ok");
        }

        let mut vm = vm.unwrap();

        vm.stack.push(Obj::Int(84));
        vm.stack.push(Obj::Int(2));
        vm.tick();
        assert_eq!(Some(Obj::Int(0)), vm.stack.pop());

        vm.stack.push(Obj::Int(2));
        vm.stack.push(Obj::Int(2));
        vm.tick();
        assert_eq!(Some(Obj::Int(1)), vm.stack.pop());

        vm.stack.push(Obj::Int(0));
        vm.stack.push(Obj::Int(2));
        vm.tick();
        assert_eq!(Some(Obj::Int(1)), vm.stack.pop());

        vm.stack.push(Obj::Int(40));
        vm.tick();
        assert!(vm.err);    // not enough elements for binary pop

        vm.err = false;
        vm.stack.push(Obj::Str(String::from("hello world")));
        vm.tick();
        assert!(vm.err);    // type mismatch
    }
}
