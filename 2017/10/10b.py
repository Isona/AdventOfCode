with open("10input.txt", "r") as file:
    data = file.readline().rstrip()
    lengths = [0]*len(data)
    for i in range(0, len(data)):
        lengths[i] = ord(data[i])

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
    denseString = ""

    for i in range(0, 16):
        for j in range(0, 16):
            denseHash[i] ^= knots[16*i + j]
        #denseString += format(denseHash[i], 'x')
        denseString += hex(denseHash[i])[2:] + ""

    print(denseHash)
    print(len(denseString))
    print(denseString)