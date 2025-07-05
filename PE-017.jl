# Project Euler 17

# This one asks for the number of letters in the text representation
# of the number from 1 to 1000 inclusive.

function count_letters_in_number(n)
  if n == 1
    return 3 # one
  elseif n == 2
    return 3 # two
  elseif n == 3
    return 5 # three
  elseif n == 4
    return 4 # four
  elseif n == 5
    return 4 # five
  elseif n == 6
    return 3 # six
  elseif n == 7
    return 5 # seven
  elseif n == 8
    return 5 # eight
  elseif n == 9
    return 4 # nine

  elseif n == 10
    return 3 # ten
  elseif n == 11
    return 6 # eleven
  elseif n == 12
    return 6 # twelve
  elseif n == 13
    return 8 # thirteen
  elseif n == 14
    return 8 # fourteen
  elseif n == 15
    return 7 # fifteen
  elseif n == 16
    return 7 # sixteen
  elseif n == 17
    return 9 # seventeen
  elseif n == 18
    return 8 # eighteen
  elseif n == 19
    return 8 # nineteen

  elseif n == 20
    return 6 # twenty
  elseif n > 20 && n < 30
    return count_letters_in_number(20) + count_letters_in_number(n % 10)
  elseif n == 30
    return 6 # thirty
  elseif n > 30 && n < 40
    return count_letters_in_number(30) + count_letters_in_number(n % 10)
  elseif n == 40
    return 5 # forty
  elseif n > 40 && n < 50
    return count_letters_in_number(40) + count_letters_in_number(n % 10)
  elseif n == 50
    return 5 # fifty
  elseif n > 50 && n < 60
    return count_letters_in_number(50) + count_letters_in_number(n % 10)
  elseif n == 60
    return 5 # sixty
  elseif n > 60 && n < 70
    return count_letters_in_number(60) + count_letters_in_number(n % 10)
  elseif n == 70
    return 7 # seventy
  elseif n > 70 && n < 80
    return count_letters_in_number(70) + count_letters_in_number(n % 10)
  elseif n == 80
    return 6 # eighty
  elseif n > 80 && n < 90
    return count_letters_in_number(80) + count_letters_in_number(n % 10)
  elseif n == 90
    return 6 # ninety
  elseif n > 90 && n < 100
    return count_letters_in_number(90) + count_letters_in_number(n % 10)

  elseif n == 1000
    return 3 + 8 # one + thousand

  else
    hundresds = n ÷ 100
    rest = n % 100
    count = count_letters_in_number(hundresds) + 7 # xxxx + hundred
    if rest != 0
      count += 3 + count_letters_in_number(rest) # and yyyy (British English)
    end
    return count
  end
end

function count_all_letters()
  total = 0
  for i in 1:1000
    count = count_letters_in_number(i)
    #println(i, " -> ", count)
    total += count
  end
  return total
end

#println(count_letters_in_number(42))
#println(count_letters_in_number(300))
#println(count_letters_in_number(342))
#println(count_letters_in_number(115))

println(count_all_letters())
