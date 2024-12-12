with open("5input.txt", "r") as file:
    regs = []
    for line in file:
        regs.append(int(line))

currentReg = 0
jumps = 0

while currentReg < len(regs) and currentReg >= 0:
    origReg = regs[currentReg]
    regs[currentReg] += -1 if regs[currentReg] >= 3 else  1
    currentReg += origReg
    jumps += 1

print(jumps)