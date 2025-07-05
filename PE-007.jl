# Project Euler 7

# This problem asks to identify the 10001st prime number, where 13 is
# considered the 6th prime.

using Primes

function find_nth_prime(n)
  last = 2
  n -= 1
  while n > 0
    last = Primes.nextprime(last + 1)
    n -= 1
  end
  return last
end

println(find_nth_prime(10001))
