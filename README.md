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
@ stack
push id             @ push object from memory onto the stack
pop id              @ pop object into memory
alu                 @ pop integer into ALU
spu                 @ pop string into SU
drop                @ drop value from stack


@ arithmetic-logic unit (ALU)
@ result will be pushed onto the stack immediately
    @ unary integer operations
its                 @ integer-to-string conversion

    @ integer arithmetic operations
add                 @ add
sub                 @ subtract right from left
mul                 @ multiply
div                 @ divide left by right
mod                 @ remainder of division of left by right

    @ integer comparisons
eq                  @ left equal to right
neq                 @ left not equal to right
gth                 @ left greater than right
lth                 @ left less than right
geq                 @ left greater than or equal to right
leq                 @ left less than or equal to right

    @ boolean operations
not
and
or


@ I/O operations
ini                 @ input integer and push onto stack
ins                 @ input string and push onto stack
out                 @ output value from the top of the stack


@ string processing unit
@ result will be pushed onto the stack immediately
    @ unary string operations
sti                 @ string-to-integer conversion

    @ binary string operations
seq                 @ left string equal to right
sneq                @ left string not equal to right
con                 @ concatenate two values as strings


@ control flow
jump                @ unconditional jump to code locations saved in memory
jmpt                @ jump if object at the top of the stack is true
jmpf                @ jump if object at the top of the stack is false
back                @ return to previous branch point
err                 @ exit program with exit code
end                 @ exit program
```


### Symbol Map

| Symbol | Meaning             |
|:------:|:--------------------|
| *      | label identifier    |
| #      | integer             |
| $      | string              |
| id     | memory identifier   |
| val    | type-blind value    |
| @      | comment             |

> The `val` represents constant literal (`#` or `$`) or variable identifier
> (`var`). It's used in commands that support auto-conversion.