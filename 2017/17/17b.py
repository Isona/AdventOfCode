skip = 377
afterZero = 0
currentItem = 0

for i in range(1, 50000001):
    currentItem = (currentItem+skip)%i + 1
    if currentItem == 1:
        afterZero = i
    # print spinLock

print(afterZero)