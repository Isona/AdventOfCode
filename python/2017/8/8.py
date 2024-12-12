with open("8input.txt", "r") as file:
    registers = {}
    maxRegister = 0
    for line in file:
        instruction = line.rstrip().split()
        if not instruction[0] in registers:
            registers[instruction[0]] = 0

        value = int(instruction[2])
        if instruction[1] == "dec":
            value = -value

        if not instruction[4] in registers:
            registers[instruction[4]] = 0

        if instruction[5] == ">" and registers[instruction[4]] > int(instruction[6]):
            registers[instruction[0]] += value
        elif instruction[5] == "<" and registers[instruction[4]] < int(instruction[6]):
            registers[instruction[0]] += value
        elif instruction[5] == ">=" and registers[instruction[4]] >= int(instruction[6]):
            registers[instruction[0]] += value
        elif instruction[5] == "<=" and registers[instruction[4]] <= int(instruction[6]):
            registers[instruction[0]] += value
        elif instruction[5] == "==" and registers[instruction[4]] == int(instruction[6]):
            registers[instruction[0]] += value
        elif instruction[5] == "!=" and registers[instruction[4]] != int(instruction[6]):
            registers[instruction[0]] += value

        if registers[instruction[0]] > maxRegister:
            maxRegister = registers[instruction[0]]

    print(max(registers.values()))
    print(maxRegister)