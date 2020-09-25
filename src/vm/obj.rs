use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Obj {
    Int(i64),
    Str(String),
}

impl fmt::Display for Obj {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Obj::Int(i) => write!(f, "{}", i),
            Obj::Str(s) => write!(f, "{}", s),
        }
   }
}

impl Obj {
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
}
