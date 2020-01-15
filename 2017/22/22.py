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

def move(position, direction):
    if direction == "up":
        return position - 101
    elif direction == "down":
        return position + 101
    elif direction == "left":
        return position - 1
    else: #direction == right
        return position + 1

grid = ["."]*(101**2)

with open("22test.txt", "r") as file:
    i = 38
    for line in file:
        grid[(i*101 + 37):(i*101 + 62)] = list(line.replace("\n",""))
        i += 1

print "".join(grid)

currentPos = len(grid)//2
print currentPos
direction = "up"
infectionCount = 0

for i in xrange(0, 7):
    print i, currentPos, direction
    if grid[currentPos] == ".":
        print "Node clean so turning left"
        direction = directionChange(direction, "left")
        grid[currentPos] = "#"
        infectionCount += 1
    else:
        print "Node infected so turning right"
        direction = directionChange(direction, "right")
        grid[currentPos] = "."

    currentPos = move(currentPos, direction)

#print grid
print infectionCount
