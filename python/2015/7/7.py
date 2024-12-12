with open("7input.txt") as file:
    instructions = file.readlines()

wires = {}
for instruction in instructions:
    split = instruction.split()
    print(split)

    if split[0] == "NOT":
        if split[1] not in wires:
            wires[split[1]] = 0
        wires[split[3]] = ~wires[split[1]]
    elif split[1] == "->":
        if split[0].isnumeric():
            wires[split[2]] = int(split[0])
        else:
            if split[0] not in wires:
                wires[split[0]] = 0
            wires[split[2]] = wires[split[0]]
    elif split[1] == "AND":
        if split[0] not in wires:
            wires[split[0]] = 0
        if split[2] not in wires:
            wires[split[2]] = 0
        wires[split[4]] = wires[split[0]]&wires[split[2]]
    elif split[1] == "OR":
        if split[0] not in wires:
            wires[split[0]] = 0
        if split[2] not in wires:
            wires[split[2]] = 0
        wires[split[4]] = wires[split[0]]|wires[split[2]]
    elif split[1] == "LSHIFT":
        if split[0] not in wires:
            wires[split[0]] = 0
        wires[split[4]] = wires[split[0]]<<int(split[2])
    elif split[1] == "RSHIFT":
        if split[0] not in wires:
            wires[split[0]] = 0
        wires[split[4]] = wires[split[0]]>>int(split[2])



for key in wires.keys():
    if wires[key] < 0:
        wires[key] += 2**16

print(wires)

print(wires["a"])