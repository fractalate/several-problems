# Project Euler 20

# This problem asks to find the digital sum of 100 factorial.

function digital_sum(text::String)
  total = 0
  for c in text
    total += Int(c) - Int('0')
  end
  return total
end

println(digital_sum(
  string(factorial(BigInt(100)))
))
