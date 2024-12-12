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

appended = 4
margin = 2
leftMost = 0
prevSum = 0
prevDiff = 0

differences = [129, 90, 142, 149, 131, 144, 130, 183, 130]
sumDiff = sum(differences)

for x in range(3000):
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

    printable = ""
    foundFirst = False
    # for i in range(len(plants)):
    #     # if not foundFirst:
    #     #     foundFirst = (plants[i] == 1)
    #     # if foundFirst:
    #     printable += str(plants[i])

    sum = 0
    for i in range(len(plants)):
        if plants[i] == 1:
            sum += (i+leftMost)
    #print(str(x) + ": Sum: " + str(sum) + ": " + printable.strip("0"))
    print(str(x) + ": Sum: " + str(sum) + " Difference: " + str(sum - prevSum) + " Difference difference: " + str(sum-prevSum - prevDiff))
    prevDiff = sum-prevSum
    prevSum = sum

    if x > 200:
        diff = x - 200
        base = diff // 9
        triangle = (base-1) * base//2
        baseScore = 20151 + base * sumDiff + triangle * 36
        finalScore = baseScore
        adding = diff % 9
        for l in range(adding):
            finalScore += base*4 + differences[l]
        if(sum != finalScore):
            print("[+] " + str(x) + ": Guess and real score don't match! Guess: " + str(finalScore) + " Real: " + str(sum))

print(sum)
