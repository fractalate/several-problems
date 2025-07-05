def multiples_of_3_or_5(below_this_number: int) -> int:
    ttl = 0

    for n in range(1, below_this_number):
        if n % 3 == 0 or n % 5 == 0:
            ttl += n

    return ttl

if __name__ == '__main__':
    print(multiples_of_3_or_5(10))
    print(multiples_of_3_or_5(1000))
