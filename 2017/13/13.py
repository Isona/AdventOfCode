scanners = {}
currentPosition = -1
score = 0

with open("13input.txt", "r") as file:
    for line in file:
        data = line.replace(":", "").rstrip().split()
        scanners[int(data[0])] = [int(data[1]), 1, 1]


while currentPosition <= max(scanners.keys()):
    currentPosition += 1
    if currentPosition in scanners:
        if scanners[currentPosition][1] == 1:
            score += currentPosition * scanners[currentPosition][0]

    for key, scanner in scanners.items():
        scanner[1] += scanner[2]

        if scanner[1] == scanner[0] or scanner[1] == 1:
            scanner[2] = -scanner[2]

print(score)