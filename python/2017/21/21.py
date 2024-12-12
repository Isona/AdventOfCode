rules = {}
currentArt = [".#./..#/###"]

def flip(input):
    matrix = []

    if len(input) == 5:
        matrix = [1, 0, 2, 4, 3]
    else:
        matrix = [2, 1, 0, 3, 6, 5, 4, 7, 10, 9, 8]

    return "".join([input[i] for i in matrix])

def rotate(input):
    matrix = []

    if len(input) == 5:
        matrix = [3,0,2,4,1]
    else:
        matrix = [8, 4, 0, 3, 9, 5, 1, 7, 10, 6, 2]

    return "".join([input[i] for i in matrix])

def findMatch(pattern):
    #print(pattern)
    global rules
    for i in range(0, 4):
        pattern = rotate(pattern)
        #print(pattern)
        if pattern in rules:
            return rules[pattern]
    pattern = flip(pattern)
    for i in range(0, 4):
        pattern = rotate(pattern)
        if pattern in rules:
            return rules[pattern]

    print("Something went wrong! There was no match!")
    exit()

def split(pattern):
    #print(len(pattern))
    if len(pattern) == 19:
        pattern1 = pattern[0]+pattern[1]+"/"+pattern[5]+pattern[6]
        pattern2 = pattern[2]+pattern[3]+"/"+pattern[7]+pattern[8]
        pattern3 = pattern[10]+pattern[11]+"/"+pattern[15]+pattern[16]
        pattern4 = pattern[12]+pattern[13]+"/"+pattern[17]+pattern[18]
        return([pattern1, pattern2, pattern3, pattern4])

with open("21input.txt", "r") as file:
    for line in file:
        rule = line.rstrip().split(" ")
        rules[rule[0]] = rule[2]

print(currentArt)
for round in range(5):
    newArt = []
    for art in currentArt:
        newPattern = findMatch(art)
        if len(newPattern) == 19:
            newArt.extend(split(newPattern))
        else:
            newArt.append(newPattern)
    currentArt = newArt
    print(len(currentArt))
    print(currentArt)
hashCount = 0
for pattern in currentArt:
    hashCount += pattern.count("#")
print(hashCount)

# Almost works but on the 2nd round it goes to a 4x3x3 grid which is currently computed as 4x3x3s but should actually by 9x2x2

# 143 too low