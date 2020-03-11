from collections import Counter
import re

def count_words(sentence):
    return dict(Counter(re.findall("(?:[0-9]+|[a-z]+(?:'[a-z])?)", sentence.lower())))

