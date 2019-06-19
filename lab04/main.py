import sys
from typing import List

from relation import make_relations
from parse import parse
from tokens import *


def main(args: List[str]) -> None:
    relations = make_relations(ALL_TOKENS, VARIABLES, CONSTANTS, PREFIX,
                               PRECEDENCE)
    tokens = args[0].strip().split()
    result = parse(tokens, ALL_TOKENS, relations)
    if result.is_valid:
        print(result.value)
    else:
        print('invalid')


if __name__ == '__main__':
    main(sys.argv[1:])
