use std::{env, process};

mod rick;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: source file not specified");
        process::exit(1);
    }

    let src = &args[1];
    let vm = rick::init(src);
    vm.boot();
}
