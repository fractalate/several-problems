# Project Euler 4

# This problem asks for the largest palindromic number that's the product
# of two three digit numbers.

function largest_palindrome_product(a0, a1)
  maximum = 0
  for a in a0:(a1-1)
    for b in (a+1):a1
      prod = a * b
      text = string(prod)
      reversed = reverse(text)
      if text == reversed && prod > maximum
        maximum = prod
      end
    end
  end
  return maximum
end

println(largest_palindrome_product(100, 999))
