# Project Euler 12

# This problem asks to find the first triangle with more than 500 divisors.

using Primes

function first_highly_divisible_triangle_order_exceeding(n)
  k = 2
  while true
    triangle = k * (k + 1) ÷ 2
    factors = Primes.factor(triangle)
    count_divisors = 1
    # For each prime in the prime factorization, the exponent plus 1 is
    # the number of ways the prime can factor in to the triangle number.
    # The products like this for all prime factors is the number of divisors
    # of the triangle number.
    for (prime, exponent) in collect(factors)
      count_divisors *= (exponent + 1)
    end
    if count_divisors > n
      return triangle
    end
    k += 1
  end
end

#println(first_highly_divisible_triangle_order_exceeding(5))
println(first_highly_divisible_triangle_order_exceeding(500))
