import collections

plants = collections.deque()
patterns = [0]*32

with open("12input.txt") as file:
    initialState = ((file.readline().split())[2])
    plants.extend(list(initialState))
    for plant in range(len(plants)):
        if(plants[plant] == "#"):
            plants[plant] = 1
        else:
            plants[plant] = 0

    #Read a blank line
    file.readline()

    for line in file:
        line = line.split()
        pattern = "0b"
        for character in line[0]:
            if character == "#":
                pattern += "1"
            else:
                pattern += "0"
        if line[2] == "#":
            patterns[int(pattern, 2)] = 1
        else:
            patterns[int(pattern, 2)] = 0

appended = 5
margin = 2
leftMost = 0

for x in range(20):
    newPrev = 0
    newCurrent = 0

    for i in range(appended):
        plants.appendleft(0)
        plants.append(0)
    leftMost -= appended
    for i in range(margin, len(plants) - margin):
        pattern = ""
        for n in range(i-2,i+3):
            pattern += str(plants[n])

        plants[i - 2] = newPrev
        newPrev = newCurrent
        newCurrent = patterns[int(pattern,2)]

sum = 0
for i in range(len(plants)):
    if plants[i] == 1:
        sum += (i+leftMost)

print(sum)
