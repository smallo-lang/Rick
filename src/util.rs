use std::process;

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

#[cfg(test)]
mod tests {
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
