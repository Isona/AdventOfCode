def turn(currentDir, direction):
    if direction == "L":
        if currentDir == "N":
            return("W")
        elif currentDir == "W":
            return("S")
        elif currentDir == "S":
            return("E")
        elif currentDir == "E":
            return("N")

    elif direction == "R":
        if currentDir == "N":
            return("E")
        elif currentDir == "W":
            return("N")
        elif currentDir == "S":
            return("W")
        elif currentDir == "E":
            return("S")


currentDir = "N"
currentLoc = [0, 0]
values = []

with open("1input.txt") as file:
    for line in file:
        values = line.rstrip().split(", ")

for line in values:
    print(line)
    direction = line[0]
    value = int(line[1:])

    currentDir = turn(currentDir, direction)

    if currentDir == "N":
        currentLoc[1] += value
    elif currentDir == "W":
        currentLoc[0] -= value
    elif currentDir == "S":
        currentLoc[1] -= value
    elif currentDir == "E":
        currentLoc[0] += value

print(currentLoc)
print(abs(currentLoc[0]) + abs(currentLoc[1]))
