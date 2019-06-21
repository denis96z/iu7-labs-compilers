import syntax_tree as st
from fa_state import State
from fa import Automata


def build_for_regexp(regexp):
    q = {
        'marked': list(),
        'unmarked': list(),
    }
    char_position = dict()
    for char in regexp:
        char_position.setdefault(char, list())
    tree = st.build_tree(regexp)
    st.visualize_tree(tree, regexp)
    fp = st.get_followpos(tree)
    q0 = State(positions=st.firstpos(tree))
    q['unmarked'].append(q0)
    for key in char_position.keys():
        char_position[key] = st.get_char_positions(tree, key)
    while len(q['unmarked']) != 0:
        r = q['unmarked'].pop(0)
        q['marked'].append(r)
        for char in char_position.keys():
            p: set = r.positions.intersection(set(char_position[char]))
            if len(p) != 0:
                s_set = set()
                for pi in p:
                    s_set.update(fp[pi])
                if len(s_set) != 0:
                    s = State(positions=s_set)
                    q_in, s = state_in_q(q, s)
                    if not q_in:
                        q['unmarked'].append(s)
                    r.move_on_char(character=char, dest=s)
    end_key_position = max(fp.keys())
    for state in q['marked']:
        if end_key_position in state.positions:
            state.isFinalState = True
    return Automata(q0)


def state_in_q(q, state):
    for s in q['marked']:
        if s.positions == state.positions:
            del state
            return True, s
    for s in q['unmarked']:
        if s.positions == state.positions:
            del state
            return True, s
    return False, state
