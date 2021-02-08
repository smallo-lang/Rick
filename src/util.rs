use std::process;
use std::io::prelude::*;
use std::fs::File;

extern crate colored;
use colored::*;

extern crate argparse;
use argparse::{ArgumentParser, Store};

pub type TResult<T> = Result<T, &'static str>;

pub fn args(src: &mut String) {
    let mut ap = ArgumentParser::new();
    ap.set_description("Execute SmallO bytecode");
    ap.refer(src)
        .add_argument("source", Store,
                      "Path to SmallO assembly source code");
    ap.parse_args_or_exit();
}

pub fn exit_on_err<T>(res: &TResult<T>) {
    if let Err(err) = res {
        exit_with_err(err);
    }
}

pub fn exit_with_err(err: &'static str) {
    println!("{}", format!("Error: {}", err).red());
    process::exit(1);
}

pub fn read_src_into_bytes(src: &String) -> TResult<Vec<u8>> {
    let file = File::open(src);
    match file {
        Err(_) => Err("failed to open executable"),
        Ok(mut f) => {
            let mut buf = Vec::new();
            match f.read_to_end(&mut buf) {
                Err(_) => Err("failed to read executable"),
                Ok(_) => Ok(buf)
            }
        }
    }
}

#[cfg(test)]
mod read_src_into_bytes_tests {
    use super::*;

    #[test]
    fn fails_if_file_not_found() {
        let path = "/home/sharpvik/Projects/Rick/executables/non-found.rk";
        let filename = String::from(path);
        assert_eq!(
            Err("failed to open executable"), read_src_into_bytes(&filename));
    }

    #[test]
    fn reads_file() {
        let path = "examples/bytecode/nop.rk";
        let filename = String::from(path);
        if let Ok(data) = read_src_into_bytes(&filename) {
            let expect: Vec<u8> = String::from("Rick\0[\"hello world\"]\0\0")
                .into_bytes();
            assert_eq!(expect, data);
        } else {
            panic!("expected Err");
        }
    }
}

