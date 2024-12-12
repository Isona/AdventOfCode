import string

with open("5input.txt") as file:
    values = file.readline().strip()

originalString = list(values)
minLen = {}

changed = True
while changed:
    changed = False
    i = 0
    while i < len(originalString)-1:
        if i+1 >= len(originalString):
            break
        if originalString[i].isupper():
            if originalString[i+1].islower() and originalString[i].lower() == originalString[i+1]:
                del originalString[i+1]
                del originalString[i]
                changed = True
            else:
                i+=1
        else:
            if originalString[i+1].isupper() and originalString[i].upper() == originalString[i+1]:
                del originalString[i+1]
                del originalString[i]
                changed = True
            else:
                i+=1

for letter in string.ascii_lowercase:
    charArray = [x for x in originalString if x != letter and x!= letter.upper()]
    #print("".join(charArray))
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
    print(letter + " " + str(len(charArray)))
    minLen[letter] = len(charArray)

print(str(minLen))