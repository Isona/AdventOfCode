def directionChange(current, change):
    if current == "up":
        if change == "left":
            return "left"
        else: return "right"
    elif current == "down":
        if change == "left":
            return "right"
        else: return "left"
    elif current == "left":
        if change == "left":
            return "down"
        else: return "up"
    else: #current == right
        if change == "left":
            return "up"
        else: return "down"

def strCoord(x, y):
    return str(x) + "," + str(y)

infectedSet = set()

with open("22input.txt", "r") as file:
    i = -12
    for line in file:
        listItems = list(line.replace("\n", ""))
        for j in range(0, 25):
            if listItems[j] == "#":
                infectedSet.add(strCoord(j-12, i))
        i += 1
#print infectedSet

direction = "up"
x = 0
y = 0
infectionCount = 0

for i in range(0, 10000):
    if strCoord(x, y) in infectedSet:
        direction = directionChange(direction, "right")
        infectedSet.remove(strCoord(x, y))
    else:
        direction = directionChange(direction, "left")
        infectedSet.add(strCoord(x, y))
        infectionCount += 1

    if direction == "up":
        y -= 1
    elif direction == "down":
        y += 1
    elif direction == "left":
        x -= 1
    else:
        x += 1

print(infectionCount)


