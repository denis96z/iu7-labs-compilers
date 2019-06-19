import sys
from typing import List

from parse_tree import print_tree
from parser import Parser


def main(args: List[str]) -> None:
    s = args[0]
    print(s)
    p = Parser(s)
    if p.check_string():
        print_tree(p.get_tree())


if __name__ == '__main__':
    main(sys.argv[1:])
