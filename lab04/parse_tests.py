from typing import Any

import pytest

from relation import make_relations
from parse import parse
from tokens import *


@pytest.mark.it('Try parse')
@pytest.mark.parametrize('in_str,exp_str', [
    pytest.param(
        'a < b',
        'a b <',
        id='1',
    ),
    pytest.param(
        'a b c',
        None,
        id='2',
    ),
    pytest.param(
        '( ( ( a / b ) ) )',
        'a b /',
        id='3',
    ),
    pytest.param(
        '( a mod b ) & c',
        'a b mod c &',
        id='4',
    ),
    pytest.param(
        'a mod b mod c +',
        None,
        id='5',
    ),
    pytest.param(
        '( a b * c )',
        None,
        id='6',
    ),
    pytest.param(
        '()',
        None,
        id='7',
    ),
])
def test_parse(in_str: str, exp_str: Any) -> None:
    relations = make_relations(ALL_TOKENS, VARIABLES, CONSTANTS, PREFIX,
                               PRECEDENCE)
    tokens = in_str.split()
    result = parse(tokens, ALL_TOKENS, relations)
    if exp_str is None:
        assert not result.is_valid
    else:
        assert result.value == exp_str
