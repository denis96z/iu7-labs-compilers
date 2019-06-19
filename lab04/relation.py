def make_relations(tokens: set, variables: set, constants: set, prefix: str,
                   precedence: dict) -> dict:
    right_associative = {'**'}

    relations = {t: {t: None for t in tokens} for t in tokens}

    relations['('][')'] = '='

    relations['$']['('] = relations['(']['('] = '<'
    relations[')']['$'] = relations[')'][')'] = '>'

    for thing in variables | constants:
        relations['$'][thing] = relations['('][thing] = '<'
        relations[thing]['$'] = relations[thing][')'] = '>'

    for op in precedence:
        relations[op]['$'] = '>'
        relations['$'][op] = '<'

        relations[op]['('] = relations['('][op] = '<'
        relations[op][')'] = relations[')'][op] = '>'

        for thing in variables | constants:
            relations[op][thing] = '<'
            relations[thing][op] = '>'

        if op in prefix:
            for op2 in precedence:
                relations[op2][op] = '<'
                if precedence[op] > precedence[op2]:
                    relations[op][op2] = '>'
                else:
                    relations[op][op2] = '<'
        else:
            for op2 in precedence:
                if precedence[op] < precedence[op2] or precedence[op] == precedence[
                        op2] and op in right_associative and op2 in right_associative:
                    relations[op][op2] = '<'
                    continue
                if precedence[op] > precedence[op2] or precedence[op] == precedence[
                        op2] and op not in right_associative and op2 not in right_associative:
                    relations[op][op2] = '>'
                    continue

    return relations
