# Project Euler 15

C_cache = Dict((0, 0) => 0)

function C(m, n)
  if m == 1
    return n + 1
  elseif n == 1
    return m + 1
  end

  if haskey(C_cache, (m, n))
    return C_cache[(m, n)]
  end

  # Number of paths to bottow right of square (m, n)
  # is equal to the sum of
  #   * the number of paths to the left neighbor and
  #   * the number of paths to the above neighbor.
  value = C(m - 1, n) + C(m, n - 1)

  C_cache[(m, n)] = value
  return value
end

#println(C(2, 2))
println(C(20, 20))
