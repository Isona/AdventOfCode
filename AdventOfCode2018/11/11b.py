grid = [[0 for x in range(300)] for y in range(300)]
gridSerial = 1723
#gridSerial = 42

for x in range(300):
    for y in range(300):
        realX = x+1
        realY = y+1
        rackID = realX + 10
        power = rackID * realY
        power += gridSerial
        power *= rackID
        power = int(str(power)[-3])
        power -= 5
        grid[x][y] = power

greatestPower = -3000
greatestX = 0
greatestY = 0
size = 0

summedArea = [[0 for x in range(300)] for y in range(300)]

for x in range(300):
    for y in range(300):
        summedArea[x][y] += grid[x][y]
        if x > 0:
            summedArea[x][y] += summedArea[x-1][y]
        if y > 0:
            summedArea[x][y] += summedArea[x][y-1]
        if x > 0 and y > 0:
            summedArea[x][y] -= summedArea[x-1][y-1]

#print(grid)
#print(summedArea)


for gridSize in range(1,301):
    for x in range(0, 301-gridSize):
        for y in range(0, 301-gridSize):
            currentPower = summedArea[x + gridSize - 1][y + gridSize - 1]
            if x > 0:
                currentPower -= summedArea[x - 1][y + gridSize - 1]
            if y > 0:
                currentPower -= summedArea[x + gridSize - 1][y - 1]
            if x > 0 and y > 0:
                currentPower += summedArea[x-1][y-1]
            #print(currentPower)

            if currentPower > greatestPower:
                greatestPower = currentPower
                greatestX = x
                greatestY = y
                size = gridSize


print("Greatest power: " + str(greatestPower) + " X: " + str(greatestX+1) + " Y: " + str(greatestY+1) + " Gridsize: " + str(size))
print(str(greatestX+1) + "," + str(greatestY + 1) + "," + str(size))