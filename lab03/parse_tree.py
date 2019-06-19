from typing import Any


class ParseTree:
    def __init__(self, value: Any) -> None:
        self.value = value
        self.children = []

    def add_child(self, child) -> None:
        self.children.append(child)

    def __repr__(self) -> str:
        return f'{self.value} -> {[child.value for child in self.children]}'


def print_tree(tree: ParseTree, indent: int = 0) -> None:
    indent_str = '\t' * indent + '+ '
    if tree.children:
        print(indent_str + f'{tree.value}')
        for child in tree.children:
            print_tree(child, indent + 1)
    else:
        print(indent_str + f'({tree.value})')
