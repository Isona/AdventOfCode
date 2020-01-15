import math

with open("10input.txt") as file:
    values = file.readlines()

#asteroidfield[y][x]
asteroids = []
for y in range(len(values)):
    row = values[y].rstrip()
    for x in range(len(row)):
        if row[x] == "#":
            asteroids.append((x, y))

maxVisible = 0
for asteroid in asteroids:
    gradients = set()
    visible = 0

    for potato in asteroids:
        if potato == asteroid:
            continue
        x = potato[1] - asteroid[1]
        y = potato[0] - asteroid[0]

        gradient = math.atan2(y,x)
        if gradient not in gradients:
            visible += 1
            gradients.add(gradient)

    #print(str(asteroid)+ ": " + str(visible))
    if visible > maxVisible:
        maxVisible = visible

print(maxVisible)
