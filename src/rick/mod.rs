pub struct VM<'a> {
    src: &'a str,
}

pub fn init(src: &str) -> VM {
    return VM {src: src};
}

impl VM<'_> {
    pub fn boot(&self) {
        println!("Executing {}...", self.src);
    }
}
