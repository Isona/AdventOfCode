width = 25
height = 6

layerLen = width*height
leastZerosLayer = -1
leastZeroCount = 1000000000000000
leastZeroValue = 100000000000000

with open("8input.txt") as file:
    contents = file.readline()

for index in range(0, len(contents), layerLen):
    zeroCount = 0
    oneCount = 0
    twoCount = 0

    print(contents[index:index+layerLen])
    for character in contents[index:index+layerLen]:
        #print(character)
        if character == "0":
            zeroCount += 1
        elif character == "1":
            oneCount += 1
        elif character == "2":
            twoCount += 1


    if zeroCount < leastZeroCount:
        print(str(zeroCount)+" is less than " + str(leastZeroCount))
        leastZeroCount = zeroCount
        leastZerosLayer = (index+layerLen)//layerLen
        leastZeroValue = oneCount * twoCount



print(leastZerosLayer)
print(leastZeroValue)