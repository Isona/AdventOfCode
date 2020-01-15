total = 0
totals = set()
totals.add(total)
values = []

with open("1input.txt") as file:
    values = file.readlines()

print("Running!")
while True:
    for value in values:
        total += int(value)
        if total in totals:
            print(total)
            exit()
        else:
            totals.add(total)

print(total)