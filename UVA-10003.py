# https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&page=show_problem&problem=944

import sys

def calculate_minimum_cost(length, cuts):
    cuts = [0] + cuts + [length]
    dp = [
        [0] * len(cuts)
            for _ in range(len(cuts))
    ]
    for sub_len in range(2, len(cuts)):
        for i in range(0, len(cuts) - sub_len):
            j = i + sub_len
            best = float('inf')
            for k in range(i + 1, j):
                cost = dp[i][k] + dp[k][j] + (cuts[j] - cuts[i])
                if cost < best:
                    best = cost
            dp[i][j] = best
    return dp[0][len(cuts) - 1]

while length := sys.stdin.readline():
    length = int(length.strip())
    if length <= 0:
        break
    number_of_cuts = sys.stdin.readline()
    number_of_cuts = int(number_of_cuts.strip())
    cuts = sys.stdin.readline()
    cuts = [int(cut) for cut in cuts.strip().split() if cut]
    cuts.sort()
    minimum_cost = calculate_minimum_cost(length, cuts)
    print(f"The minimum cutting is {minimum_cost}.")
