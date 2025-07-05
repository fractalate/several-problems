# Project Euler 6

# This problem asks for the difference between two sums. The first sum is
# the sum of squares which has a formula to calculate directly:
# n*(n+1)*(2n+1)/6. The other sum is the sum of consecutive integers squared
# which also has a formula to calculate directly: (n*(n+1)/2)^2.

function sum_of_integers(n)
  return n * (n + 1) ÷ 2
end

function sum_of_integers_squared(n)
  result = sum_of_integers(n)
  return result * result
end
function sum_of_squares(n)
  return n * (n + 1) * (2 * n + 1) ÷ 6
end

function difference_between_sums(n)
  return sum_of_integers_squared(n) - sum_of_squares(n)
end

println(difference_between_sums(100))
