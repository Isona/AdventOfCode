rules = {}
currentArt = ".#./..#/###"

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
    global rules
    pattern = rotate(inputPattern)
    pattern = rotate(pattern)
    pattern = rotate(pattern)
    pattern = flip(rotate(pattern))
    pattern = rotate(pattern)
    pattern = rotate(pattern)
    pattern = rotate(pattern)
    for i in range(0, 4):
        pattern = rotate(pattern)
        if pattern in rules:
            return rules[pattern]
    pattern = flip(pattern)
    for i in range(0, 4):
        pattern = rotate(pattern)
        if pattern in rules:
            return rules[pattern]

    print("Something went wrong! There was no match!")
    exit()


with open("21input.txt", "r") as file:
    for line in file:
        rule = line.split(" ")
        rules[rule[0]] = rule[2]

