input = ""
currentIndex = 0

def getNode():
    global currentIndex
    sum = 0
    children = input[currentIndex]
    metadata = input[currentIndex+1]
    currentIndex += 2
    foundChildren = 0
    while foundChildren < children:
        sum += getNode()
        foundChildren += 1

    for i in range(metadata):
        sum += input[currentIndex + i]
    currentIndex += metadata

    return(sum)


with open("8input.txt") as file:
    input = file.readline()

input = input.split()
for i in range(len(input)):
    input[i] = int(input[i])

sum = getNode()

print(str(sum))