#[macro_use] extern crate text_io;

mod util;
mod vm;

fn main() {
    let mut src = String::from("");
    util::args(&mut src);
    if src.len() == 0 {
        util::exit_with_err("source path not specified");
    }

    let data = util::read_src_into_bytes(&src);
    util::exit_on_err(&data);

    let data = data.unwrap();
    let vm = vm::VM::new(&data);
    util::exit_on_err(&vm);

    vm.unwrap().boot();
}

