import string

values = []

with open("2input.txt") as file:
    values = file.readlines()

count3 = 0
count2 = 0

for value in values:
    found3 = False
    found2 = False
    for letter in string.ascii_lowercase:
        count = value.count(letter)
        if count == 3: found3 = True
        if count == 2: found2 = True
        if found3 and found2: break
    if found3: count3+=1
    if found2: count2+=1

print(count3)
print(count2)
print(count3*count2)