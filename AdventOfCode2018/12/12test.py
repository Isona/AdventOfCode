n = 50000000000 - 2
twoHundredScore = 20151
differences = [129, 90, 142, 149, 131, 144, 130, 183, 130]
sumDiff = sum(differences)
print(sumDiff)

if n > 200:
    diff = n - 200
    base = diff // 9
    print(base)
    triangle = (base-1) * base//2
    print(triangle)
    baseScore = twoHundredScore + base * sumDiff + triangle * 36#(base - 1) * 36
    print(baseScore)
    finalScore = baseScore
    adding = diff % 9
    print(adding)
    for n in range(adding):
        finalScore += base*4 + differences[n]
    print(finalScore)

#209 is actually 21379
#this breaks at 227 :(
# 227 = 23943

# 227 is + 36 + 72
# Triangle numbers yay


#n * (n + 1) // 2


# 555555557811111113166 is too high
# 555555557788888890944 - this is still too high (n = however many minus 2)