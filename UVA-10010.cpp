// https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=12&page=show_problem&problem=951
//
// g++ -lm -lcrypt -O2 -std=c++11 -pipe -DONLINE_JUDGE UVA-10009.cpp

#include <algorithm>
#include <iostream>
#include <string>
#include <vector>

bool needle_in_haystack_at(std::vector<std::string> const &haystack, std::string const &needle, int row, int col, int dr, int dc) {
    for (int i = 0; i < needle.size(); ++i) {
        if (row < 0 || row >= haystack.size()) {
            return false;
        }
        if (col < 0 || col >= haystack[0].size()) {
            return false;
        }
        if (haystack[row][col] != needle[i]) {
            return false;
        }
        row += dr;
        col += dc;
    }
    return true;
}

bool needle_in_haystack_at(std::vector<std::string> const &haystack, std::string const &needle, int row, int col) {
    return (
        needle_in_haystack_at(haystack, needle, row, col, 0, 1) ||
        needle_in_haystack_at(haystack, needle, row, col, 0, -1) ||
        needle_in_haystack_at(haystack, needle, row, col, 1, 0) ||
        needle_in_haystack_at(haystack, needle, row, col, -1, 0) ||
        needle_in_haystack_at(haystack, needle, row, col, -1, -1) ||
        needle_in_haystack_at(haystack, needle, row, col, -1, 1) ||
        needle_in_haystack_at(haystack, needle, row, col, 1, 1) ||
        needle_in_haystack_at(haystack, needle, row, col, 1, -1)
    );
}

std::pair<int, int> find_needle_in_haystack(std::vector<std::string> const &haystack, std::string const &needle) {
    for (int row = 0; row < haystack.size(); ++row) {
        auto &line = haystack[row];
        for (int col = 0; col < line.size(); ++col) {
            if (line[col] == needle[0]) {
                if (needle_in_haystack_at(haystack, needle, row, col)) {
                    return std::make_pair(row + 1, col + 1);
                }
            }
        }
    }
    return std::make_pair(0, 0);
}

int main() {
    size_t number_of_test_cases = 0;

    if (std::cin >> number_of_test_cases) {
        for (size_t test_case = 0; test_case < number_of_test_cases; ++test_case) {
            size_t number_of_rows = 0;
            size_t number_of_columns = 0;
            std::vector<std::string> haystack;

            if (std::cin >> number_of_rows >> number_of_columns) {
                for (size_t row_number = 0; row_number < number_of_rows; ++row_number) {
                    std::string line;
                    std::cin >> line;
                    for (auto it = line.begin(); it != line.end(); ++it) {
                        *it = toupper(*it);
                    }
                    haystack.push_back(line);
                }

                if (test_case > 0) {
                    std::cout << std::endl;
                }

                size_t number_of_needles = 0;

                if (std::cin >> number_of_needles) {
                    for (size_t needle_number = 0; needle_number < number_of_needles; ++needle_number) {
                        std::string needle;
                        if (std::cin >> needle) {
                            for (auto it = needle.begin(); it != needle.end(); ++it) {
                                *it = toupper(*it);
                            }
                            auto location = find_needle_in_haystack(haystack, needle);
                            std::cout << location.first << ' ' << location.second << std::endl;
                        }
                    }
                }
            }
        }
    }

    return 0;
}
