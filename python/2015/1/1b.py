with open("1input.txt") as file:
    input = file.readline()
    
currentFloor = 0
for i in range(len(input)):
    if input[i] == "(":
        currentFloor += 1
    elif input[i] == ")":
        currentFloor -= 1

    if currentFloor == -1:
        print(i+1)
        break