import string

registers = {}
for character in string.ascii_lowercase:
    registers[character] = 0

instructions = []

with open("18input.txt", "r") as file:
    for line in file:
        instructions.append(line.rstrip().split(" "))


current = 0
lastSound = 0

while True:
    i = instructions[current]
    if i[0] == "snd":
        lastSound = registers[i[1]]
    elif i[0] == "set":
        if i[2] in string.ascii_lowercase:
            registers[i[1]] = registers[i[2]]
        else:
            registers[i[1]] = int(i[2])
    elif i[0] == "add":
        registers[i[1]] += int(i[2])
    elif i[0] == "mul":
        registers[i[1]] *= int(i[2])
    elif i[0] == "mod":
        if i[2] in string.ascii_lowercase:
            registers[i[1]] %= registers[i[2]]
        else:
            registers[i[1]] %= int(i[2])
    elif i[0] == "rcv" and registers[i[1]] != 0:
        print(lastSound)
        break

    if i[0] == "jgz" and registers[i[1]] > 0:
        current += int(i[2])
    else:
        current += 1

print(registers)