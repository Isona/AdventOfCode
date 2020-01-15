with open("2input.txt", "r") as file:
    total = 0
    for line in file:
        line.rstrip()
        numberLine = [int(n) for n in line.split()]
        total += max(numberLine) - min(numberLine)

    print(total)