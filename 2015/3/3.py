with open("3input.txt") as file:
    input = file.readline()


santaLocation = [0, 0]
roboLocation = [0, 0]
locations = set()
locations.add(str(santaLocation))

for i in range(0,len(input),2):
    if input[i] == "^":
        santaLocation[1] += 1
    elif input[i] == "v":
        santaLocation[1] -= 1
    elif input[i] == ">":
        santaLocation[0] += 1
    elif input[i] == "<":
        santaLocation[0] -= 1
    locations.add(str(santaLocation))

    if input[i+1] == "^":
        roboLocation[1] += 1
    elif input[i+1] == "v":
        roboLocation[1] -= 1
    elif input[i+1] == ">":
        roboLocation[0] += 1
    elif input[i+1] == "<":
        roboLocation[0] -= 1
    locations.add(str(roboLocation))

print(len(locations))