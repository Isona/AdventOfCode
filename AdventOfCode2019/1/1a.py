sum = 0
with open("1input.txt") as file:
    for line in file:
        sum += (int(line)//3)-2
print(sum)



#print(14//3 - 2)