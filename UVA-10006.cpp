// https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&category=12&page=show_problem&problem=947
//
// g++ -lm -lcrypt -O2 -std=c++11 -pipe -DONLINE_JUDGE UVA-10006.cpp

#include <iostream>

/*
Just a list of the relevant Carmichael numbers because the list is quite small.
Determined by pre-computing them:

from sympy import nextprime

def generate_primes():
    n = 2
    while True:
        yield n
        n = nextprime(n)

primes = set()
for p in generate_primes():
    if p > 65000:
        break
    primes.add(p)

def pow_mod(a, n):
    result = 1
    for i in range(n):
        result = (result * a) % n
    return result

def is_carmichael(n):
    if n in primes:
        return False
    for a in range(2, n):
        if pow_mod(a, n) != a:
            return False
    return True

for n in range(2, 65000 + 1):
    if is_carmichael(n):
        print(n)
*/
bool is_carmichael(int n) {
    switch (n) {
        case 561:
        case 1105:
        case 1729:
        case 2465:
        case 2821:
        case 6601:
        case 8911:
        case 10585:
        case 15841:
        case 29341:
        case 41041:
        case 46657:
        case 52633:
        case 62745:
        case 63973:
            return true;
    }
    return false;
}

int main() {
    int n = 0;
    while (std::cin >> n) {
        if (n == 0) {
            break;
        } else if (is_carmichael(n)) {
            std::cout << "The number " << n << " is a Carmichael number." << std::endl;
        } else {
            std::cout << n << " is normal." << std::endl;
        }
    }
}
