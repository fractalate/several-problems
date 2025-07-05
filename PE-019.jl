# Project Euler 19

# This problem asks to count the number of Sundays that landed
# on the start of the month in the 20th century (1 Jan 1901 to 31 Dec 2000)
# 1 Jan 1900 was a Monday.

function get_days_per_month(month, year)
  # 30 days has September, April, June, and November...
  if month == 4 || month == 6 || month == 9 || month == 11
    return 30
  # February has 28 except on leap years where it has 29.
  elseif month == 2
    # New centuries are not leap years
    # except for when the year is divisible by 400.
    if year % 100 == 0
      if year % 400 == 0
        return 29
      end
    # Any other year divisible by 4 is a leap year.
    elseif year % 4 == 0
      return 29
    end
    return 28
  end
  # The rest have 31 days.
  return 31
end

function count_first_of_month_sundays()
  count = 0

  year = 1900
  month = 1
  dow = 1 # Monday. Sunday is 0.

  while year < 2001
    if year > 1900
      if dow == 0
        count += 1
      end
    end

    # Go to the next month.
    dow += get_days_per_month(month, year)
    dow %= 7
    month += 1

    # Go to the next year if needed.
    if month > 12
      month = 1
      year += 1
    end
  end

  return count
end

#println(get_days_per_month(2, 1900))
#println(get_days_per_month(2, 2000))
println(count_first_of_month_sundays())
