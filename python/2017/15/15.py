# a = 65
# b = 8921

a = 873
b = 583
matchCount = 0
mask = 65535

for count in range(0, 40000000):
    a = a * 16807 % 2147483647
    b = b * 48271 % 2147483647
    
    if (a & mask) == (b & mask):
        matchCount += 1

print(matchCount)