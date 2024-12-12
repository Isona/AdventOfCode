with open("3input.txt") as file:
    line1Path = file.readline().split(",")
    line2Path = file.readline().split(",")

line1Locs = {}
line1Steps = 0
line1Loc = [0,0]
line1Locs[str(line1Loc[0])+","+str(line1Loc[1])] = 0
for direction in line1Path:
    ordinal = direction[0]
    value = int(direction[1:])
    if ordinal == "R":
        for i in range(value):
            line1Loc[0] += 1
            line1Steps += 1
            key = str(line1Loc[0])+","+str(line1Loc[1])
            if key not in line1Locs.keys():
                line1Locs[key] = line1Steps
    elif ordinal == "L":
        for i in range(value):
            line1Loc[0] -= 1
            line1Steps += 1
            key = str(line1Loc[0])+","+str(line1Loc[1])
            if key not in line1Locs.keys():
                line1Locs[key] = line1Steps
    elif ordinal == "U":
        for i in range(value):
            line1Loc[1] += 1
            line1Steps += 1
            key = str(line1Loc[0])+","+str(line1Loc[1])
            if key not in line1Locs.keys():
                line1Locs[key] = line1Steps
    elif ordinal == "D":
        for i in range(value):
            line1Loc[1] -= 1
            line1Steps += 1
            key = str(line1Loc[0])+","+str(line1Loc[1])
            if key not in line1Locs.keys():
                line1Locs[key] = line1Steps


#print(line1Locs)

minDistance = 100000000000000000000000000000
line2Loc = [0,0]
line2Steps = 0

for direction in line2Path:
    ordinal = direction[0]
    value = int(direction[1:])
    if ordinal == "R":
        for i in range(value):
            line2Loc[0] += 1
            line2Steps += 1
            key = str(line2Loc[0])+","+str(line2Loc[1])
            if key in line1Locs.keys():
                if line1Locs[key] + line2Steps < minDistance:
                    minDistance = line1Locs[key] + line2Steps
    elif ordinal == "L":
        for i in range(value):
            line2Loc[0] -= 1
            line2Steps += 1
            key = str(line2Loc[0])+","+str(line2Loc[1])
            if key in line1Locs.keys():
                if line1Locs[key] + line2Steps < minDistance:
                    minDistance = line1Locs[key] + line2Steps
    elif ordinal == "U":
        for i in range(value):
            line2Loc[1] += 1
            line2Steps += 1
            key = str(line2Loc[0])+","+str(line2Loc[1])
            if key in line1Locs.keys():
                if line1Locs[key] + line2Steps < minDistance:
                    minDistance = line1Locs[key] + line2Steps
    elif ordinal == "D":
        for i in range(value):
            line2Loc[1] -= 1
            line2Steps += 1
            key = str(line2Loc[0])+","+str(line2Loc[1])
            if key in line1Locs.keys():
                if line1Locs[key] + line2Steps < minDistance:
                    minDistance = line1Locs[key] + line2Steps


print(minDistance)