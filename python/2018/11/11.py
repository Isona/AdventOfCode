grid = [[0 for x in range(300)] for y in range(300)]
gridSerial = 7672

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

for x in range(0, 298):
    for y in range(0, 298):
        currentPower = 0
        for subX in range(3):
            for subY in range(3):
                currentPower += grid[x+subX][y+subY]

        if currentPower > greatestPower:
            greatestPower = currentPower
            greatestX = x
            greatestY = y

print("Greatest power: " + str(greatestPower) + " X: " + str(greatestX+1) + " Y: " + str(greatestY+1))