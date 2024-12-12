def ringValue(ring):
    return((ring*2+1)**2)

input = 347991

ring = 0
while ringValue(ring) <= input:
    ring+=1

wholeThing = ringValue(ring)
centre = ringValue(ring-1)
perimeter = wholeThing - centre

centrePoints = [0]*4;
centrePoints[0] = centre + perimeter/8
for i in range(1, 4):
    centrePoints[i] = centrePoints[i-1] + perimeter/4

potato = [0]*4
for i in range(0, 4):
    potato[i] = abs(centrePoints[i]-input)

print(min(potato)+ring)