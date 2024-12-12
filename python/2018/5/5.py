with open("5input.txt") as file:
    values = file.readline().strip()

charArray = list(values)

changed = True

while changed:
    changed = False
    i = 0
    while i < len(charArray)-1:
        if i+1 >= len(charArray):
            break
        if charArray[i].isupper():
            if charArray[i+1].islower() and charArray[i].lower() == charArray[i+1]:
                del charArray[i+1]
                del charArray[i]
                changed = True
            else:
                i+=1
        else:
            if charArray[i+1].isupper() and charArray[i].upper() == charArray[i+1]:
                del charArray[i+1]
                del charArray[i]
                changed = True
            else:
                i+=1


print("".join(charArray))
print(len(charArray))