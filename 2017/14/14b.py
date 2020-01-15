input = "jxqlasbh"
hashGrid = []

def knottyHash(toHash):
    lengths = [0]*len(toHash)
    for i in range(0, len(toHash)):
        lengths[i] = ord(toHash[i])

    lengths.extend([17, 31, 73, 47, 23])

    knots = list(range(0,256))
    skipSize = 0
    currentItem = 0

    for n in range(0, 64):
        for length in lengths:
            reversedSection = [0]*length
            count = 0
            for i in range(currentItem, currentItem+length):
                reversedSection[count] = knots[i%len(knots)]
                count+=1

            reversedSection = reversedSection[::-1]

            count = 0
            for i in range(currentItem, currentItem + length):
                knots[i%len(knots)] = reversedSection[count]
                count += 1
            currentItem = (currentItem + length + skipSize)%len(knots)
            skipSize += 1

    denseHash = [0]*16

    for i in range(0, 16):
        for j in range(0, 16):
            denseHash[i] ^= knots[16*i + j]

    return(denseHash)

def gridAppend(inputString):
    #print inputString
    binaryString = ''.join('{0:08b}'.format(x, 'b') for x in inputString)
    #print binaryString

    for binary in binaryString:
        if binary == "0":
            hashGrid.append(False)
        elif binary == "1":
            hashGrid.append(True)

def findNeighbours(coord):
    hashGrid[coord] = False

    neighbours = []
    if coord - 128 >= 0 and hashGrid[coord - 128]:
        neighbours.append(coord - 128)

    if coord + 128 < len(hashGrid) and hashGrid[coord + 128]:
        neighbours.append(coord + 128)

    if coord - 1 >= 0 and ((coord - 1) // 128) == (coord // 128) and hashGrid[coord - 1]:
        neighbours.append(coord - 1)

    if coord + 1 < len(hashGrid) and ((coord + 1) // 128) == (coord // 128) and hashGrid[coord + 1]:
        neighbours.append(coord + 1)

    for loc in neighbours:
        findNeighbours(loc)


for n in range(0, 128):
    currentString = input + "-" + str(n)
    gridAppend(knottyHash(currentString))

regions = 0

for n in range(0, 128*128):
    if hashGrid[n]:
        findNeighbours(n)
        regions += 1

print("There are " + str(regions) + " regions")