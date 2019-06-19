from collections import namedtuple

from tokens import MARKER

ParseResult = namedtuple('ParseResult', ['is_valid', 'value'])


def parse(tokens: list, all_tokens: list, relations: dict) -> ParseResult:
    tokens = enumerate(tokens + [MARKER])
    result = []
    next_token_no, next_token = next(tokens)
    stack_tail, stack_head = [], MARKER
    while True:
        if next_token in all_tokens:
            if stack_head == MARKER and next_token == MARKER:
                break
            relation = relations[stack_head][next_token]
            if relation in ('<', '='):
                stack_tail.append(stack_head)
                stack_head = next_token
                next_token_no, next_token = next(tokens)
                continue
            if relation == '>':
                while True:
                    if stack_head not in ('(', ')'):
                        result.append(stack_head)
                    old_stack_head = stack_head
                    stack_head = stack_tail.pop()
                    if relations[stack_head][old_stack_head] == '<':
                        break
                continue
        return ParseResult(False, f'Error in {next_token_no} token')
    return ParseResult(True, ' '.join(result))
