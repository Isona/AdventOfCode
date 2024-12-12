with open("8input.txt") as file:
    lines = file.readlines()

memLength = 0
stringLength = 0

for line in lines:
    line = line.rstrip()
    memLength += len(line)
    stringLength += len(bytes(line, "utf-8").decode("unicode_escape"))-2

print(memLength)
print(stringLength)
print(memLength - stringLength)