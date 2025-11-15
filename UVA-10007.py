# https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&page=show_problem&problem=948

import sys
from functools import lru_cache

def factorial(n):
    t = 1
    for i in range(2, n + 1):
        t *= i
    return t

@lru_cache(301)
def count_unlabeled_trees(n):
    if n == 0:
        return 1
    elif n <= 2:
        return n
    total = 0
    for i in range(n):
        total += count_unlabeled_trees(i) * count_unlabeled_trees(n - i - 1)
    return total

def count_trees(n):
    return factorial(n) * count_unlabeled_trees(n)

for line in sys.stdin:
    line = line.strip()
    if line:
        n = int(line)
        if n == 0:
            break
        print(count_trees(n))

