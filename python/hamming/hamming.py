def distance(strand_a, strand_b):
    if len(strand_a) != len(strand_b):
        raise ValueError("Non matching stand sizes")

    return len([a for (a, b) in zip(strand_a, strand_b) if a != b])
