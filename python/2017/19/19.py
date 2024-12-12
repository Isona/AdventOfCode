import string

inputGrid = ""
with open("19input.txt", "r") as file:
    for line in file:
        inputGrid += line.replace("\n", "")

for current in range(0, 202):
    if inputGrid[current] == "|":
        break

encountered = ""
currentDirection = "down"
stepCount = 1

while inputGrid[current] != "B":
    stepCount += 1
    #print current, inputGrid[current]
    if currentDirection == "down":
        current += 201
    elif currentDirection == "up":
        current -= 201
    elif currentDirection == "right":
        current += 1
    else: # left
        current -= 1

    if inputGrid[current] == "+":
        if currentDirection != "down" and inputGrid[current-201] == "|":
            currentDirection = "up"
        elif currentDirection != "up" and inputGrid[current + 201] == "|":
            currentDirection = "down"
        elif currentDirection != "left" and inputGrid[current+1] == "-":
            currentDirection = "right"
        else:
            currentDirection = "left"

    elif inputGrid[current] in string.ascii_uppercase:
        encountered += inputGrid[current]

    elif inputGrid[current] == " ":
        print("Something has gone wrong!")
        print(current, inputGrid[current], currentDirection)
        break

print(encountered)
print(stepCount)