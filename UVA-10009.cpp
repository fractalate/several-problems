// https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=12&page=show_problem&problem=950
//
// g++ -lm -lcrypt -O2 -std=c++11 -pipe -DONLINE_JUDGE UVA-10009.cpp

#include <algorithm>
#include <iostream>
#include <string>
#include <map>

std::string find_path(std::map<char, char> const &roads, char starting_city, char ending_city) {
    std::string starting_path;
    std::string ending_path;

    char city = starting_city;
    starting_path += city;
    while (city != 'R') {
        auto it = roads.find(city); // should crash if city is not found
        city = it->second;
        starting_path += city;
    }

    city = ending_city;
    ending_path += city;
    while (city != 'R') {
        auto it = roads.find(city); // should crash if city is not found
        city = it->second;
        ending_path += city;
    }

    char common_base = 0;
    while (true) {
        if (starting_path.empty() || ending_path.empty()) {
            break;
        }
        if (starting_path[starting_path.size() - 1] != ending_path[ending_path.size() - 1]) {
            break;
        }
        common_base = starting_path[starting_path.size() - 1];
        starting_path.resize(starting_path.size() - 1);
        ending_path.resize(ending_path.size() - 1);
    }

    std::reverse(ending_path.begin(), ending_path.end());

    return starting_path + common_base + ending_path;
}

int main() {
    size_t number_of_test_cases = 0;

    if (std::cin >> number_of_test_cases) {
        for (size_t test_case = 0; test_case < number_of_test_cases; ++test_case) {
            size_t number_of_roads = 0;
            size_t number_of_queries = 0;
            std::map<char, char> roads; // key is a city, value is the lower level city it connects to

            if (std::cin >> number_of_roads >> number_of_queries) {
                for (size_t road_number = 0; road_number < number_of_roads; ++road_number) {
                    std::string lower_city;
                    std::string higher_city;

                    if ((std::cin >> lower_city >> higher_city) && !lower_city.empty() && !higher_city.empty()) {
                        roads[higher_city[0]] = lower_city[0];
                    }
                }

                if (test_case > 0) {
                    std::cout << std::endl;
                }

                for (size_t query_number = 0; query_number < number_of_queries; ++query_number) {
                    std::string starting_city;
                    std::string ending_city;

                    if ((std::cin >> starting_city >> ending_city) && !starting_city.empty() && !ending_city.empty()) {
                        auto path = find_path(roads, starting_city[0], ending_city[0]);
                        std::cout << path << std::endl;
                    }
                }
            }
        }
    }

    return 0;
}
