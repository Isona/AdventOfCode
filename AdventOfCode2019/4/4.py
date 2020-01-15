count = 0

for value in range(137683,596254):
    valueString = str(value)

    if "".join(sorted(valueString)) != valueString:
        continue

    for i in range(0, 5):
        if valueString.count(valueString[i]) >= 2:
            count += 1
            break

print(count)