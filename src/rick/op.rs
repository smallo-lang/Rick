#![allow(dead_code)]

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
}

pub fn op(o: Op) -> u8 {
    return o as u8;
}
