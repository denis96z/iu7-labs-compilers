class State(object):
    def __init__(self, positions, is_final=False):
        self.char_range_min = 97
        self.char_range_max = 122

        self.positions = positions
        self.is_final = is_final
        self.char_transitions = []
        for _ in range(self.char_range_max + 1):
            self.char_transitions.append(None)
        self.eps_transitions = list()

    def __str__(self):
        return str(self.positions)

    def is_supported_char(self, character):
        if len(character) > 1:
            raise ValueError("Expected a empty or one character string")
        if len(character) == 0:
            return True
        return self.char_range_min <= ord(character) <= self.char_range_max

    def move_on_char(self, character, dest):
        if not (isinstance(character, str)
                and isinstance(dest, State)):
            raise ValueError("Invalid parameters passed")
        if not self.is_supported_char(character):
            raise ValueError("Character symbol is invalid")
        char_ascii_value = ord(character)
        self.char_transitions[char_ascii_value] = dest

    def move_on_eps(self, dest):
        if not isinstance(dest, State):
            raise ValueError("Invalid parameters passed")
        self.eps_transitions.append(dest)

    def matches(self, string):
        return self.simulate(string)

    def simulate(self, string):
        eps_moves = self.get_eps_reachability()
        for ch in string:
            char_moves = set()
            for state in eps_moves:
                if state.is_final:
                    continue
                for move in state.get_transitions_for_char(ch):
                    char_moves.add(move)
            eps_moves = self.get_eps_reachability_for_states(char_moves)
        for state in eps_moves:
            if state.is_final:
                return True

        return False

    def get_transitions_for_char(self, ch):
        if not self.is_supported_char(ch):
            return []
        return self.char_transitions[ord(ch)]

    def get_eps_reachability(self, visited=None):
        stack = [self, ]
        if visited is None:
            visited = set()
        while stack:
            curr = stack.pop()
            if curr not in visited:
                visited.add(curr)
                stack.extend([state for state in curr.eps_transitions if state not in visited])

        return visited

    @staticmethod
    def get_eps_reachability_for_states(sources, visited=None):
        if visited is None:
            visited = set()
        for state in sources:
            if state not in visited:
                state.get_eps_reachability(visited)
        return visited

    def simulate_backtrace(self, string, processed_states):
        if self in processed_states:
            return False
        processed_states.add(self)
        if string:
            c0 = string[0]
            if self.is_supported_char(c0):
                for next_state in self.char_transitions[ord(c0)]:
                    if next_state.matches(string[1:]):
                        return True
            for eps_state in self.eps_transitions:
                if eps_state.simulate(string, processed_states):
                    return True
        else:
            if self.is_final:
                return True
            for eps_state in self.eps_transitions:
                if eps_state.simulate('', processed_states): return True
        return False
