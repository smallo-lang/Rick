use std::env;

#[macro_use] extern crate text_io;

mod util;
mod vm;

fn main() {
    let args = env::args().collect();
    let src = util::src(&args);
    util::exit_on_err(&src);

    let src = src.unwrap();
    let data = util::read_src_into_bytes(&src);
    util::exit_on_err(&data);

    let data = data.unwrap();
    let vm = vm::VM::new(&data);
    util::exit_on_err(&vm);

    vm.unwrap().boot();
}

