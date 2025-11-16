// https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=12&page=show_problem&problem=949
//
// g++ -lm -lcrypt -O2 -std=c++11 -pipe -DONLINE_JUDGE UVA-10008.cpp

#include <algorithm>
#include <cctype>
#include <iostream>
#include <vector>

int main() {
    int char_count[256] = {};

    size_t number_of_lines = 0;
    std::cin >> number_of_lines;
    std::cin.ignore();

    for (size_t i = 0; i < number_of_lines; ++i) {
        std::string line;
        std::getline(std::cin, line);
        for (auto c = line.begin(); c != line.end(); ++c) {
            ++char_count[std::toupper(*c)];
        }
    }

    // Order by count descending; on a tie, order alphabetical (increasing).
    std::vector<std::pair<int, int>> result;
    result.reserve('Z' - 'A' + 1);
    for (int c = 'A'; c <= 'Z'; ++c) {
        result.push_back(std::make_pair(char_count[c], -c));
    }
    std::sort(result.rbegin(), result.rend());

    for (auto it = result.begin(); it != result.end(); ++it) {
        if (it->first > 0) {
            std::cout << char(-it->second) << ' ' << it->first << std::endl;
        }
    }

    return 0;
}
