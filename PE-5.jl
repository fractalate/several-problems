# Project Euler 5

# This problem asks for the smalles number evenly divisible by the numbers
# 1 to 20. This could be done by finding successive LCMs.

function smallest_multiple_of_numbers_to(n)
  least = 1
  for i in 2:n
    least = lcm(least, i)
  end
  return least
end

println(smallest_multiple_of_numbers_to(20))
