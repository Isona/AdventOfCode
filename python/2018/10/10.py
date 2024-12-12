import sys
from parse import compile

p = compile("position=<{:d},{:d}>velocity=<{:d},{:d}>")

points = []

with open("10input.txt") as file:
    for line in file:
        line = line.replace(" ", "")
        xCoord, yCoord, xVel, yVel = p.parse(line)
        points.append([xCoord, yCoord, xVel, yVel])

converging = True
smallestRectangle = 1000000000000000000000000000000000
multiplier = 0
totalSmallestX = 0
totalSmallestY = 0
totalLargestX = 0
totalLargestY = 0

while converging:
    smallestX = sys.maxsize
    smallestY = sys.maxsize
    largestX = 0
    largestY = 0

    for point in points:
        currentX = point[0] + multiplier * point[2]
        currentY = point[1] + multiplier * point[3]
        if currentX < smallestX:
            smallestX = currentX
        if currentY < smallestY:
            smallestY = currentY
        if currentX > largestX:
            largestX = currentX
        if currentY > largestY:
            largestY = currentY

    currentArea = (largestX - smallestX) * (largestY - smallestY)

    if currentArea < smallestRectangle:
        smallestRectangle = currentArea
        totalSmallestX = smallestX
        totalSmallestY = smallestY
        totalLargestX = largestX
        totalLargestY = largestY
        multiplier += 1
    else:
        converging = False

print("SmallestX: " + str(totalSmallestX) + " LargestX: " + str(totalLargestX))
print("SmallestY : " + str(totalSmallestY) + " LargestY: " + str(totalLargestY))
print(totalLargestX - totalSmallestX)
print(totalLargestY - totalSmallestY)
print(smallestRectangle)
multiplier -= 1

grid = [["-" for x in range(totalLargestX - totalSmallestX + 1)] for y in range(totalLargestY - totalSmallestY + 1)]
#grid = [["-" for x in range(totalLargestY - totalSmallestY + 1)] for y in range(totalLargestX - totalSmallestX + 1)]

for point in points:
    currentX = point[0] + multiplier * point[2]
    currentY = point[1] + multiplier * point[3]
    #print("CurrentX: " + str(currentX) + " GridX: " + str(currentX - totalSmallestX) + " CurrentY: " + str(currentY) + " GridY: " + str(currentY - totalSmallestY))
    #grid[currentX - totalSmallestX][currentY - totalSmallestY] = "#"
    grid[currentY - totalSmallestY][currentX - totalSmallestX] = "#"


for x in grid:
    output = ""
    for y in x:
        output += y
    print(output)

print(multiplier)