with open("8testinput.txt") as file:
    lines = file.readlines()

memLength = 0
stringLength = 0

for line in lines:
    line = line.rstrip()
    memLength += len(line)
    print(len(line))
    print(len(line.encode("unicode_escape")))

print(memLength)
print(stringLength)
print(memLength - stringLength)