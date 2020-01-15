total = 0
with open("1input.txt") as file:
    for line in file:
        total += int(line)

print(total)