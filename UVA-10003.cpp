#include <algorithm>
#include <cmath>
#include <iostream>
#include <limits>
#include <map>
#include <vector>

int calculate_cost(int length, std::vector<int> const &cuts);

std::map<std::pair<int, std::vector<int>>, int> memoized_costs;

int _calculate_cost(int length, std::vector<int> const &cuts) {
    int best_cost = std::numeric_limits<int>::max();
    for (int i = 0; i < cuts.size(); ++i) {
        int cut = cuts[i];
        int cost = 0;
        if (i > 0) {
            std::vector<int> left_cuts;
            left_cuts.reserve(i);
            for (int j = 0; j < i; ++j) {
                left_cuts.push_back(cuts[j]);
            }
            cost += calculate_cost(cut, left_cuts);
        }
        if (i < cuts.size() - 1) {
            std::vector<int> right_cuts;
            right_cuts.reserve(cuts.size() - i);
            for (int j = i + 1; j < cuts.size(); ++j) {
                right_cuts.push_back(cuts[j] - cut);
            }
            cost += calculate_cost(length - cut, right_cuts);
        }
        if (cost < best_cost) {
            best_cost = cost;
        }
    }
    return length + best_cost;
}

int calculate_cost(int length, std::vector<int> const &cuts) {
    auto it = memoized_costs.find(std::make_pair(length, cuts));
    if (it != memoized_costs.end()) {
        return it->second;
    }
    auto cost = _calculate_cost(length, cuts);
    memoized_costs[std::make_pair(length, cuts)] = cost;
    return cost;
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
        cuts.reserve(number_of_cuts);
        for (int i = 0; i < number_of_cuts; ++i) {
            int cut = 0;
            std::cin >> cut;
            cuts.push_back(cut);
        }
        std::sort(cuts.begin(), cuts.end());
        int cost = calculate_cost(length, cuts);
        std::cout << "The minimum cutting is " << cost << '.' << std::endl;
    }
    return 0;
}
