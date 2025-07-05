# Project Euler 3

# This problem asks for the largest prime divisor of a number. I use the
# Primes.jl library for this. To set that up, in a Julia REPL, use the
# following commands:
#
#   using Pkg
#   Pkg.add("Primes")

using Primes

function largest_prime_factor_of(n)
  maximum = 0
  for (factor, _multiplicity) in Primes.eachfactor(n)
    if factor > maximum
      maximum = factor
    end
  end
  return maximum
end

println(largest_prime_factor_of(600851475143))
