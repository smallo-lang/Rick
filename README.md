# Rick

Rick is a virtual machine written specifically for the [SmallO] programming
language. It is fast and reliable.

[SmallO]: https://github.com/sharpvik/SmallO



## Type System

SmallO supports two simple types: `integer` and `string` where `integer` also
serves as `boolean`. The `string` type can be interpreted as `boolean` using its
length (empty `string` gives `false`, non-empty one gives `true`).



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
back                @ return to previous branch point
err                 @ exit program with exit code
```


### Symbol Map

| Symbol | Meaning                                      |
|:------:|:---------------------------------------------|
| id     | memory identifier (label, const or variable) |
| @      | comment                                      |

> The `val` represents constant literal (`#` or `$`) or variable identifier
> (`var`). It's used in commands that support auto-conversion.
