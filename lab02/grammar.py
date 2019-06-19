from __future__ import annotations

import itertools
from collections import namedtuple
from typing import List, Set

from consts import EPS
from rule import Rule
from utils import lists_eq


class Grammar:
    def __init__(self) -> None:
        self.non_terminals: List[str] = []
        self.terminals: List[str] = []
        self.rules: List[Rule] = []
        self.start = ''

    def __eq__(self, other: Grammar) -> bool:
        if self.start != other.start:
            return False
        return lists_eq(self.non_terminals, other.non_terminals) and \
            lists_eq(self.terminals, other.terminals) and \
            lists_eq(self.rules, other.rules)

    def add_non_terminal(self, non_terminal: str) -> None:
        self.non_terminals.append(non_terminal)

    def add_terminal(self, terminal: str) -> None:
        self.terminals.append(terminal)

    def add_rule(self, left_part: str, right_part: str) -> None:
        if not self.rule_part_correct(left_part) or \
                not self.rule_part_correct(right_part):
            raise ValueError(f'Invalid rule: {left_part} -> {right_part}')
        self.rules.append(Rule(left_part, right_part))

    def add_start(self, start: str) -> None:
        self.start = start

    def rule_part_correct(self, part: str) -> bool:
        for ch in part.split(' '):
            if (ch not in self.non_terminals) and \
                    (ch not in self.terminals) and \
                    (ch != EPS):
                return False
        return True

    def remove_left_recursion(self) -> None:
        RulesTuple = namedtuple('RulesTuple', ['recursive', 'non_recursive'])

        a = self.non_terminals
        for i in range(len(a)):
            for j in range(i):
                aij_rules = []
                for rule in self.rules:
                    if rule.left_part == a[i] and a[j] == rule.right_part[0]:
                        aij_rules.append(rule)
                for rule in aij_rules:
                    aj = rule.right_part[0]
                    aj_rules = []
                    for r in self.rules:
                        if r.left_part == aj:
                            aj_rules.append(r)
                    self.rules.remove(rule)
                    for r in aj_rules:
                        right_part = r.right_part.copy()
                        right_part.extend(rule.right_part[1:])
                        self.add_rule(rule.left_part, ' '.join(right_part))
            rules = RulesTuple([], [])
            for rule in self.rules:
                if rule.left_part == a[i]:
                    if rule.right_part[0] == a[i]:
                        rules.recursive.append(rule)
                    else:
                        rules.non_recursive.append(rule)

            if len(rules.recursive) > 0:
                new_non_literal = f'{a[i]}1'
                self.add_non_terminal(new_non_literal)
                for rule in rules.non_recursive:
                    self.rules.remove(rule)
                    if rule.right_part[0] != EPS:
                        self.add_rule(
                            a[i],
                            ' '.join(rule.right_part) + ' ' + new_non_literal)
                    else:
                        self.add_rule(a[i], new_non_literal)
                        self.add_rule(new_non_literal, EPS)
                for rule in rules.recursive:
                    self.rules.remove(rule)
                    self.add_rule(
                        new_non_literal,
                        ' '.join(rule.right_part[1:]) + ' ' + new_non_literal)

    def find_eps_rules(self) -> Set[str]:
        rules_without_terminals = [
            rule for rule in self.rules
            if not rule.has_terminals(self.non_terminals)
        ]
        is_eps = {non_term: False for non_term in self.non_terminals}
        concerned_rules = {non_term: [] for non_term in self.non_terminals}
        counter = {
            rule: rule.count_non_terminals(self.non_terminals)
            for rule in rules_without_terminals
        }

        for rule in rules_without_terminals:
            for non_term in rule.right_part:
                if non_term != EPS:
                    concerned_rules[non_term].append(rule)

        q = []
        for rule, count in counter.items():
            if count == 0:
                q.append(rule.left_part)
                is_eps[rule.left_part] = True
        while len(q) > 0:
            non_term = q.pop(0)
            for rule in concerned_rules[non_term]:
                count = counter[rule]
                counter[rule] = count - 1
                if counter[rule] == 0 and not is_eps[rule.left_part]:
                    is_eps[rule.left_part] = True
                    q.append(rule.left_part)

        return {non_term for non_term, f in is_eps.items() if f}

    def remove_eps_rules(self):
        g = Grammar()

        g.terminals = self.terminals.copy()
        g.non_terminals = self.non_terminals.copy()
        g.rules = self.rules.copy()
        g.start = self.start

        eps_non_terms = self.find_eps_rules()
        for rule in self.rules:
            rule_eps = eps_non_terms.intersection(rule.right_part)
            for i in range(len(rule_eps)):
                for combo in itertools.combinations(rule_eps, i + 1):
                    new_rule = Rule(rule.left_part, ' '.join(rule.right_part))
                    for ch in combo:
                        new_rule.right_part.remove(ch)
                    if len(new_rule.right_part) > 0:
                        g.rules.append(new_rule)

        eps_rules = []
        for rule in g.rules:
            if rule.right_part[0] == EPS:
                eps_rules.append(rule)
        for rule in eps_rules:
            g.rules.remove(rule)

        self.non_terminals = g.non_terminals
        self.terminals = g.terminals
        self.rules = g.rules
        self.start = g.start

    def load_from_file(self, filename: str) -> None:
        self.non_terminals.clear()
        self.terminals.clear()
        self.rules.clear()
        self.start = ''
        with open(filename) as file:
            non_terminals = file.readline().replace('\n', '').split(' ')
            for non_terminal in non_terminals:
                self.non_terminals.append(non_terminal)
            terminals = file.readline().replace('\n', '').split(' ')
            for terminal in terminals:
                self.terminals.append(terminal)
            rules_count = int(file.readline().replace('\n', ''))
            for _ in range(rules_count):
                rule_parts = file.readline().replace('\n', '').split('->')
                self.add_rule(left_part=rule_parts[0],
                              right_part=rule_parts[1])
            self.start = file.readline().replace('\n', '')

    def save_to_file(self, filename: str) -> None:
        with open(filename, 'w') as f:
            f.write(' '.join(self.non_terminals))
            f.write('\n')
            f.write(' '.join(self.terminals))
            f.write('\n')
            f.write(str(len(self.rules)))
            f.write('\n')
            for rule in self.rules:
                f.write(f'{rule.left_part}->{" ".join(rule.right_part)}')
                f.write('\n')
            f.write(self.start)
