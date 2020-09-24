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
UNKNOWN = b'\xFF'


def mem(*args):
    return json.dumps(args).encode() + b'\0'


def code(mem, *ins):
    return WATERMARK + mem + b''.join(ins) + END


def i(n):
    return n.to_bytes(4, 'big')


if __name__ == '__main__':
    c = code(
        mem("Input your name: ", "Nice to meet you, "),
        PUSH, i(0),
        OUT, DROP,
        INS,
        PUSH, i(1),
        OUT, DROP,
        OUT, DROP,
        NL
    )
    with open('name.rk', 'wb') as file:
        file.write(c)
