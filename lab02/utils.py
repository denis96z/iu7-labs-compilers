def lists_eq(a, b):
    return len(a) == len(set(a).intersection(b))
