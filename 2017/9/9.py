with open("9input.txt", "r") as file:
    data = file.readline()
    ignoreNext = False
    currentlyGarbage = False
    garbageCount = 0
    currentLevel = 0
    score = 0

    for character in data:
        if not ignoreNext:
            if currentlyGarbage and character == "!":
                ignoreNext = True
            elif not currentlyGarbage and character == "<":
                currentlyGarbage = True
            elif character == ">":
                currentlyGarbage = False
            elif currentlyGarbage:
                garbageCount+=1
            elif not currentlyGarbage and character == "{":
                currentLevel+=1
            elif not currentlyGarbage and character == "}":
                score+=currentLevel    
                currentLevel-=1

        else:
            ignoreNext = False

    print(score)
    print(garbageCount)