from typing import Any

from parse_tree import ParseTree


class Parser:
    def __init__(self, s: str) -> None:
        self.string = s.replace(' ', '')
        self.identifiers = list(chr(i) for i in range(65, 91))
        self.index = 0
        self.tree = None

    def get_tree(self) -> Any:
        return self.tree

    def check_string(self) -> bool:
        tree = self._expression()
        if tree is not None and self._string_is_end():
            self.tree = tree
            return True
        return False

    def _expression(self):
        tree = ParseTree('Expr')
        node = self._log_expr()
        if node is not None:
            tree.add_child(node)
            return tree
        return None

    def _log_expr(self):
        tree = ParseTree('log expr')
        log_node = self._log_mon()
        if log_node is not None:
            tree.add_child(log_node)
        log_expr_node = self._log_expr_()
        if log_expr_node is not None:
            tree.add_child(log_expr_node)
        if tree.children:
            return tree
        return None

    def _log_expr_(self):
        if self._out_of_range():
            return None
        tree = ParseTree('log expr')
        if self.string[self.index] == '!':
            self.index += 1
            tree.add_child(ParseTree('!'))
            log_mon_node = self._log_mon()
            if log_mon_node:
                tree.add_child(log_mon_node)
                log_expr_node = self._log_expr_()
                if log_expr_node:
                    tree.add_child(log_expr_node)
                return tree
        return None

    def _log_mon(self) -> Any:
        tree = ParseTree('log mon')
        sec_expr_node = self._sec_expr()
        if sec_expr_node:
            tree.add_child(sec_expr_node)
        log_mon_node = self._log_mon_()
        if log_mon_node is not None:
            tree.add_child(log_mon_node)
        if tree.children:
            return tree
        return None

    def _log_mon_(self) -> Any:
        if self._out_of_range():
            return None
        tree = ParseTree('log mon')
        if self.string[self.index] == '&':
            self.index += 1
            tree.add_child(ParseTree('&'))
            sec_expr_node = self._sec_expr()
            if sec_expr_node:
                tree.add_child(sec_expr_node)
                log_mon_node = self._log_mon_()
                if log_mon_node:
                    tree.add_child(log_mon_node)
                return tree
        return None

    def _sec_expr(self):
        tree = ParseTree('sec_expr')
        first_exp_node = self._first_expr()
        if first_exp_node:
            tree.add_child(first_exp_node)
            return tree
        if self._out_of_range():
            return None
        if self.string[self.index] == '~':
            self.index += 1
            tree.add_child(ParseTree('~'))
            first_exp_node = self._first_expr()
            if first_exp_node:
                tree.add_child(first_exp_node)
                return tree
        return None

    def _first_expr(self):
        tree = ParseTree('first_expr')
        log_value_node = self._log_value()
        if log_value_node:
            tree.add_child(log_value_node)
            return tree
        identifier_node = self._identifier()
        if identifier_node:
            tree.add_child(identifier_node)
            return tree
        return None

    def _log_value(self) -> Any:
        tree = ParseTree('log value')
        if self.string[self.index:self.index + 4] == 'true':
            self.index += 4
            tree.add_child(ParseTree('true'))
            return tree
        if self.string[self.index:self.index + 5] == 'false':
            self.index += 5
            tree.add_child(ParseTree('false'))
            return tree
        return None

    def _identifier(self) -> Any:
        tree = ParseTree('identifier')
        count = 0
        while True:
            if self._out_of_range():
                break
            if self.string[self.index] in self.identifiers:
                self.index += 1
                count += 1
            else:
                break
        if count:
            tree.add_child(
                ParseTree(self.string[self.index - count:self.index]))
            return tree
        return None

    def _out_of_range(self) -> bool:
        return self.index > (len(self.string) - 1)

    def _string_is_end(self):
        return self.index == len(self.string)

    def __repr__(self):
        return f'{self.index}'
