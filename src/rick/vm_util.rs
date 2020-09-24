extern crate serde_json;
use serde_json as sj;
use serde_json::Value;

use crate::util::TResult;
use super::obj::Obj;

pub fn watermark_ok(data: &Vec<u8>) -> bool {
    data.starts_with("Rick\0".as_bytes())
}

pub fn read_mem(data: &Vec<u8>) -> TResult<Vec<Obj>> {
    // As per specification, watermark "Rick\0" is 5 bytes long, therefore,
    // we start reading JSON mem data at index 5.
    let mut end: usize = 5;
    while data[end] != b'\0' {
        end += 1;
    }

    let vals: sj::Result<Vec<Value>> = serde_json::from_slice(&data[5..end]);
    match vals {
        Err(e) => {
            println!("{}", e);
            Err("invalid memory value")
        },
        Ok(v) => Ok(json_into_obj(v)?),
    }
}

pub fn json_into_obj(json_vals: Vec<Value>) -> TResult<Vec<Obj>> {
    let mut obj_vals: Vec<Obj> = Vec::new();

    for val in json_vals.iter() {
        let conv = obj_type(val)?;
        obj_vals.push(conv);
    }

    Ok(obj_vals)
}

pub fn obj_type(json_val: &Value) -> TResult<Obj> {
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

pub fn read_instructions(data: &Vec<u8>) -> TResult<Vec<u8>> {
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
