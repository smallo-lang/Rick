# Rick

Rick is a virtual machine written specifically for the [SmallO] programming
language. It is fast and reliable.

[SmallO]: https://github.com/smallo-lang/



## Installation

### Basics

I am assuming that you came here to use *Rick*, right? It's simple:

```bash
# Install using Cargo.
cargo install --path .

# Use it straight away!
rick executables/year_of_birth.rk
```


### Rust and Cargo

Oh no! Installation failed because you don't have [Rust] and *Cargo* installed?
Don't worry, follow [this link][Install] and install *Rust* using `rustup` as
recommended by the site.

[Rust]: https://www.rust-lang.org/
[Install]: https://www.rust-lang.org/tools/install


### Dependencies

> Alright, I have Rust and Cargo installed now, but what about project
> dependencies? Don't I have to install like a couple dozens of additional
> packages to be able to build this thing?
> &copy; Every Software Dev

Actually, you don't. Cargo will take care of all dependencies.


### Dealing With PATH

Upon installation, *Rust* will display a warning like this:

```bash
warning: be sure to add `/home/username/.cargo/bin` to your PATH to be able to
run the installed binaries
```

So yeah, you'll need to do just that. If you don't know what `PATH` is and how
to work with it, here are a couple of useful links for you, my friend:

- [What exactly is `PATH`](http://www.linfo.org/path_env_var.html)
- [How to add to `PATH` on Windows 10](https://www.architectryan.com/2018/03/17/add-to-the-path-on-windows-10/)



## Family

*Rick* has a family. There are different members, each with their own life.
Talk to [Jerry](https://github.com/smallo-lang/Jerry) as he can give you a
better overview of the household and how its members fit together.



## Operations

### Instruction Set

```asm
end                 @ exit program

push id             @ push object from memory onto the stack
pop id              @ pop object into memory
drop                @ drop value from stack

ini                 @ input integer and push onto stack
ins                 @ input string and push onto stack
out                 @ output value from the top of the stack
nl                  @ print a newline character

sti                 @ string-to-integer conversion
bool                @ object-to-boolean conversion

add                 @ add
sub                 @ subtract right from left
mul                 @ multiply
div                 @ divide left by right
mod                 @ remainder of division of left by right

gth                 @ left greater than right
lth                 @ left less than right
geq                 @ left greater than or equal to right
leq                 @ left less than or equal to right

not                 @ unary logical not
and                 @ binary logical and
or                  @ binary logical or

eq                  @ left equal to right
neq                 @ left not equal to right

con                 @ concatenate two top values from the the stack as strings

jump                @ unconditional jump to code location on top of the stack
jmpt                @ jump if true
jmpf                @ jump if false
br                  @ unconditional branch to code location
brt                 @ branch if true
brf                 @ branch if false
back                @ return to previous branch point
err                 @ exit program with exit code
```


### Symbol Map

| Symbol | Meaning                                      |
|:------:|:---------------------------------------------|
| id     | memory identifier (label, const or variable) |
| @      | comment                                      |



## License

This project is licensed under the **Mozilla Public License Version 2.0** --
see the [LICENSE](LICENSE) file for details.

Please note that this project is distributred as is,
**with absolutely no warranty of any kind** to those who are going to deploy
and/or use it. None of the authors and contributors are responsible (liable)
for **any damage**, including but not limited to, loss of sensitive data and
machine malfunction.
