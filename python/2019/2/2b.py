#with open("testinput.txt") as file:
with open("2input.txt") as file:
    originalList = file.readline().split(",")
    for i in range(len(originalList)):
        originalList[i] = int(originalList[i])




for noun in range(100):
    for verb in range(100):
        values = originalList.copy()

        values[1] = noun
        values[2] = verb

        currentIndex = 0

        while values[currentIndex] != 99:
            if values[currentIndex] == 1:
                values[values[currentIndex+3]] = values[values[currentIndex+1]] + values[values[currentIndex+2]]
                currentIndex += 4
            elif values[currentIndex] == 2:
                values[values[currentIndex+3]] = values[values[currentIndex+1]] * values[values[currentIndex+2]]
                currentIndex += 4
            else:
                print("Something went wrong: " + values[currentIndex])
                break


        result = values[0]
        if result == 19690720:
            print(str(result) + ": " + str(noun) + ", " + str(verb))
            print((100*noun+verb))