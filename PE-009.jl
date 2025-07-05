# Project Euler 9

# This problem asks to find the one Pythagorean triplet where a + b + c = 1000
# and then find the product abc.

function find_triple(target_sum)
  for a in 1:target_sum - 1
    for b in a:target_sum
      c = target_sum - a - b
      if a^2 + b^2 == c^2
        return (a, b, c)
      end
    end
  end
  return nothing
end

const IntTripleOrNothing = Union{Tuple{Int, Int, Int}, Nothing}
function triple_prod(triple::IntTripleOrNothing)
  if triple === nothing
    return nothing
  end
  return  triple[1] * triple[2] * triple[3]
end

triple = find_triple(1000)
prod = triple_prod(triple)
println(triple, ' ', prod)
