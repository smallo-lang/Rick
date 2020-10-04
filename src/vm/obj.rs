use std::fmt;

extern crate serde_json;
use serde_json::Value;

use crate::util::TResult;

#[derive(Clone, Debug, PartialEq)]
pub enum Obj {
    Null,
    Int(i64),
    Str(String),
}

impl fmt::Display for Obj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Obj::Null => write!(f, "null"),
            Obj::Int(i) => write!(f, "{}", i),
            Obj::Str(s) => write!(f, "{}", s),
        }
   }
}

impl Obj {
    pub fn from_json(json_val: &Value) -> TResult<Obj> {
        match json_val {
            Value::Null => Ok(Obj::Null),
            Value::Number(n) => if n.is_i64() {
                Ok(Obj::Int(n.as_i64().unwrap()))
            } else {
                Err("invalid JSON type used in memory")
            },
            Value::String(s) => Ok(Obj::Str(s.clone())),
            Value::Bool(b) => Ok(Obj::Int(*b as i64)),
            _ => Err("invalid JSON type used in memory"),
        }
    }

    pub fn is_int(&self) -> bool {
        match self {
            Obj::Int(_) => true,
            _ => false
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Obj::Int(i) => Some(*i),
            _ => None
        }
    }

    pub fn to_bool(&self) -> Option<Obj> {
        match self {
            Obj::Int(i) => Some(Obj::Int((*i != 0) as i64)),
            Obj::Str(s) => Some(Obj::Int((s.len() != 0) as i64)),
            Obj::Null => None,
        }
    }

    pub fn equal(&self, other: &Obj) -> bool {
        match (self, other) {
            (Obj::Int(i), Obj::Int(j)) => i == j,
            (Obj::Str(s), Obj::Str(t)) => s == t,
            _ => false,
        }
    }
}
