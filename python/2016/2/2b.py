with open("2input.txt") as file:
    directions = file.readlines()
keypad = [["x","x","5","x","x"],
          ["x","2","6","A","x"],
          ["1","3","7","B","D"],
          ["x","4","8","C","x"],
          ["x","x","9","x","x"]]
locationX = 0
locationY = 2
code = ""

for direction in directions:
    direction = direction.rstrip()
    for i in range(len(direction)):
        if direction[i] == "L":
            if locationX > 0 and keypad[locationX-1][locationY] != "x":
                locationX -= 1
        elif direction[i] == "R":
            if locationX < 4 and keypad[locationX+1][locationY] != "x":
                locationX += 1
        elif direction[i] == "D":
            if locationY < 4 and keypad[locationX][locationY+1] != "x":
                locationY += 1
        elif direction[i] == "U":
            if locationY > 0 and keypad[locationX][locationY-1] != "x":
                locationY -= 1
    
    print(keypad[locationX][locationY], end="")
    #print(str(locationX) +  str(locationY))