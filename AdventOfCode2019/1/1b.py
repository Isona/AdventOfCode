def getFuel(i):
    return (i//3 - 2) 

totalFuel = 0
with open("1input.txt") as file:
#with open("testinput.txt") as file:
    for line in file:
        j = (int(line)//3)-2
        while j > 0:
            totalFuel += j
            j = getFuel(j)

# j = getFuel(totalFuel)
# #totalFuel += j

# while j > 0:
#     totalFuel += j
#     j = getFuel(j)

print(totalFuel)