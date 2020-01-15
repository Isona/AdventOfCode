import math

with open("10input.txt") as file:
#with open("10test2input.txt") as file:
    values = file.readlines()

asteroids = []
for y in range(len(values)):
    row = values[y].rstrip()
    for x in range(len(row)):
        if row[x] == "#":
            asteroids.append((x, y))

maxVisible = 0
baseAsteroid = 0
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
        baseAsteroid = asteroid

asteroids = sorted(asteroids, key=lambda x: (x[0]-baseAsteroid[0])**2+(x[1]-baseAsteroid[1])**2)
gradients = {}

for asteroid in asteroids:
    if asteroid == baseAsteroid:
        continue
    x = (asteroid[1] - baseAsteroid[1])
    y = (asteroid[0] - baseAsteroid[0])
    gradients[asteroid] = math.atan2(y,x)

destroyed = set()
previousDestroyed = 0
gradients = sorted(gradients.items(), key=lambda x: x[1], reverse=True)
numberDestroyed = 0

for gradient in gradients:
    #print(gradient)
    if gradient[0] in destroyed:
        continue
    if gradient[1] == previousDestroyed:
        continue
    previousDestroyed = gradient[1]
    numberDestroyed += 1
    destroyed.add(gradient[0])
    print(str(numberDestroyed)+": " + str(gradient[0]))