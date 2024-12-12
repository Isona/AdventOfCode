filename = "5input.txt"
programInput = 5

def getValues(values, index):
    opcode = str(values[index]).zfill(5)
    operation = int(opcode[-2]+opcode[-1])

    if operation == 99:
        return (99, [])
    elif operation == 1 or operation == 2 or operation == 7 or operation == 8:
        parameters = []
        for i in range(0, 2):
            if opcode[2-i] == "0":
                parameters.append(values[values[index+i+1]]) 
            elif opcode[2-i] == "1":
                parameters.append(values[index+i+1])
        parameters.append(values[index+3])
        return(operation, parameters)

    elif operation == 3 or operation == 4:
        return(operation, [values[index+1]])

    elif operation == 5 or operation == 6:
        parameters = []
        for i in range(0, 2):
            if opcode[2-i] == "0":
                parameters.append(values[values[index+i+1]]) 
            elif opcode[2-i] == "1":
                parameters.append(values[index+i+1])

        return(operation, parameters)
    else:
        print("Something went wrong: " + values[currentIndex])
        exit()



with open(filename) as file:
    values = file.readline().split(",")
    for i in range(len(values)):
        values[i] = int(values[i])

currentIndex = 0

opcode = -1
parameters = []

while values[currentIndex] != 99:
    (opcode, parameters) = getValues(values, currentIndex)

    # Add
    if opcode == 1:
        values[parameters[2]] = parameters[0] + parameters[1]
        currentIndex += 4

    # Multiply
    elif opcode == 2:
        values[parameters[2]] = parameters[0] * parameters[1]
        currentIndex += 4

    # Take input
    elif opcode == 3:
        values[parameters[0]] = programInput
        currentIndex += 2

    # Print output
    elif opcode == 4:
        print(values[parameters[0]])
        currentIndex += 2


    elif opcode == 5:
        print(opcode, parameters, values[currentIndex:currentIndex+10])
        if parameters[0] != 0:
            currentIndex = parameters[1]
        else:
            currentIndex += 3


    elif opcode == 6:
        print(opcode, parameters, values[currentIndex:currentIndex+10])
        if parameters[0] == 0:
            currentIndex = parameters[1]
        else:
            currentIndex += 3

    elif opcode == 7:
        print(opcode, parameters, values[currentIndex:currentIndex+10])
        if parameters[0] < parameters[1]:
            values[parameters[2]] = 1
        else:
            values[parameters[2]] = 0
        currentIndex += 4
    elif opcode == 8:
        print(opcode, parameters, values[currentIndex:currentIndex+10])
        if parameters[0] == parameters[1]:
            values[parameters[2]] = 1
        else:
            values[parameters[2]] = 0
        currentIndex += 4
    else:
        print("Something went wrong: " + values[currentIndex])
        break


#print(values)