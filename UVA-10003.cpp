// https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&page=show_problem&problem=944
//
// g++ -lm -lcrypt -O2 -std=c++11 -pipe -DONLINE_JUDGE UVA-10003.cpp

#include <algorithm>
#include <cmath>
#include <iostream>
#include <limits>
#include <vector>
#include <set>

int calculate_cost(int length, std::vector<int> const &cuts) {
    int dp[cuts.size()][cuts.size()] = {};
    for (int i = 0; i < cuts.size(); ++i) {
        for (int j = 0; j < cuts.size(); ++j) {
            dp[i][j] = 0;
        }
    }
    for (int sub_len = 2; sub_len < cuts.size(); ++sub_len) {
        for (int i = 0; i < cuts.size() - sub_len; ++i) {
            int j = i + sub_len;
            int best = std::numeric_limits<int>::max();
            for (int k = i + 1; k < j; ++k) {
                int cost = dp[i][k] + dp[k][j] + cuts[j] - cuts[i];
                if (cost < best) {
                    best = cost;
                }
            }
            dp[i][j] = best;
        }
    }
    return dp[0][cuts.size() - 1];
}

int main() {
    int length = 0;
    while (std::cin >> length) {
        if (length <= 0) {
            break;
        }
        int number_of_cuts = 0;
        std::cin >> number_of_cuts;
        std::vector<int> cuts;
        cuts.reserve(number_of_cuts + 2);
        cuts.push_back(0);
        for (int i = 0; i < number_of_cuts; ++i) {
            int cut = 0;
            std::cin >> cut;
            cuts.push_back(cut);
        }
        cuts.push_back(length);
        int cost = calculate_cost(length, cuts);
        std::cout << "The minimum cutting is " << cost << '.' << std::endl;
    }
    return 0;
}
