use std::{env};

mod util;

fn main() {
    let args = env::args().collect();
    let src = util::src(&args);
    util::exit_on_err(&src);
    println!("Executing {}...", src.unwrap());
}
