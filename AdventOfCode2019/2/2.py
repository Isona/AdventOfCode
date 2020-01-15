#with open("testinput.txt") as file:
with open("2input.txt") as file:
    values = file.readline().split(",")
    for i in range(len(values)):
        values[i] = int(values[i])

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


print(values)