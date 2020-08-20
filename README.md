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
drop                @ drop value from stack


@ conversions
sti                 @ string-to-integer conversion
bool                @ object-to-boolean conversion


@ integer operations
    @ integer arithmetic operations
add                 @ add
sub                 @ subtract right from left
mul                 @ multiply
div                 @ divide left by right
mod                 @ remainder of division of left by right

    @ integer comparisons
gth                 @ left greater than right
lth                 @ left less than right
geq                 @ left greater than or equal to right
leq                 @ left less than or equal to right

    @ boolean operations
not
and
or

@ untyped comparisons
eq                  @ left equal to right
neq                 @ left not equal to right


@ I/O operations
ini                 @ input integer and push onto stack
ins                 @ input string and push onto stack
out                 @ output value from the top of the stack


@ string operations
con                 @ concatenate two values from the top of the stack


@ control flow
jump                @ unconditional jump to code location on top of the stack
jmpt                @ jump if true
jmpf                @ jump if false
back                @ return to previous branch point
err                 @ exit program with exit code
end                 @ exit program
```


### Symbol Map

| Symbol | Meaning                                      |
|:------:|:---------------------------------------------|
| id     | memory identifier (label, const or variable) |
| @      | comment                                      |

> The `val` represents constant literal (`#` or `$`) or variable identifier
> (`var`). It's used in commands that support auto-conversion.
