with open("8input.txt") as file:
    lines = file.readlines()

memLength = 0
stringLength = 0

for line in lines:
    line = line.rstrip()
    memLength += len(line)
    substring = line[1:-1]
    print(substring)
    currStringLength = len(substring)

    currStringLength -= substring.count("\\\\")
    currStringLength -= substring.count("\\\"")
    currStringLength -= 3*(substring.count("\\x")-substring.count("\\\\x"))

    print(line)
    print(len(line))
    print(currStringLength)

    stringLength += currStringLength


print(memLength)
print(stringLength)

print(memLength - stringLength)