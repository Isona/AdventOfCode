grid = []
# with open("new19input.txt") as file:
with open("new19input.txt") as file:
#with open("19testinput.txt") as file:
    for line in file:
        grid.append(list(line.replace("\n", "")))


# It's grid[y][x]
currentY = 0
currentX = grid[0].index("|")
direction = "D"
encounteredLetters = ""
gridWidth = len(grid[0])
gridHeight = len(grid)
totalSteps = 0

while True:
    if direction == "D":
        currentY += 1
    elif direction == "U":
        currentY -= 1
    elif direction == "R":
        currentX += 1
    elif direction == "L":
        currentX -= 1
    print(grid[currentY][currentX])
    if grid[currentY][currentX] == " ":
        print("Ending because this is a space")
        break
    elif grid[currentY][currentX] == "+":
        print("This is a plus so I should turn")
        print(grid[currentY][currentX+1])
        if direction != "D" and currentY-1 >= 0 and grid[currentY-1][currentX] != " ":
            print("Turning up!")
            direction = "U"
        elif direction != "U" and currentY+1 < gridHeight and grid[currentY+1][currentX] != " ":
            print("Turning down!")
            direction = "D"
        elif direction != "L" and currentX+1 < gridWidth and grid[currentY][currentX+1] != " ":
            print("Turning right!")
            direction = "R"
        elif direction != "R" and currentX-1 >= 0 and grid[currentY][currentX-1] != " ":
            print("Turning left!")
            direction = "L"
        else:
            print("I didn't turn and I maybe should have?")
    elif grid[currentY][currentX].isalpha():
        print("Encountered a letter: " + grid[currentY][currentX])
        encounteredLetters += grid[currentY][currentX]

    totalSteps += 1

print(encounteredLetters)
print(totalSteps+1)
#print(grid)