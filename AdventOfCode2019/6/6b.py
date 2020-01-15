from collections import deque

orbiting = {"COM":["",0]}
orbited = {}

with open("6input.txt") as file:
    for line in file:
        line = line.rstrip().split(")")
        orbiting[line[1]] = [line[0], -1]

        if line[0] not in orbited.keys():
            orbited[line[0]] = [line[1]]
        else:
            orbited[line[0]].append(line[1])

print(orbiting)
print(orbited)

queue = deque(orbited["COM"])

print(queue)
sum = 0

youList = set()
sanList = set()

next = "YOU"
while next != "COM":
    next = orbiting[next][0]
    print(next)
    youList.add(next)

next = "SAN"
while next != "COM":
    next = orbiting[next][0]
    print(next)
    sanList.add(next)



print("youList: " + str(youList))
print("sanList: " + str(sanList))

print("youList - sanList: " + str(youList - sanList))
print("sanList - youList: " + str(sanList - youList))
#print(len(list(youList - sanList)) + len(list(sanList-youList)))
print(len(youList - sanList) + len(sanList - youList))

# 385
