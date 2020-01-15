def isValid(wires, value):
    return(value.isnumeric() or value in wires.keys())

def getValue(wires, value):
    if value.isnumeric():
        return int(value)
    elif value in wires.keys():
        return wires[value]
    else:
        print("Error!: " + value)
        print(wires)


with open("7input.txt") as file:
    instructions = file.readlines()

wires = {}
wires["b"] = 3176
while len(instructions) > 0:
    deleteList = []

    for i in range(len(instructions)):
        split = instructions[i].split()
        print(split)

        if split[0] == "NOT":
            if isValid(wires, split[1]):
                wires[split[3]] = ~getValue(wires, split[1])
                deleteList.append(i)

        elif split[1] == "->":
            if split[2] == "b":
                deleteList.append(i)
            elif isValid(wires, split[0]):
                wires[split[2]] = getValue(wires, split[0])
                deleteList.append(i)

        elif split[1] == "AND":
            if isValid(wires,split[0]) and isValid(wires, split[2]):
                wires[split[4]] = getValue(wires, split[0])&getValue(wires, split[2])
                deleteList.append(i)

        elif split[1] == "OR":
            if isValid(wires,split[0]) and isValid(wires, split[2]):
                wires[split[4]] = getValue(wires, split[0])|getValue(wires, split[2])
                deleteList.append(i)

        elif split[1] == "LSHIFT":
            if isValid(wires,split[0]) and isValid(wires, split[2]):
                wires[split[4]] = getValue(wires, split[0])<<getValue(wires, split[2])
                deleteList.append(i)

        elif split[1] == "RSHIFT":
            if isValid(wires,split[0]) and isValid(wires, split[2]):
                wires[split[4]] = getValue(wires, split[0])>>getValue(wires, split[2])
                deleteList.append(i)

    for index in sorted(deleteList, reverse=True):
        del instructions[index]




for key in wires.keys():
    if wires[key] < 0:
        wires[key] += 2**16

print(wires)

print(wires["a"])