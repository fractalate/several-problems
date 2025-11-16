# https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&page=show_problem&problem=948

import sys
from functools import lru_cache

PROBLEM_LIMIT = 300 + 1

@lru_cache(PROBLEM_LIMIT)
def factorial(n):
    if n <= 1:
        return 1
    elif n == 2:
        return 2
    return n * factorial(n - 1)

# This calculates the n-th Catalan number in O(n^2) multiplications.
@lru_cache(PROBLEM_LIMIT)
def count_unlabeled_trees(n):
    if n == 0:
        return 1
    elif n <= 2:
        return n
    total = 0
    for i in range(n):
        total += count_unlabeled_trees(i) * count_unlabeled_trees(n - i - 1)
    return total

# This calculates the numerator of the n-th Catalan number, the product (n+2)*...*(2*n).
# Then the Catalan number is (n+2)*...*(2*n) / n!
@lru_cache(PROBLEM_LIMIT)
def catalan_numerator(n):
    if n == 1:
        return 1
    result = n + 2
    for i in range(n + 3, 2 * n + 1):
        result *= i
    return result

# This calculates the n-th Catalan number in O(n) multiplications.
@lru_cache(PROBLEM_LIMIT)
def catalan(n):
    return catalan_numerator(n) // factorial(n)

def count_trees(n):
    # return factorial(n) * count_unlabeled_trees(n)
    # return factorial(n) * catalan(n)
    return catalan_numerator(n)

for line in sys.stdin:
    line = line.strip()
    if line:
        n = int(line)
        if n == 0:
            break
        print(count_trees(n))

