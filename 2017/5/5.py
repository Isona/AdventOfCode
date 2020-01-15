with open("5input.txt", "r") as file:
    regs = []
    for line in file:
        regs.append(int(line))

currentReg = 0
jumps = 0

while currentReg < len(regs) and currentReg >= 0:
    regs[currentReg] += 1
    currentReg += regs[currentReg] -1
    jumps += 1

print(jumps)