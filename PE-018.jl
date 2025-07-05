# Project Euler 18

# This problem asks to find the sum of the path through the rows of
# the triangle that is maximal.

function compute_max_sums(below::Array, above::Array)
  result = copy(above)
  for i in 1:length(above)
    result[i] += max(below[i], below[i + 1])
  end
  return result
end

#below = [8 5 9 3]
#below = compute_max_sums(below, [2 4 6])
#below = compute_max_sums(below, [7 4])
#below = compute_max_sums(below, [3])
#println(below)

below = [04 62 98 27 23 09 70 98 73 93 38 53 60 04 23]
below = compute_max_sums(below, [63 66 04 68 89 53 67 30 73 16 69 87 40 31])
below = compute_max_sums(below, [91 71 52 38 17 14 91 43 58 50 27 29 48])
below = compute_max_sums(below, [70 11 33 28 77 73 17 78 39 68 17 57])
below = compute_max_sums(below, [53 71 44 65 25 43 91 52 97 51 14])
below = compute_max_sums(below, [41 48 72 33 47 32 37 16 94 29])
below = compute_max_sums(below, [41 41 26 56 83 40 80 70 33])
below = compute_max_sums(below, [99 65 04 28 06 16 70 92])
below = compute_max_sums(below, [88 02 77 73 07 63 67])
below = compute_max_sums(below, [19 01 23 75 03 34])
below = compute_max_sums(below, [20 04 82 47 65])
below = compute_max_sums(below, [18 35 87 10])
below = compute_max_sums(below, [17 47 82])
below = compute_max_sums(below, [95 64])
below = compute_max_sums(below, [75])
println(below)
