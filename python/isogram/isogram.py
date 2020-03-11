def is_isogram(string):
    string = list(filter(lambda c: c not in [' ', '-'], sorted(string.lower())))
    return all(map(lambda x: x[0] != x[1], zip(string[:-1], string[1:])))
