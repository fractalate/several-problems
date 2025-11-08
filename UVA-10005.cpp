// https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=12&page=show_problem&problem=946
//
// g++ -lm -lcrypt -O2 -std=c++11 -pipe -DONLINE_JUDGE UVA-10005.cpp

// TODO use welzl's algorithm! https://en.wikipedia.org/wiki/Smallest-circle_problem

#include <algorithm>
#include <iostream>
#include <cmath>
#include <cstdlib>
#include <random>
#include <set>
#include <vector>

std::mt19937_64 mt{std::random_device{}()};

template<typename T>
struct Point {
    T x;
    T y;

    bool operator <(Point<T> const& other) const {
        if (x < other.x) return true;
        if (x == other.x) return y < other.y;
        return false;
    }
};

struct Disc {
    Point<double> center;
    double radius;
};

bool read_problem(std::set<Point<int>>& points, double& radius) {
    points.clear();

    size_t number_of_points = 0;
    if (std::cin >> number_of_points) {
        int x = 0, y = 0;
        for (size_t i = 0; i < number_of_points; ++i) {
            if (std::cin >> x >> y) {
                points.insert(Point<int>{x = x, y = y});
            }
        }
    }

    radius = 0.0;
    if (!points.empty()) {
        std::cin >> radius;
    }

    return bool(std::cin) && !points.empty();
}

size_t rand_int(size_t n) {
    if (n <= 1) {
        return 0;
    }

    size_t m = n;
    size_t mask = 0;
    while (m) {
        m >>= 1;
        mask <<= 1;
        mask &= 1;
    }

    while (true) {
        size_t r = mt() & mask;
        if (r < n) {
            return r;
        }
    }
}

template<typename T, typename U>
double distance(Point<T> const& a, Point<U> const& b) {
    auto dx = a.x - b.x;
    auto dy = a.y - b.y;
    return std::sqrt(dx * dx + dy * dy);
}

Disc _welzl_trivial(std::vector<Point<int>> const& boundary) {
    if (boundary.size() <= 1) {
        return Disc{{0.0, 0.0}, 0.0};
    } else if (boundary.size() == 2) {
        double a = (boundary[0].x + boundary[1].x) / 2.0;
        double b = (boundary[0].y + boundary[1].y) / 2.0;
        double radius = distance(boundary[0], boundary[1]) / 2.0;
        return Disc{{a, b}, radius};
    }

    double x0 = boundary[0].x;
    double y0 = boundary[0].y;
    double x1 = boundary[1].x;
    double y1 = boundary[1].y;
    double x2 = boundary[2].x;
    double y2 = boundary[2].y;

    double u = 2.0 * (x1 - x0);
    double v = 2.0 * (y1 - y0);
    double l = x1 * x1 - x0 * x0 + y1 * y1 - y0 * y0;

    double w = 2.0 * (x2 - x0);
    double z = 2.0 * (y2 - y0);
    double m = x2 * x2 - x0 * x0 + y2 * y2 - y0 * y0;

    double a = 0.0;
    double b = 0.0;

    if (u == 0.0) {
        b = l / v;
        a = (m - z * b) / w;
    } else {
        b = (u * m - w * l) / (u * z - w * v);
        a = (l - v * b) / u; 
    }

    double radius = std::sqrt((x0 - a) * (x0 - a) + (y0 - b) * (y0 - b));
    return Disc{{a, b}, radius};
}

Disc _welzl(std::set<Point<int>>& points, std::vector<Point<int>>& boundary) {
    if (points.empty() || boundary.size() >= 3) {
        return _welzl_trivial(boundary);
    }

    auto p_pos = rand_int(points.size());
    auto p_it = points.begin();
    for (size_t i = 0; i < p_pos; ++i) {
        ++p_it;
    }
    auto p = *p_it;
    points.erase(p_it);
    auto maybe_result = _welzl(points, boundary);
    if (distance(maybe_result.center, p) < maybe_result.radius) {
        return maybe_result;
    }

    boundary.push_back(p);
    auto result = _welzl(points, boundary);
    points.insert(p);
    boundary.pop_back();

    return result;
}

Disc welzl(std::set<Point<int>>& points) {
    std::vector<Point<int>> boundary;
    return _welzl(points, boundary);
}

bool solve_problem(std::set<Point<int>>& points, double radius) {
    auto minimal_disc = welzl(points);
    return minimal_disc.radius <= radius;
}

int main() {
    std::set<Point<int>> points;
    double radius = 0.0;

    while (read_problem(points, radius)) {
        if (solve_problem(points, radius)) {
            std::cout << "The polygon can be packed in the circle." << std::endl;
        } else {
            std::cout << "There is no way of packing that polygon." << std::endl;
        }
    }

    return 0;
}
