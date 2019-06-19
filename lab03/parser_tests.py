import pytest

from parser import Parser


class TestParser:
    @pytest.mark.it('Parse string')
    @pytest.mark.parametrize('s,is_valid', [
        pytest.param(
            '~A ! ~B & ~C',
            True,
            id='1',
        ),
        pytest.param(
            '~B',
            True,
            id='2',
        ),
        pytest.param(
            '~A | C',
            False,
            id='3',
        ),
        pytest.param(
            'A true',
            False,
            id='4',
        ),
        pytest.param(
            'true & ~true',
            True,
            id='5',
        ),
        pytest.param(
            '~A ! ~B & ~C',
            True,
            id='6',
        ),
        pytest.param(
            'true',
            True,
            id='7',
        ),
    ])
    def test_check_string(self, s: str, is_valid: bool) -> None:
        assert Parser(s).check_string() == is_valid
