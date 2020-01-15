with open("4input.txt") as file:
    values = file.readlines()
values.sort()
#print(values)

guards = {}
currentGuard = ""
sleeping = False
fellAsleep = 0

maxSleep = 0
mostSleepy = ""

for value in values:
    splitVal = value.split()
    if splitVal[2] == "Guard":
        #print(splitVal[3])
        currentGuard = splitVal[3]
        if currentGuard not in guards:
            guards[currentGuard] = 0
    elif splitVal[2] == "falls":
        #print("Sleeping")
        fellAsleep = int(splitVal[1].split(":")[1][:-1])
        sleeping = True
    elif splitVal[2] == "wakes":
        #print("Awake")
        if sleeping:
            sleeping = False
            wakes = int(splitVal[1].split(":")[1][:-1])
            guards[currentGuard] += wakes - fellAsleep

            if guards[currentGuard] > maxSleep:
                maxSleep = guards[currentGuard]
                mostSleepy = currentGuard


print("Most sleepy: " + mostSleepy + " - " + str(maxSleep))
minutes = [0 for x in range(60)]

for value in values:
    splitVal = value.split()
    if splitVal[2] == "Guard":
        currentGuard = splitVal[3]
    elif splitVal[2] == "falls":
        #print("Sleeping")
        fellAsleep = int(splitVal[1].split(":")[1][:-1])
        sleeping = True
    elif splitVal[2] == "wakes":
        #print("Awake")
        if mostSleepy == currentGuard and sleeping:
            sleeping = False
            wakes = int(splitVal[1].split(":")[1][:-1])
            guards[currentGuard] += wakes - fellAsleep
            for i in range(wakes-fellAsleep):
                minutes[fellAsleep+i] += 1

print(minutes)
print(max(minutes))
print("Most sleepy guard is " + mostSleepy + " and the most common minute was " + str(minutes.index(max(minutes))))

print(minutes.index(max(minutes)) * int(mostSleepy[1::]) )

#17465 - too low