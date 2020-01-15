from itertools import permutations 

#filename = "9testinput.txt"
filename = "david9input.txt"
#programInput = 5

def getValues(values, index, relativeAddress):
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
            elif opcode[2-i] == "2":
                parameters.append(values[values[index+i+1]+relativeAddress])

        if opcode[0] == "0" or opcode[0] == "1":
            parameters.append(values[index+3])
        elif opcode[0] == "2":
            parameters.append(values[index+3]+relativeAddress)
        
        return(operation, parameters)

    elif operation == 3:
        if opcode[2] == "0" or opcode[2] == "1":
            return(operation, [values[index+1]])
        elif opcode[2] == "2":
            return(operation, [values[index+1]+relativeAddress])

    elif operation == 4 or operation == 9:
        if opcode[2] == "0":
            return(operation, [values[values[index+1]]])
        elif opcode[2] == "1":
            return(operation, [values[index+1]])
        elif opcode[2] == "2":
            return(operation, [values[values[index+1]+relativeAddress]])

    elif operation == 5 or operation == 6:
        parameters = []
        for i in range(0, 2):
            if opcode[2-i] == "0":
                parameters.append(values[values[index+i+1]]) 
            elif opcode[2-i] == "1":
                parameters.append(values[index+i+1])
            elif opcode[2-i] == "2":
                parameters.append(values[values[index+i+1]+relativeAddress]) 

        return(operation, parameters)
    else:
        print("Something went wrong: " + str(values[index]))
        exit()


def doIntCode(programInput):
    with open(filename) as file:
        values = file.readline().split(",")
        for i in range(len(values)):
            values[i] = int(values[i])

    values.extend([0]*10000)

    currentIndex = 0
    relativeAddress = 0

    opcode = -1
    parameters = []
    lastOutput = 0

    while values[currentIndex] != 99:
        #print("Relative addresss: " + str(relativeAddress))
        (opcode, parameters) = getValues(values, currentIndex, relativeAddress)

        # Add
        if opcode == 1:
            #print(len(values))
            #print(parameters[1])
            values[parameters[2]] = parameters[0] + parameters[1]
            currentIndex += 4

        # Multiply
        elif opcode == 2:
            values[parameters[2]] = parameters[0] * parameters[1]
            currentIndex += 4

        # Take input
        elif opcode == 3:
            #print(programInput)
            values[parameters[0]] = programInput.pop(0)
            currentIndex += 2

        # Print output
        elif opcode == 4:
            lastOutput = parameters[0]
            print(parameters[0])
            currentIndex += 2

        elif opcode == 5:
            #print(opcode, parameters, values[currentIndex:currentIndex+10])
            if parameters[0] != 0:
                currentIndex = parameters[1]
            else:
                currentIndex += 3


        elif opcode == 6:
            #print(opcode, parameters, values[currentIndex:currentIndex+10])
            if parameters[0] == 0:
                currentIndex = parameters[1]
            else:
                currentIndex += 3

        elif opcode == 7:
            #print(opcode, parameters, values[currentIndex:currentIndex+10])
            if parameters[0] < parameters[1]:
                values[parameters[2]] = 1
            else:
                values[parameters[2]] = 0
            currentIndex += 4

        elif opcode == 8:
            #print(opcode, parameters, values[currentIndex:currentIndex+10])
            if parameters[0] == parameters[1]:
                values[parameters[2]] = 1
            else:
                values[parameters[2]] = 0
            currentIndex += 4

        elif opcode == 9:
            #print(opcode, parameters, values[currentIndex:currentIndex+10])
            relativeAddress += parameters[0]
            currentIndex += 2
        else:
            print("Something went wrong: " + values[currentIndex])
            break
    return(lastOutput)


#print(doIntCode([0,0]))
#prevOutput = 0

prevOutput = doIntCode([1])
print("Output: " + str(prevOutput))