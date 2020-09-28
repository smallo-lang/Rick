# Rick

Rick is a virtual machine written specifically for the [SmallO] programming
language. It is fast and reliable.

[SmallO]: https://github.com/smallo-lang/



## Installation

I am assuming that you came here to use *Rick*, right? It's simple:

```bash
# Install using Cargo.
cargo install --path .

# Use it straight away!
rick executables/year_of_birth.rk
```



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
