def getString(x, y):
    return(str(x)+","+str(y))


with open("6input.txt") as file:
    instructions = file.readlines()

onLights = set()

for instruction in instructions:
    parts = instruction.split()

    if parts[0] == "toggle":
        coords = parts[1].split(",")
        coords2 = parts[3].split(",")
        for x in range(int(coords[0]),int(coords2[0])+1):
            for y in range(int(coords[1]),int(coords2[1])+1):
                stringCoord = getString(x, y)
                if stringCoord in onLights:
                    onLights.discard(stringCoord)
                else:
                    onLights.add(stringCoord)
    elif parts[1] == "on":
        coords = parts[2].split(",")
        coords2 = parts[4].split(",")
        for x in range(int(coords[0]),int(coords2[0])+1):
            for y in range(int(coords[1]),int(coords2[1])+1):
                stringCoord = getString(x, y)
                onLights.add(stringCoord)
    elif parts[1] == "off":
        coords = parts[2].split(",")
        coords2 = parts[4].split(",")
        for x in range(int(coords[0]),int(coords2[0])+1):
            for y in range(int(coords[1]),int(coords2[1])+1):
                stringCoord = getString(x, y)
                onLights.discard(stringCoord)


print(len(onLights))