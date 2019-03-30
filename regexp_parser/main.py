DEBUG = True


class Stack:
    def __init__(self):
        self.__items = []

    def push(self, item):
        self.__items.append(item)

    def pop(self):
        if len(self.__items) == 0:
            return None
        return self.__items.pop()

    def peek(self):
        n = len(self.__items)
        if n == 0:
            return None
        return self.__items[n - 1]

    def empty(self):
        return len(self.__items) == 0

    def items(self):
        return self.__items


def is_letter(a: str):
    return (a is not None) and (a.isalpha())


def is_un_op(a: str):
    return a == '*'


def is_bin_op(a: str):
    return (a == '.') or (a == '|')


def is_valid_symbol(a: str):
    return (a == '(') or (a == ')') or is_letter(a) or is_un_op(a) or is_bin_op(a)


def raise_err(s: str, idx: int):
    err = "Syntax error: "
    offset = len(err) + idx
    message = err + s
    if idx > 0:
        message += '\n' + (' ' * offset) + '^'
    print(message)
    exit(-1)


exp = input("Expression:")

ops = Stack()
vals = Stack()

prev = None
for i in range(len(exp)):
    cur = exp[i]
    if cur == '(':
        if is_letter(prev) or (prev == ')') or (prev == '*'):
            ops.push('.')
        ops.push(cur)
    elif is_letter(cur):
        if is_letter(prev) or is_un_op(prev) or prev == ')':
            assert (not vals.empty())
            vals.push((vals.pop(), '.', (None, cur, None)))
        elif is_bin_op(prev):
            assert (not ops.empty()) and (not vals.empty())
            vals.push((vals.pop(), ops.pop(), (None, cur, None)))
        else:
            vals.push((None, cur, None))
    elif is_un_op(cur):
        if is_letter(prev) or (prev == ')'):
            assert (not vals.empty())
            vals.push((vals.pop(), cur, None))
        else:
            raise_err(exp, i)
    elif is_bin_op(cur):
        ops.push(cur)
    elif cur == ')':
        if ops.pop() != '(':
            raise_err(exp, i)
    else:
        raise_err(exp, i)

    prev = cur

    if DEBUG:
        print("\n\n")
        print("INDEX: ", i + 1)
        print("VALS:", vals.items())
        print(" OPS:", ops.items())

while not ops.empty():
    right, op, left = vals.pop(), ops.pop(), vals.pop()
    if (op is None) or (not is_bin_op(op)):
        raise_err(exp, -1)
    vals.push((left, op, right))

print("\n\n")
print("TREE:", vals.items()[0])
