use std::process;
use std::io::prelude::*;
use std::fs::File;

pub fn src(args: &Vec<String>) -> Result<&String, &'static str> {
    match args.len() {
        1 => Err("source file name not specified"),
        2 => Ok(&args[1]),
        _ => Err("invalid number of command line arguments")
    }
}

pub fn exit_on_err<T>(res: &Result<T, &'static str>) {
    if let Err(err) = res {
        println!("Error: {}", err);
        process::exit(1);
    }
}

pub fn read_src_into_bytes(src: &String) -> Result<Vec<u8>, &'static str> {
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
mod src_tests {
    use super::*;

    #[test]
    fn test_fails_if_src_not_specified() {
        let args = vec![String::from("rick")];
        assert_eq!(Err("source file name not specified"), src(&args));
    }

    #[test]
    fn test_fails_if_too_many_args() {
        let args = vec![
            String::from("rick"),
            String::from("example.rk"),
            String::from("extra"),
        ];
        assert_eq!(Err("invalid number of command line arguments"), src(&args));
    }

    #[test]
    fn test_returns_proper_src() {
        let args = vec![String::from("rick"), String::from("example.rk")];
        assert_eq!(Ok(&args[1]), src(&args));
    }
}

#[cfg(test)]
mod read_src_into_bytes_tests {
    use super::*;

    #[test]
    fn test_fails_if_file_not_found() {
        let path = "/home/sharpvik/Projects/Rick/executables/non-found.rk";
        let filename = String::from(path);
        assert_eq!(
            Err("failed to open executable"), read_src_into_bytes(&filename));
    }

    #[test]
    fn test_reads_file() {
        let path = "/home/sharpvik/Projects/Rick/executables/nop.rk";
        let filename = String::from(path);
        if let Ok(data) = read_src_into_bytes(&filename) {
            let expect: Vec<u8> = String::from("Rick\0{}\0\0").into_bytes();
            assert_eq!(expect, data);
        } else {
            panic!("call should have returned Ok");
        }
    }
}
