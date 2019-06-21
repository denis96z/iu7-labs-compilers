from fa_state import State
from graphviz import Digraph
import numpy as np


class Automata(object):
    def __init__(self, q0):
        if not isinstance(q0, State):
            raise ValueError("Invalid parameters passed")
        self.initial_state = q0

    def matches(self, s):
        return self.initial_state and self.initial_state.matches(s)

    def visualize(self, filename):
        fa_graph = Digraph('finite_state_machine')
        states = self.states()
        fa_graph.attr('node', shape='doublecircle')
        for state in states:
            if state.is_final:
                fa_graph.node(str(state))
        fa_graph.attr('node', shape='circle')
        fa_graph.edge('START', str(self.initial_state))
        for state in states:
            for code, char_transition in enumerate(state.char_transitions):
                if char_transition is not None:
                    fa_graph.edge(str(state), str(char_transition), label=chr(code))
        fa_graph.view(filename=filename)

    def states(self):
        visited = list()
        state_stack = list()
        state_stack.append(self.initial_state)
        while len(state_stack) != 0:
            state = state_stack.pop(len(state_stack) - 1)
            if state not in visited:
                visited.append(state)
                for destinations in state.char_transitions:
                    if destinations is not None and destinations not in visited:
                        state_stack.append(destinations)
        visited.sort(key=State.__str__)
        return visited

    def alphabet(self):
        alph = ''
        states = self.states()
        for state in states:
            for code, dest in enumerate(state.char_transitions):
                if code >= state.char_range_min and dest is not None:
                    if chr(code) not in alph:
                        alph = alph + chr(code)
        return alph

    def reverse_edges_table(self):
        states = self.states()
        a = self.alphabet()
        states.insert(0, State({0}))
        for i in range(len(states)):
            state = states[i]
            for char in a:
                if state.char_transitions[ord(char)] is None:
                    state.move_on_char(char, states[0])
        size = len(states)
        sigma = np.chararray((size, size), itemsize=len(a) + 1)
        sigma[:] = b'0'
        for num, state in enumerate(states):
            for code, dest in enumerate(state.char_transitions):
                if dest is not None:
                    dest = dest
                    dest_index = states.index(dest)
                    new = sigma[dest_index][num].decode('utf-8') + chr(code)
                    sigma[dest_index][num] = new
        return sigma

    def check(self, check_string):
        print(f'Tested string: {check_string}')
        current_state = self.initial_state
        for c in check_string:
            next_state = current_state.get_transitions_for_char(c)
            if next_state is not None:
                print(f'Current state: {str(current_state)}. "{c}" -> {str(next_state)}')
                current_state = next_state
            else:
                print(f'Current state: {str(current_state)}. Not allowed')
                break
        if current_state.is_final:
            print(f'Final state: {str(current_state)}. OK')
        else:
            print(f'Non-final state: {str(current_state)}. ERROR')
