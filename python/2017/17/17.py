skip = 377
spinLock = [0]
currentItem = 0

for i in range(1, 2018):
    currentItem = (currentItem+skip)%len(spinLock) + 1
    spinLock.insert(currentItem, i)
    # print spinLock

print(spinLock)
print(spinLock.index(0))
print(spinLock[0])
print(spinLock[1])
print(spinLock[currentItem])
print(spinLock[(currentItem+1)%len(spinLock)])