use std::convert::TryInto;

extern crate serde_json;
use serde_json as sj;
use serde_json::Value;

use crate::util::TResult;
mod stack;
use stack::Stack;

#[derive(Clone, Debug, PartialEq)]
enum Obj {
    Int(i64),
    Str(String),
}

pub struct VM {
    err: bool,
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
            err: false,
            mem: read_mem(data)?,
            instructions: read_instructions(data)?,
            ip: 0,
            opcode: 0,
            operand: 0,
            stack: Stack::new(),
        })
    }
}

fn watermark_ok(data: &Vec<u8>) -> bool {
    data.starts_with("Rick\0".as_bytes())
}

fn read_mem(data: &Vec<u8>) -> TResult<Vec<Obj>> {
    // As per specification, watermark "Rick\0" is 5 bytes long, therefore,
    // we start reading JSON mem data at index 5.
    let mut end: usize = 5;
    while data[end] != b'\0' {
        end += 1;
    }

    let vals: sj::Result<Vec<Value>> = serde_json::from_slice(&data[5..end]);
    match vals {
        Err(_) => Err("invalid memory type"),
        Ok(v) => Ok(json_into_obj(v)?),
    }
}

fn json_into_obj(json_vals: Vec<Value>) -> TResult<Vec<Obj>> {
    let mut obj_vals: Vec<Obj> = Vec::new();

    for val in json_vals.iter() {
        let conv = obj_type(val)?;
        obj_vals.push(conv);
    }

    Ok(obj_vals)
}

fn obj_type(json_val: &Value) -> TResult<Obj> {
    if json_val.is_null() {
        return Ok(Obj::Int(0));
    } else if json_val.is_i64() {
        return Ok(Obj::Int(json_val.as_i64().unwrap()));
    } else if json_val.is_string() {
        return Ok(Obj::Str(String::from(json_val.as_str().unwrap())));
    } else if json_val.is_boolean() {
        return Ok(if json_val.as_bool().unwrap() { Obj::Int(1) }
                  else { Obj::Int(0) })
    }
    Err("invalid JSON type used in memory")
}

fn read_instructions(data: &Vec<u8>) -> TResult<Vec<u8>> {
    let mut start: usize = 5;
    while data[start] != b'\0' {
        start += 1;
    }
    start += 1;

    let instructions = data[start..].to_vec();
    match instructions.len() {
        0 => Err("empty instructions list"),
        _ => Ok(instructions)
    }
}

struct InstructionData {
    opcode_method: fn(&mut VM),
    operand_offset: usize,
}

/// INSTRUCTION_SET contains opcode instruction data for each available opcode
/// in the VM.
const INSTRUCTION_SET: [InstructionData; 2] = [
    InstructionData { opcode_method: VM::push, operand_offset: 4 },
    InstructionData { opcode_method: VM::pop, operand_offset: 4 },
];

impl VM {
    pub fn boot(&self) {
        println!("Rick is booting...");
        println!("mem space: {}", self.mem.len());
        println!("instructions space: {}", self.instructions.len());
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

    pub fn tick(&mut self) {
        self.fetch();
        self.decode();
        self.execute();
    }

    pub fn fetch(&mut self) {
        self.opcode = self.instructions[self.ip];
        self.ip += 1;
    }

    pub fn decode(&mut self) {
        if self.opcode_is_unknown() {
            self.err = true;
            return;
        }

        let ip_with_offset = self.ip + self.operand_offset();

        let operand_bytes = &self
            .instructions[self.ip..ip_with_offset]
            .try_into()
            .expect("slice of incorrect length");
        self.operand = u32::from_be_bytes(*operand_bytes);

        self.ip = ip_with_offset;
    }

    pub fn execute(&mut self) {
        if self.err {
            return;
        }
        let method = self.opcode_method();
        method(self);
    }

    /* Opcode methods follow. */

    fn push(&mut self) {
        self.stack.push(self.mem[self.operand as usize].clone());
    }

    fn pop(&mut self) {
        match self.stack.pop() {
            None => self.err = true,
            Some(obj) => self.mem[self.operand as usize] = obj
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

    #[test]
    fn push() {
        // push 0
        let data: Vec<u8> = "Rick\0[42]\0\0\0\0\0\0".as_bytes().to_vec();
        let vm = VM::new(&data);
        if let Err(_) = vm {
            panic!("expected Ok");
        }

        let mut vm = vm.unwrap();
        vm.tick();
        assert_eq!(Some(Obj::Int(42)), vm.stack.pop());
    }

    #[test]
    fn pop() {
        // pop 0
        let data: Vec<u8> = vec![82, 105, 99, 107, 0, 91, 48, 93, 0,
            1, 0, 0, 0, 0];
        let vm = VM::new(&data);
        if let Err(_) = vm {
            panic!("expected Ok");
        }

        let mut vm = vm.unwrap();
        vm.stack.push(Obj::Int(42));
        vm.tick();
        assert_eq!(Obj::Int(42), vm.mem[0]);
    }
}

