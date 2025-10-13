import sys

from functools import lru_cache

@lru_cache(None)
def calculate_minimum_cost(stick, cuts):
    length = stick
    best_cost = float('inf')
    for i, cut in enumerate(cuts):
        cost = 0
        if i > 0:
            cost += calculate_minimum_cost(cut, cuts[:i])
        if i < len(cuts) - 1:
            cost += calculate_minimum_cost(stick - cut, tuple(c - cut for c in cuts[i+1:]))
        if cost < best_cost:
            best_cost = cost
    return length + best_cost

while length := sys.stdin.readline():
    length = int(length.strip())
    if length <= 0:
        break
    number_of_cuts = sys.stdin.readline()
    number_of_cuts = int(number_of_cuts.strip())
    cuts = sys.stdin.readline()
    cuts = [int(cut) for cut in cuts.strip().split() if cut]
    cuts.sort()
    cuts = tuple(cuts)
    minimum_cost = calculate_minimum_cost(length, cuts)
    print(f"The minimum cutting is {minimum_cost}.")
