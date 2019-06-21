from fa_builder import build_for_regexp
from fa_minimization import minimization


def main():
    regexp = str(input('Regexp: '))
    fa = build_for_regexp(regexp)
    states = fa.states()
    for state in states:
        print(state)
    fa.visualize('original_fa')
    min_fa = minimization(fa)
    min_fa.visualize('min_fa')
    check1 = str(input('Test expression: '))
    min_fa.check(check1)


if __name__ == '__main__':
    main()
