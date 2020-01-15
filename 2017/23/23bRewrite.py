b = -93300# b = 67*100 - 100000
c = -110300# c = 67*100 - 100000 - 17000
h = 0

e = 0 # init for scope
h = 0

while True:

    if hasFactors(b):
        h -= 1
    # while True:
    #     if b%d == 0:
    #         f = 0

    #     d -= 1
    #     if d == b:
    #         break

    if b == c:
        break
    b -= 17

print(h)