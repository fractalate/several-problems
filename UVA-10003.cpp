// https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&page=show_problem&problem=944
//
// This problem asks us to find the minimum cost to cut a stick at specified locations.
// We do this by considering the stick and the cuts along it as dividing up the stick.
// Then we think about the minimum cost to cut each pair of adjacent segments, then each
// triplets, then quadruplets, and so on until we have found the minimum cost to cut all
// segments.
//
// g++ -lm -lcrypt -O2 -std=c++11 -pipe -DONLINE_JUDGE UVA-10003.cpp

#include <cstring>
#include <iostream>
#include <limits>
#include <vector>

int calculate_minimum_cost(std::vector<int> const &cuts) {
    // dp[i][j] will be the minimum total cost to cut from i to j.
    int dp[cuts.size()][cuts.size()];

    // dp[i][i+1] = 0 initially since there is no cut between i and i + 1.
    for (size_t i = 1; i < cuts.size(); ++i) {
        dp[i-1][i] = 0;
    }

    // Consider groups of segments of a increasing length.
    for (size_t segments = 2; segments < cuts.size(); ++segments) {
        // Consider subsequent groups of segments on the stick.
        for (size_t i = 0, j = segments; j < cuts.size(); ++i, ++j) {
            // Consider cutting the group at each k between i and j.
            int minimum_cost = std::numeric_limits<int>::max();
            for (size_t k = i + 1; k < j; ++k) {
                minimum_cost = std::min(minimum_cost, dp[i][k] + dp[k][j]);
            }
            // So we know the minimum total cost to cut from i to j.
            int group_cost = cuts[j] - cuts[i];
            dp[i][j] = group_cost + minimum_cost;
        }
    }

    return dp[0][cuts.size() - 1];
}

bool read_problem(std::vector<int>& cuts) {
    cuts.clear();

    size_t length = 0;
    if (!(std::cin >> length)) {
        return false;
    } else if (length == 0) {
        return false;
    }

    size_t number_of_cuts = 0;
    if (!(std::cin >> number_of_cuts)) {
        return false;
    }

    cuts.reserve(number_of_cuts + 2);
    cuts.push_back(0);
    for (size_t i = 0; i < number_of_cuts; ++i) {
        int cut = 0;
        if (!(std::cin >> cut)) {
            return false;
        }
        cuts.push_back(cut);
    }
    cuts.push_back(length);

    return true;
}

int main() {
    std::vector<int> cuts;

    while (read_problem(cuts)) {
        int minimum_cost = calculate_minimum_cost(cuts);
        std::cout << "The minimum cutting is " << minimum_cost << '.' << std::endl;
    }

    return 0;
}
