import unittest

from fa_builder import build_for_regexp
from fa_state import State
from fa import Automata
from fa_minimization import minimization
import syntax_tree as st


class MyTestCase(unittest.TestCase):
    def test_tree_build(self):
        regex = '(a(b|a))+b'
        tree = st.build_tree(regex)
        st.visualize_tree(tree, regex)
        self.assertEqual(True, True)

    def test_minimize(self):
        origin_dfa = build_minimize_dfa()
        origin_dfa.visualize('origin')
        min_dfa = minimization(origin_dfa)
        min_dfa.visualize('min')
        self.assertEqual(True, True)

    def test_dfa_builder(self):
        regexp = '(a|b)*abb'
        fa = build_for_regexp(regexp)
        fa.visualize('test_builder')
        self.assertEqual(True, True)


if __name__ == '__main__':
    unittest.main()


def build_dfa():
    states = list()
    for i in range(5):
        states.append(State(positions={i+1}))
    states[0].move_on_char('a', states[2])
    states[0].move_on_char('b', states[1])
    states[1].move_on_char('b', states[0])
    states[1].move_on_char('a', states[3])
    states[2].move_on_char('b', states[3])
    states[2].move_on_char('a', states[4])
    states[3].move_on_char('a', states[3])
    states[3].move_on_char('b', states[3])
    states[4].move_on_char('a', states[2])
    states[4].move_on_char('b', states[1])
    states[0].is_final = True
    states[4].is_final = True
    dfa = Automata(q0=states[0])
    return dfa


def build_minimize_dfa():
    states = list()
    for i in range(7):
        states.append(State(positions={i+1}))
    states[0].move_on_char('a', states[6])
    states[0].move_on_char('b', states[1])
    states[1].move_on_char('a', states[6])
    states[1].move_on_char('b', states[0])
    states[2].move_on_char('a', states[3])
    states[2].move_on_char('b', states[4])
    states[3].move_on_char('a', states[4])
    states[3].move_on_char('b', states[5])
    states[4].move_on_char('a', states[4])
    states[4].move_on_char('b', states[4])
    states[5].move_on_char('a', states[5])
    states[5].move_on_char('b', states[4])
    states[6].move_on_char('a', states[2])
    states[6].move_on_char('b', states[2])
    states[4].is_final = True
    states[5].is_final = True
    return Automata(q0=states[0])
