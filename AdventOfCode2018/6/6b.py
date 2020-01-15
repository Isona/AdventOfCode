points = []
inRegion = 0

with open("6input.txt") as file:
    for line in file:
        pointString = line.split(", ")
        x = int(pointString[0])
        y = int(pointString[1].rstrip())
        points.append((x,y))

for x in range(0,400):
    for y in range(0,400):
        totalDistance = 0
        for point in points:
            currentDist = abs(x-point[0]) + abs(y-point[1])
            totalDistance += currentDist
        if totalDistance < 10000:
            inRegion += 1

print(str(inRegion))