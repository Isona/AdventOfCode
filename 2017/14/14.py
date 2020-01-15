input = "jxqlasbh"

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

def binaryCount(inputString):
    oneCount = 0
    for character in inputString:
        oneCount += (bin(character)).count("1")

    return oneCount

squares = 0
for n in range(0, 128):
    currentString = input + "-" + str(n)
    print(currentString)
    squares += binaryCount(knottyHash(currentString))

print(squares)