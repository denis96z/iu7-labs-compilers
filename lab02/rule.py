from __future__ import annotations

from typing import List

from consts import EPS


class Rule:
    def __init__(self, left_part: str, right_part: str) -> None:
        self.left_part = left_part
        self.right_part = right_part.split(' ')

    def __eq__(self, other: Rule) -> bool:
        return self.left_part == other.left_part and \
               self.right_part == other.right_part

    def __hash__(self) -> int:
        return hash(self.left_part + '->' + ' '.join(self.right_part))

    def count_non_terminals(self, non_terminals: List[str]) -> int:
        count = 0
        for ch in self.right_part:
            if ch in non_terminals:
                count = count + 1
        return count

    def has_terminals(self, non_terminals: List[str]) -> bool:
        for ch in self.right_part:
            if ch not in non_terminals and ch != EPS:
                return True
        return False
