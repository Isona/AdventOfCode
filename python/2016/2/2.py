with open("2input.txt") as file:
    directions = file.readlines()
keypad = [[1, 4, 7],[2,5,8],[3,6,9]]
locationX = 1
locationY = 1
code = ""

for direction in directions:
    direction = direction.rstrip()
    for i in range(len(direction)):
        if direction[i] == "L":
            if locationX > 0:
                locationX -= 1
        elif direction[i] == "R":
            if locationX < 2:
                locationX += 1
        elif direction[i] == "D":
            if locationY < 2:
                locationY += 1
        elif direction[i] == "U":
            if locationY > 0:
                locationY -= 1
    #print(str(locationX) +  str(locationY))
    print(keypad[locationX][locationY])