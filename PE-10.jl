# Project Euler 10

# This problem asks to find the sum of the primes less than a given number.
# This is accomplished by generatig all the primes less than that number and
# summing them together.

using Primes

function sum_of_primes_below(n)
  total = 0
  for prime in Primes.primes(n - 1)
    total += prime
  end
  return total
end

#println(sum_of_primes_below(10))
#println(sum_of_primes_below(11)) # Testing the "below" condition with a prime.
println(sum_of_primes_below(2_000_000))
