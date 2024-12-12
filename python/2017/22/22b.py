def directionChange(current, change):
    if current == "up":
        if change == "left":
            return "left"
        elif change == "right":
            return "right"
        else:
            return "down"
    elif current == "down":
        if change == "left":
            return "right"
        elif change == "right":
            return "left"
        else:
            return "up"
    elif current == "left":
        if change == "left":
            return "down"
        elif change == "right":
            return "up"
        else:
            return "right"
    else: #current == right
        if change == "left":
            return "up"
        elif change == "right":
            return "down"
        else:
            return "left"

def strCoord(x, y):
    return str(x) + "," + str(y)

# 0 = weakened, 1 = infected, 2 = flagged
infectedSet = {}

with open("22input.txt", "r") as file:
    i = -12
    for line in file:
        listItems = list(line.replace("\n", ""))
        for j in range(0, 25):
            if listItems[j] == "#":
                infectedSet[strCoord(j-12, i)] = 1
        i += 1
#print infectedSet

direction = "up"
x = 0
y = 0
infectionCount = 0

for i in range(0, 10000000):
    coord = strCoord(x, y)
    if coord in infectedSet:
        if infectedSet[coord] == 0:
            infectedSet[coord] = 1
            infectionCount += 1
        elif infectedSet[coord] == 1:
            direction = directionChange(direction, "right")
            infectedSet[coord] = 2
        elif infectedSet[coord] == 2:
            direction = directionChange(direction, "back")
            del infectedSet[coord]
    else:
        direction = directionChange(direction, "left")
        infectedSet[coord] = 0

    if direction == "up":
        y -= 1
    elif direction == "down":
        y += 1
    elif direction == "left":
        x -= 1
    else:
        x += 1

print(infectionCount)