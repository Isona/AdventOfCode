with open("3input.txt") as file:
    values = file.readlines()
    
grid = [[0 for x in range(1000)] for y in range(1000)]

for value in values:
    splitVal = value.split(" ")
    #I'm very sorry for this horror
    startX = int(splitVal[2].split(",")[0])
    startY = int(splitVal[2].split(",")[1].split(":")[0])
    xLen = int(splitVal[3].split("x")[0])
    yLen = int(splitVal[3].split("x")[1].split("\n")[0])

    for x in range(xLen):
        for y in range(yLen):
            grid[startX + x][startY + y] += 1

overTwo = 0

for value in values:
    splitVal = value.split(" ")
    #I'm very sorry for this horror
    startX = int(splitVal[2].split(",")[0])
    startY = int(splitVal[2].split(",")[1].split(":")[0])
    xLen = int(splitVal[3].split("x")[0])
    yLen = int(splitVal[3].split("x")[1].split("\n")[0])

    clash = False
    for x in range(xLen):
        for y in range(yLen):
            if grid[startX + x][startY + y] != 1:
                clash = True
                break
        if clash:
            break
    if not clash:
        print(splitVal[0])
