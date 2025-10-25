// https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=12&page=show_problem&problem=945
//
// g++ -lm -lcrypt -O2 -std=c++11 -pipe -DONLINE_JUDGE UVA-10004.cpp

#include <algorithm>
#include <deque>
#include <iostream>
#include <cstring>
#include <set>
#include <vector>

struct Node {
    std::set<size_t> neighbors;
    bool color;
};

bool you_bi(std::vector<Node> &nodes, std::set<std::pair<size_t, size_t>> &edges) {
    std::set<size_t> seen;
    std::deque<std::pair<bool, size_t>> queue;

    seen.insert(0);
    queue.push_back(std::make_pair(false, 0));

    while (!queue.empty()) {
        auto &item = queue.front();
        bool color = !item.first;
        size_t i = item.second;
        queue.pop_front();

        nodes[i].color = color;

        for (size_t j : nodes[i].neighbors) {
            if (seen.find(j) == seen.end()) { // if not seen
                queue.push_back(std::make_pair(color, j));
                seen.insert(j);
            }

            size_t a = i;
            size_t b = j;

            /*if (b < a) { std::swap(a, b); }
            auto where = edges.find(std::make_pair(a, b));
            if (where != edges.end()) {
                edges.erase(where);
            }*/
        }
    }

    for (auto &edge : edges) {
        if (nodes[edge.first].color == nodes[edge.second].color) {
            return false;
        }
    }

    return true;
}

int main() {
    size_t v = 0;

    while (std::cin >> v) {
        if (v == 0) {
            break;
        }

        std::vector<Node> nodes(v);

        size_t e = 0;
        if (std::cin >> e) {
            std::set<std::pair<size_t, size_t>> edges;

            for (int i = 0; i < e; ++i) {
                size_t a = 0, b = 0;
                std::cin >> a >> b;

                if (b < a) { std::swap(a, b); }
                edges.insert(std::make_pair(a, b));

                nodes[a].neighbors.insert(b);
                nodes[b].neighbors.insert(a);
            }

            std::cout << (you_bi(nodes, edges) ? "BICOLORABLE." : "NOT BICOLORABLE.") << std::endl;
        }
    }

    return 0;
}
