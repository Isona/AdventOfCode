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
visited = set()
visited.add(str(currentLoc))

with open("1input.txt") as file:
    for line in file:
        values = line.rstrip().split(", ")

for line in values:
    print(line)
    direction = line[0]
    value = int(line[1:])

    currentDir = turn(currentDir, direction)

    if currentDir == "N":
        for i in range(value):
            currentLoc[1] += 1
            if str(currentLoc) in visited:
                print(currentLoc)
                print(visited)
                print("Found duplicate!: " + str(currentLoc))
                print(abs(currentLoc[0]) + abs(currentLoc[1]))
                exit()
            visited.add(str(currentLoc))

    elif currentDir == "W":
        for i in range(value):
            currentLoc[0] -= 1
            if str(currentLoc) in visited:
                print(currentLoc)
                print(visited)
                print("Found duplicate!: " + str(currentLoc))
                print(abs(currentLoc[0]) + abs(currentLoc[1]))
                exit()
            visited.add(str(currentLoc))

    elif currentDir == "S":
        for i in range(value):
            currentLoc[1] -= 1
            if str(currentLoc) in visited:
                print(currentLoc)
                print(visited)
                print("Found duplicate!: " + str(currentLoc))
                print(abs(currentLoc[0]) + abs(currentLoc[1]))
                exit()
            visited.add(str(currentLoc))

    elif currentDir == "E":
        for i in range(value):
            currentLoc[0] += 1
            if str(currentLoc) in visited:
                print(currentLoc)
                print(visited)
                print("Found duplicate!: " + str(currentLoc))
                print(abs(currentLoc[0]) + abs(currentLoc[1]))
                exit()
            visited.add(str(currentLoc))



print(visited)
print(currentLoc)
print(abs(currentLoc[0]) + abs(currentLoc[1]))
