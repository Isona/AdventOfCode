import sys

points = []
sums = {}
infinites = []

with open("6input.txt") as file:
    for line in file:
        pointString = line.split(", ")
        x = int(pointString[0])
        y = int(pointString[1].rstrip())
        points.append((x,y))

for point in points:
    sums[point] = 0

for x in range(0,400):
    for y in range(0,400):
        closestPoint = points[0]
        closestDist = sys.maxsize
        closestCount = 0
        for point in points:
            currentDist = abs(x-point[0]) + abs(y-point[1])
            if currentDist < closestDist:
                closestDist = currentDist
                closestPoint = point #deepcopy(point)
                closestCount = 1
            elif currentDist == closestDist:
                closestCount += 1
        if closestCount == 1:
            #print(closestPoint)
            if x == 399 or x == 0 or y == 399 or y == 0:
                if closestPoint not in infinites:
                    infinites.append(closestPoint)
            sums[closestPoint] += 1

greatestCount = 0
greatPoint = points[0]
for sum in sums:
    if sum not in infinites:
        if sums[sum] > greatestCount:
            greatestCount = sums[sum]
            greatestPoint = sum

print(str(greatestPoint) + ": " + str(greatestCount))
