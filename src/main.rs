use std::env;

mod util;
mod rick;

fn main() {
    let args = env::args().collect();
    let src = util::src(&args);
    util::exit_on_err(&src);

    let src = src.unwrap();
    let data = util::read_src_into_bytes(&src);
    util::exit_on_err(&data);

    let data = data.unwrap();
    let vm = rick::VM::new(&data);
    util::exit_on_err(&vm);

    vm.unwrap().boot();
}

