MARKER = '$'

PRECEDENCE = {
    op: p
    for p, ops in enumerate(
        reversed([
            {'**', 'abs', 'not'},
            {'*', '/', 'mod', 'rem'},
            {'+\'', '-\''},
            {'+"', '-"', '&'},
            {'<', '<=', '=', '/>', '>', '>='},
            {'and', 'or', 'xor'},
        ])) for op in ops
}

PREFIX = {'abs', 'not', '+\'', '-\''}

VARIABLES = {chr(i) for i in range(ord('a'), ord('z') + 1)}
CONSTANTS = {chr(i) for i in range(ord('0'), ord('9') + 1)}

ALL_TOKENS = set(PRECEDENCE) | VARIABLES | CONSTANTS | {'(', ')', MARKER}
