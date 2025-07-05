# Project Euler 14

# This problems asks to find the longest Collatz sequence for the
# starting numbers under 1 million.

collatz_length = Dict(1 => 1)

function find_collatz_sequence_length(n)
  if haskey(collatz_length, n)
    return collatz_length[n]
  end
  if n % 2 == 0
    len = find_collatz_sequence_length(n ÷ 2) + 1
  else
    len = find_collatz_sequence_length(3 * n + 1) + 1
  end
  collatz_length[n] = len
  return len
end

function find_longest_collatz_sequence_starting_below(n)
  longest = 0
  starting_number = 0
  for i in 1:n
    len = find_collatz_sequence_length(i)
    if len > longest
      longest = len
      starting_number = i
    end
  end
  return starting_number
end

println(find_longest_collatz_sequence_starting_below(1_000_000))
