def getDist(x, y, z):
    return (abs(x) + abs(y) + abs(z))/2

with open("11input.txt", "r") as file:
    data = file.readline().rstrip().split(",")

    currentX = 0
    currentY = 0
    currentZ = 0
    maxDist = 0

    for direction in data:
        if direction == "n":
            currentY += 1
            currentZ -= 1
        elif direction == "s":
            currentY -= 1
            currentZ += 1
        elif direction == "ne":
            currentX += 1
            currentZ -= 1
        elif direction == "nw":
            currentX -= 1
            currentY += 1
        elif direction == "se":
            currentX += 1
            currentY -= 1
        elif direction == "sw":
            currentX -= 1
            currentZ += 1
        if getDist(currentX, currentY, currentZ) > maxDist:
            maxDist = getDist(currentX, currentY, currentZ)

    print("x = " + str(currentX))
    print("y = " + str(currentY))
    print("z = " + str(currentZ))

    
    print("Distance = " + str(getDist(currentX, currentY, currentZ)))
    print("Maximum Distance = " + str(maxDist))