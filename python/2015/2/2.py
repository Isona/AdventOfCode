with open("2input.txt") as file:
    dimensions = file.readlines()

totalArea = 0

for dimension in dimensions:
    split = dimension.split("x")
    sideA = 2 * int(split[0]) * int(split[1])
    sideB = 2 * int(split[1]) * int(split[2])
    sideC = 2 * int(split[0]) * int(split[2])
    area = sideA + sideB + sideC + min(sideA,sideB,sideC)//2
    totalArea += area

print(totalArea)