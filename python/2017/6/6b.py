def findMax(maximum, data):
    for i in range(0, len(data)):
        if data[i] == maximum:
            return i

with open("6input.txt", "r") as file:
    data = file.read().split()
    for i in range(0, len(data)):
        data[i] = int(data[i])

    stateDict = {}
    stateDict[str(data)] = 0
    count = 0
    repeatedState = 0
    while True:
        maximum = max(data)
        maxPlace = findMax(maximum, data)
        data[maxPlace] = 0
        spreadLoc = (maxPlace + 1) % len(data)
        while maximum > 0:
            data[spreadLoc] += 1
            maximum -= 1
            spreadLoc = (spreadLoc + 1)% len(data)

        dataString = str(data)
        count += 1
        if dataString in stateDict:
            repeatedState = stateDict[dataString]
            break
        else:
            stateDict[dataString] = count


    print(repeatedState)
    print(count)
    print(count - repeatedState)