input = ""
currentIndex = 0

def getNode():
    global currentIndex
    value = 0
    children = input[currentIndex]
    metadata = input[currentIndex+1]
    currentIndex += 2
    foundChildren = 0
    childValues = []

    while foundChildren < children:
        childValues.append(getNode())
        foundChildren += 1

    if children == 0:
        for i in range(metadata):
            value += input[currentIndex + i]
    else:
        for i in range(metadata):
            currentMeta = input[currentIndex + i]
            if currentMeta != 0 and currentMeta <= children:
                value += childValues[currentMeta - 1]
    currentIndex += metadata
    return(value)


with open("8input.txt") as file:
    input = file.readline()

input = input.split()
for i in range(len(input)):
    input[i] = int(input[i])

sum = getNode()

print(str(sum))