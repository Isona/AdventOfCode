from parse import *

particles = []

with open("20input.txt") as file:
    for line in file:
        r = parse("p=<{},{},{}>, v=<{},{},{}>, a=<{},{},{}>", line)
        # print(line.rstrip().split(", "))
        particles.append([[int(r[0]),int(r[1]),int(r[2])],[int(r[3]),int(r[4]),int(r[5])],[int(r[6]),int(r[7]),int(r[8])]])

for i in range(1000):
    for particle in particles:
        for i in range(3):
            particle[1][i] += particle[2][i]
            particle[0][i] += particle[1][i]


closestDist = 100000000000000000000
closest = -1
for i in range(len(particles)):
    dist = abs(particles[i][0][0]) + abs(particles[i][0][1]) + abs(particles[i][0][2])
    if dist < closestDist:
        closestDist = dist
        closest = i


print(closest)

