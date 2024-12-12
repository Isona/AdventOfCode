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

while len(queue) > 0:
    base = queue.popleft()
    print("Base: " + base)
    orbiting[base][1] = orbiting[orbiting[base][0]][1]+1
    sum += orbiting[base][1]
    if base in orbited.keys():
        for planet in orbited[base]:
            print(planet)
            queue.append(planet)


print(orbiting)
print(sum)