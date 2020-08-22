use std::{env};

mod util;

fn main() {
    let args = env::args().collect();
    let src = util::src(&args);
    util::exit_on_err(&src);

    let src = src.unwrap();
    let data = util::read_src_into_bytes(&src);
    util::exit_on_err(&data);

    let data = data.unwrap();
    if !util::watermark_ok(&data) {
        util::exit_with_err("watermark check failed");
    }

    println!("OK: executing {}...", src);
}
