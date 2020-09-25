import json


WATERMARK = b'Rick\0'

# Instructions.
END     = b'\x00'
PUSH    = b'\x01'
POP     = b'\x02'
DROP    = b'\x03'
INI     = b'\x04'
INS     = b'\x05'
OUT     = b'\x06'
NL      = b'\x07'
STI     = b'\x08'
BOOL    = b'\x09'
ADD     = b'\x0A'
SUB     = b'\x0B'
MUL     = b'\x0C'
DIV     = b'\x0D'
MOD     = b'\x0E'
UNKNOWN = b'\xFF'


def mem(*args):
    return json.dumps(args).encode() + b'\0'


def code(mem, *ins):
    return WATERMARK + mem + b''.join(ins) + END


def i(n):
    return n.to_bytes(4, 'big')


if __name__ == '__main__':
    c = code(
        mem("Input your age: ", 2020, "You were born in "),
        PUSH, i(0),     # [ "Input your age: " ]
        OUT, DROP,      # > Input your age:
        PUSH, i(1),     # [ 2020 ]
        INI,            # [ 2020 | #age ]
        SUB,            # [ #year ]
        PUSH, i(2),     # [ #year | "You were born in " ]
        OUT, DROP,      # > You were born in:
        OUT, DROP,      # > #year
        NL,
    )
    with open('year_of_birth.rk', 'wb') as file:
        file.write(c)
