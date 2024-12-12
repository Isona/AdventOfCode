with open("3input.txt") as file:
    line1Path = file.readline().split(",")
    line2Path = file.readline().split(",")

line1Locs = set()
line1Loc = [0,0]
line1Locs.add(str(line1Loc[0])+","+str(line1Loc[1]))
for direction in line1Path:
    ordinal = direction[0]
    value = int(direction[1:])
    if ordinal == "R":
        for i in range(value):
            line1Loc[0] += 1
            line1Locs.add(str(line1Loc[0])+","+str(line1Loc[1]))
    elif ordinal == "L":
        for i in range(value):
            line1Loc[0] -= 1
            line1Locs.add(str(line1Loc[0])+","+str(line1Loc[1]))
    elif ordinal == "U":
        for i in range(value):
            line1Loc[1] += 1
            line1Locs.add(str(line1Loc[0])+","+str(line1Loc[1]))
    elif ordinal == "D":
        for i in range(value):
            line1Loc[1] -= 1
            line1Locs.add(str(line1Loc[0])+","+str(line1Loc[1]))


#print(line1Locs)

minDistance = 100000000000000000000000000000
line2Loc = [0,0]
for direction in line2Path:
    ordinal = direction[0]
    value = int(direction[1:])
    if ordinal == "R":
        for i in range(value):
            line2Loc[0] += 1
            if str(line2Loc[0])+","+str(line2Loc[1]) in line1Locs:
                if abs(line2Loc[0])+abs(line2Loc[1]) < minDistance:
                    minDistance = abs(line2Loc[0])+abs(line2Loc[1])
    elif ordinal == "L":
        for i in range(value):
            line2Loc[0] -= 1
            if str(line2Loc[0])+","+str(line2Loc[1]) in line1Locs:
                if abs(line2Loc[0])+abs(line2Loc[1]) < minDistance:
                    minDistance = abs(line2Loc[0])+abs(line2Loc[1])
    elif ordinal == "U":
        for i in range(value):
            line2Loc[1] += 1
            if str(line2Loc[0])+","+str(line2Loc[1]) in line1Locs:
                if abs(line2Loc[0])+abs(line2Loc[1]) < minDistance:
                    minDistance = abs(line2Loc[0])+abs(line2Loc[1])
    elif ordinal == "D":
        for i in range(value):
            line2Loc[1] -= 1
            if str(line2Loc[0])+","+str(line2Loc[1]) in line1Locs:
                if abs(line2Loc[0])+abs(line2Loc[1]) < minDistance:
                    minDistance = abs(line2Loc[0])+abs(line2Loc[1])


print(minDistance)