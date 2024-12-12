def getString(x, y):
    return(str(x)+","+str(y))


with open("6input.txt") as file:
    instructions = file.readlines()

onLights = {}

for instruction in instructions:
    parts = instruction.split()

    if parts[0] == "toggle":
        coords = parts[1].split(",")
        coords2 = parts[3].split(",")
        for x in range(int(coords[0]),int(coords2[0])+1):
            for y in range(int(coords[1]),int(coords2[1])+1):
                stringCoord = getString(x, y)
                if stringCoord in onLights:
                    onLights[stringCoord] += 2
                else:
                    onLights[stringCoord] = 2
    elif parts[1] == "on":
        coords = parts[2].split(",")
        coords2 = parts[4].split(",")
        for x in range(int(coords[0]),int(coords2[0])+1):
            for y in range(int(coords[1]),int(coords2[1])+1):
                stringCoord = getString(x, y)
                if stringCoord in onLights:
                    onLights[stringCoord] += 1
                else:
                    onLights[stringCoord] = 1
    elif parts[1] == "off":
        coords = parts[2].split(",")
        coords2 = parts[4].split(",")
        for x in range(int(coords[0]),int(coords2[0])+1):
            for y in range(int(coords[1]),int(coords2[1])+1):
                stringCoord = getString(x, y)
                if stringCoord in onLights:
                    if onLights[stringCoord] > 0:
                        onLights[stringCoord] -= 1
                else:
                    onLights[stringCoord] = 0


print(sum(onLights.values()))