# Project Euler 2

# This problem asks for the sum of the even Fibonacci numbers not exceeding
# four million. This solution generates the sequence of numbers and totals
# the even ones.

function sum_even_fibonacci_not_exceeding(n)
  a = 1
  b = 2
  total = 0
  while b <= n
    if b % 2 == 0
      total += b
    end
    c = a + b
    a = b
    b = c
  end
  return total
end

println(sum_even_fibonacci_not_exceeding(4_000_000))
