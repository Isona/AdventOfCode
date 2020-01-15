import string

registers = {}
for character in string.ascii_lowercase[0:8]:
    registers[character] = 0

print(registers)
instructions = []

with open("23ainput.txt", "r") as file:
    for line in file:
        instructions.append(line.rstrip().split(" "))

mulCount = 0
current = 0

while True:
    if current < 0 or current >= len(instructions):
        break

    i = instructions[current]
    if i[0] == "set":
        if i[2] in string.ascii_lowercase:
            registers[i[1]] = registers[i[2]]
        else:
            registers[i[1]] = int(i[2])
    elif i[0] == "sub":
        if i[2] in string.ascii_lowercase:
            registers[i[1]] -= registers[i[2]]
        else:
            registers[i[1]] -= int(i[2])
    elif i[0] == "mul":
        mulCount += 1
        if i[2] in string.ascii_lowercase:
            registers[i[1]] *= registers[i[2]]
        else:
            registers[i[1]] *= int(i[2])

    if i[0] == "jnz":
        jump = False
        if i[1] in string.ascii_lowercase:
            jump = registers[i[1]] != 0
        else: jump = i[1] != 0

        if jump:
            if i[2] in string.ascii_lowercase:
                current += registers[i[2]]
            else:
                current += int(i[2])
        else:
            current += 1

    else:
        current += 1

print(registers)
print(mulCount)