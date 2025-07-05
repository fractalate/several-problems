# Project Euler 16

# This problem asks to find the sum of the digits of a number
# raised to a power.

function power_digit_sum(number, power)
  text = string(BigInt(number)^power)
  total = 0
  for c in text
    total += Int(c) - Int('0')
  end
  return total
end

#println(power_digit_sum(2, 15))
println(power_digit_sum(2, 1000))
