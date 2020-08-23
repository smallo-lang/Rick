extern crate serde_json;
use serde_json as sj;
use serde_json::Value;

#[derive(Debug, PartialEq)]
enum Obj {
    Int(i64),
    Str(String),
}

pub struct VM {
    mem: Vec<Obj>,
    instructions: Vec<u8>,
}

pub fn new(data: &Vec<u8>) -> Result<VM, &'static str> {
    if !watermark_ok(data) {
        return Err("watermark check failed");
    }

    Ok(VM{
        mem: read_mem(data)?,
        instructions: read_instructions(data)?,
    })
}

fn watermark_ok(data: &Vec<u8>) -> bool {
    data.starts_with("Rick\0".as_bytes())
}

fn read_mem(data: &Vec<u8>) -> Result<Vec<Obj>, &'static str> {
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

fn json_into_obj(json_vals: Vec<Value>) -> Result<Vec<Obj>, &'static str> {
    let mut obj_vals: Vec<Obj> = Vec::new();

    for val in json_vals.iter() {
        let conv = obj_type(val)?;
        obj_vals.push(conv);
    }

    Ok(obj_vals)
}

fn obj_type(json_val: &Value) -> Result<Obj, &'static str> {
    if json_val.is_null() {
        return Ok(Obj::Int(0));
    } else if json_val.is_number() {
        return Ok(Obj::Int(json_val.as_i64().unwrap()));
    } else if json_val.is_string() {
        return Ok(Obj::Str(String::from(json_val.as_str().unwrap())));
    } else if json_val.is_boolean() {
        return Ok(if json_val.as_bool().unwrap() { Obj::Int(1) }
                  else { Obj::Int(0) })
    }
    Err("invalid JSON type used in memory")
}

fn read_instructions(data: &Vec<u8>) -> Result<Vec<u8>, &'static str> {
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

impl VM {
    pub fn boot(&self) {
        println!("Rick is booting...");
        println!("mem space: {}", self.mem.len());
        println!("instructions space: {}", self.instructions.len());
    }
}

#[cfg(test)]
mod new_tests {
    use super::*;

    #[test]
    fn test_fails_on_invalid_watermark() {
        let data = "Rock\0{}\0\0".as_bytes().to_vec();
        if let Ok(_) = new(&data) {
            panic!("expected Err");
        }
    }
}

#[cfg(test)]
mod watermark_ok_tests {
    use super::*;

    #[test]
    fn test_fails_on_invalid_watermark() {
        let data = "Rock\0{}\0\0".as_bytes().to_vec();
        assert!(!watermark_ok(&data));
    }

    #[test]
    fn test_works_on_valid_watermark() {
        let data = "Rick\0{}\0\0".as_bytes().to_vec();
        assert!(watermark_ok(&data));
    }
}

#[cfg(test)]
mod read_mem_tests {
    use super::*;

    #[test]
    fn test_fails_on_wrong_data_format() {
        let data = "Rick\0{}\0\0".as_bytes().to_vec();
        if let Ok(_) = read_mem(&data) {
            panic!("expected Err");
        }
    }

    #[test]
    fn test_fails_for_unexpected_values() {
        let data = "Rick\0[[\"hello\", 32], 42]\0\0".as_bytes().to_vec();
        if let Ok(_) = read_mem(&data) {
            panic!("expected Err");
        }
    }

    #[test]
    fn test_works_on_right_data() {
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
    fn test_fails_with_no_instructions() {
        let data = "Rick\0[]\0".as_bytes().to_vec();
        assert_eq!(Err("empty instructions list"), read_instructions(&data));
    }

    #[test]
    fn test_works_with_good_instructions() {
        let data = "Rick\0[]\0\0".as_bytes().to_vec();
        assert_eq!(Ok("\0".as_bytes().to_vec()), read_instructions(&data));
    }
}
