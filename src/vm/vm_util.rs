extern crate serde_json;
use serde_json as sj;
use serde_json::Value;

use crate::util::TResult;
use super::obj::Obj;

pub fn watermark_ok(bytecode: &Vec<u8>) -> bool {
    bytecode.starts_with("Rick\0".as_bytes())
}

pub fn read_mem(bytecode: &Vec<u8>) -> TResult<Vec<Obj>> {
    // As per specification, watermark "Rick\0" is 5 bytes long, therefore,
    // we start reading JSON mem data at index 5.
    let mut end: usize = 5;
    while bytecode[end] != b'\0' {
        end += 1;
    }

    let vals: sj::Result<Vec<Value>> = sj::from_slice(&bytecode[5..end]);
    match vals {
        Err(e) => {
            println!("{}", e);
            Err("invalid memory value")
        },
        Ok(v) => Ok(json_into_obj(v)?),
    }
}

fn json_into_obj(json_vals: Vec<Value>) -> TResult<Vec<Obj>> {
    let mut objects: Vec<Obj> = Vec::new();

    for val in json_vals.iter() {
        objects.push(Obj::from_json(val)?);
    }

    Ok(objects)
}

pub fn read_instructions(bytecode: &Vec<u8>) -> TResult<Vec<u8>> {
    let mut start: usize = 5;
    while bytecode[start] != b'\0' {
        start += 1;
    }
    start += 1;

    let instructions = bytecode[start..].to_vec();
    match instructions.len() {
        0 => Err("empty instructions list"),
        _ => Ok(instructions)
    }
}

#[cfg(test)]
mod vm_util_tests {
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
                Obj::Null,
                Obj::Int(1),
            ]),
        }
    }

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
