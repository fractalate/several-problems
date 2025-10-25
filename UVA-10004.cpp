// https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=12&page=show_problem&problem=945
//
// g++ -lm -lcrypt -O2 -std=c++11 -pipe -DONLINE_JUDGE UVA-10004.cpp

#include <iostream>
#include <cstring>

int adj[200][200];

bool you_bi(size_t v) {
    if (v < 2) {
        return false;
    } else if (v == 2) {
        return true; // our graph is guaranteed to be connected
    }

    for (size_t i = 0; i < v - 2; ++i) {
        for (size_t j = i + 1; j < v - 1; ++j) {
            if (adj[i][j]) {
                for (size_t k = j + 1; k < v; ++k) {
                    if (adj[k][j] && adj[i][k]) {
                        return false;
                    }
                }
            }
        }
    }

    return true;
}

int main() {
    size_t v = 0;
    size_t e = 0;

    while (std::cin >> v) {
        if (v == 0) {
            break;
        }

        if (std::cin >> e) {
            std::memset(adj, 0, sizeof(adj));

            for (int i = 0; i < e; ++i) {
                size_t a = 0, b = 0;
                std::cin >> a >> b;
                adj[a][b] = adj[b][a] = 1;
            }

            std::cout << (you_bi(v) ? "BICOLORABLE." : "NOT BICOLORABLE.") << std::endl;
        }
    }

    return 0;
}
