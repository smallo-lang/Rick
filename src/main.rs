use std::{env};

mod util;

fn main() {
    let args = env::args().collect();
    let src = util::src(&args);
    util::exit_on_err(&src);

    let src = src.unwrap();
    let data = util::read_src_into_bytes(&src);
    util::exit_on_err(&data);
}
