# a = 65
# b = 8921

a = 873
b = 583
matchCount = 0
mask = 65535

for count in range(0, 5000000):
    while True:
        a = a * 16807 % 2147483647
        if a % 4 == 0:
            break
    
    while True:
        b = b * 48271 % 2147483647
        if b % 8 == 0:
            break
    
    if (a & mask) == (b & mask):
        matchCount += 1

print(matchCount)