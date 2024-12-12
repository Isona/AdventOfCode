from parse import *

particles = []

with open("20input.txt") as file:
    for line in file:
        #print(line)
        r = parse("p=<{},{},{}>, v=<{},{},{}>, a=<{},{},{}>", line)
        #print(r)
        #print(line.rstrip().split(", "))
        particles.append([[int(r[0]),int(r[1]),int(r[2])],[int(r[3]),int(r[4]),int(r[5])],[int(r[6]),int(r[7]),int(r[8])],1])

for i in range(1000):
    positions = {}
    for particleIndex in range(len(particles)):
        particle = particles[particleIndex]
        if particle[3] == 0:
            continue
        for i in range(3):
            particle[1][i] += particle[2][i]
            particle[0][i] += particle[1][i]
            #print(str(particle[0]))
        if str(particle[0]) in positions.keys():
            particle[3] = 0
            particles[positions[str(particle[0])]][3] = 0
            print("Particle " + str(particleIndex) + " and particle " + str(positions[str(particle[0])]) + " collided")
        else:
            positions[str(particle[0])] = particleIndex
    #print(positions)


particleCount = 0
for particle in particles:
    if particle[3] == 1:
        particleCount += 1

print(particleCount)