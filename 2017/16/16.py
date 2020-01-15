with open("16input.txt", "r") as file:
    inputs = file.readline().rstrip().split(",")

    programs = ["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p"]

    for instruction in inputs:
        # print instruction       
        if instruction[0] == "s":
            spinLen = int(instruction[1::])
            spun = programs[-spinLen:]
            spun.extend(programs[:-spinLen])
            programs = spun
        elif instruction[0] == "x":
            values = instruction[1:].split("/")
            values[0] = int(values[0])
            values[1] = int(values[1])
            # print values
            temp = programs[values[0]]
            programs[values[0]] = programs[values[1]]
            programs[values[1]] = temp
        elif instruction[0] == "p":
            values = instruction[1:].split("/")
            index0 = programs.index(values[0])
            index1 = programs.index(values[1])
            temp = programs[index0]
            programs[index0] = programs[index1]
            programs[index1] = temp

        # print programs


    print("".join(programs))