def loops(numberLine):
    for i in range(0, len(numberLine)):
        for j in range(0, len(numberLine)):
            if i!=j and numberLine[i] % numberLine[j] == 0:
                return(numberLine[i]/numberLine[j])

with open("2input.txt", "r") as file:
    total = 0
    for line in file:
        line.rstrip()
        numberLine = [int(n) for n in line.split()]
        total += loops(numberLine)
    print(total)