planets = {}

def calcOrbitNum(key):
    global planets
    
    if planets[key][0] not in planets.keys():
        planets[key][1] = 1
    elif planets[planets[key][0]][1] != -1:
        planets[key][1] = planets[planets[key][0]][1] + 1
    elif planets[planets[key][0]][1] == -1:
        calcOrbitNum(key)
        planets[key][1] = planets[planets[key][0]][1] + 1


with open("6testinput.txt") as file:
    for line in file:
        line = line.rstrip().split(")")
        planets[line[1]] = [line[0], -1]

print(planets)


sum = 0
for key in planets.keys():
    if planets[key][1] == -1:
        calcOrbitNum(key)
    sum += planets[key][1]


print(sum)