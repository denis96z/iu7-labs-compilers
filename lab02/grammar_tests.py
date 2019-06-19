import copy

import pytest
from dill.source import getname

from grammar import Grammar

TESTS_DIR = './tests'

GRAMMAR_1_FILE = f'{TESTS_DIR}/grammar_1'
GRAMMAR_2_FILE = f'{TESTS_DIR}/grammar_2'

RECURSIVE_GRAMMAR_1_FILE = f'{TESTS_DIR}/recursive_grammar_1'
NON_RECURSIVE_GRAMMAR_1_FILE = f'{TESTS_DIR}/non_recursive_grammar_1'
RECURSIVE_GRAMMAR_1_FIXED_FILE = f'{TESTS_DIR}/recursive_grammar_1_fixed'

RECURSIVE_GRAMMAR_2_FILE = f'{TESTS_DIR}/recursive_grammar_2'
NON_RECURSIVE_GRAMMAR_2_FILE = f'{TESTS_DIR}/non_recursive_grammar_2'
RECURSIVE_GRAMMAR_2_FIXED_FILE = f'{TESTS_DIR}/recursive_grammar_2_fixed'

EPS_GRAMMAR_1_FILE = f'{TESTS_DIR}/eps_grammar_1'
NON_EPS_GRAMMAR_1_FILE = f'{TESTS_DIR}/non_eps_grammar_1'
EPS_GRAMMAR_1_FIXED_FILE = f'{TESTS_DIR}/eps_grammar_1_fixed'


class TestGrammar:
    @pytest.mark.it('Check the same grammars equal')
    def test_grammar_eq(self) -> None:
        a = Grammar()
        b = Grammar()
        a.load_from_file(GRAMMAR_1_FILE)
        b.load_from_file(GRAMMAR_1_FILE)
        assert a == b

    @pytest.mark.it('Check different grammars differ')
    def test_grammar_not_eq(self) -> None:
        a = Grammar()
        b = Grammar()
        a.load_from_file(GRAMMAR_1_FILE)
        b.load_from_file(GRAMMAR_2_FILE)
        assert a != b

    @pytest.mark.it('Check left recursion is removed correctly')
    @pytest.mark.parametrize('src_file,dest_file,exp_file', [
        pytest.param(
            RECURSIVE_GRAMMAR_1_FILE,
            RECURSIVE_GRAMMAR_1_FIXED_FILE,
            NON_RECURSIVE_GRAMMAR_1_FILE,
            id='1',
        ),
        pytest.param(
            RECURSIVE_GRAMMAR_2_FILE,
            RECURSIVE_GRAMMAR_2_FIXED_FILE,
            NON_RECURSIVE_GRAMMAR_2_FILE,
            id='2',
        ),
    ])
    def test_rec_remove(self, src_file: str, dest_file: str,
                        exp_file: str) -> None:
        self._check_operation(getname(Grammar.remove_left_recursion), src_file,
                              dest_file, exp_file)

    @pytest.mark.it('Check ε-rules are removed correctly')
    @pytest.mark.parametrize('src_file,dest_file,exp_file', [
        pytest.param(
            EPS_GRAMMAR_1_FILE,
            EPS_GRAMMAR_1_FIXED_FILE,
            NON_EPS_GRAMMAR_1_FILE,
            id='1',
        ),
    ])
    def test_eps_remove(self, src_file: str, dest_file: str,
                        exp_file: str) -> None:
        self._check_operation(getname(Grammar.remove_left_recursion), src_file,
                              dest_file, exp_file)

    @staticmethod
    def _check_operation(method: str, src_file: str, dest_file: str,
                         exp_file: str) -> None:
        a = Grammar()
        a.load_from_file(src_file)

        b = copy.deepcopy(a)
        getattr(b, method)()

        b.save_to_file(dest_file)
        data = [set(open(i).read().split()) for i in (dest_file, exp_file)]
        assert data[0] == data[1]
