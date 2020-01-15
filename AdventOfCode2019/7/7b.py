from itertools import permutations 

filename = "7input.txt"
#programInput = 5

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


def doIntCode(intCode, currentIndex, programInput):
    opcode = -1
    parameters = []
    lastOutput = 0

    while intCode[currentIndex] != 99:
        (opcode, parameters) = getValues(intCode, currentIndex)

        # Add
        if opcode == 1:
            intCode[parameters[2]] = parameters[0] + parameters[1]
            currentIndex += 4

        # Multiply
        elif opcode == 2:
            intCode[parameters[2]] = parameters[0] * parameters[1]
            currentIndex += 4

        # Take input
        elif opcode == 3:
            #print(programInput)
            intCode[parameters[0]] = programInput.pop(0)
            currentIndex += 2

        # Print output
        elif opcode == 4:
            lastOutput = intCode[parameters[0]]
            #print(values[parameters[0]])
            currentIndex += 2
            return(intCode, currentIndex, lastOutput)

        elif opcode == 5:
            #print(opcode, parameters, intCode[currentIndex:currentIndex+10])
            if parameters[0] != 0:
                currentIndex = parameters[1]
            else:
                currentIndex += 3


        elif opcode == 6:
            #print(opcode, parameters, intCode[currentIndex:currentIndex+10])
            if parameters[0] == 0:
                currentIndex = parameters[1]
            else:
                currentIndex += 3

        elif opcode == 7:
            #print(opcode, parameters, intCode[currentIndex:currentIndex+10])
            if parameters[0] < parameters[1]:
                intCode[parameters[2]] = 1
            else:
                intCode[parameters[2]] = 0
            currentIndex += 4
        elif opcode == 8:
            #print(opcode, parameters, intCode[currentIndex:currentIndex+10])
            if parameters[0] == parameters[1]:
                intCode[parameters[2]] = 1
            else:
                intCode[parameters[2]] = 0
            currentIndex += 4
        else:
            print("Something went wrong: " + intCode[currentIndex])
            break
    return(intCode, currentIndex, 0)
    #return(lastOutput)


with open(filename) as file:
    baseIntcode = file.readline().split(",")
    for i in range(len(baseIntcode)):
        baseIntcode[i] = int(baseIntcode[i])

#print(doIntCode([0,0]))
perms = list(permutations(range(0, 5)))
perms2 = list(permutations(range(6, 10)))
#prevOutput = 0

greatestOutput = 0
greatestPermutation = ""

for permutation in perms:
    prevOutput = 0
    generatorCodes = [""]*5
    generatorIndexes = [0]*5
    for ampIndex in range(len(permutation)):
        generatorCodes[ampIndex], generatorIndexes[ampIndex], prevOutput = doIntCode(list(baseIntcode), 0, [permutation[ampIndex], prevOutput])

    firstStageOutput = prevOutput
    print(firstStageOutput)

    for permutation2 in perms2:
        prevOutput = firstStageOutput
        for ampIndex in range(len(permutation2)):
            currentInput = [permutation2[ampIndex], prevOutput]
            currentIntcode = list(generatorCodes[ampIndex])
            currentIndex = generatorIndexes[ampIndex]
            (_, _, prevOutput) = doIntCode(currentIntcode, currentIndex, currentInput)

        if prevOutput > greatestOutput:
            greatestOutput = prevOutput
            greatestPermutation = str(permutation) + str(permutation2)


print("Greatest Output: " + str(greatestOutput))
print("Greatest Permutation: " + greatestPermutation)