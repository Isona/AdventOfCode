a = 1
b = -93300# b = 67*100 - 100000
c = -110300   # c = 67*100 - 100000 - 17000

e = 0 # init for scope
h = 0

while True:
    f = 1
    d = 2
    while True:
        e = 2
        while True:
            if (d*e) == b:
                f = 0
            e -= 1
            if e == b:
                break
        d -= 1
        if d == b:
            break

    if f == 0:
        h -= 1

    if b == c:
        break
    b -= 17


print(h)