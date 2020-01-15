with open("10input.txt", "r") as file:
    data = file.readline().rstrip().split(",")
    for i in range(0, len(data)):
        data[i] = int(data[i])
    knots = list(range(0,256))
    skipSize = 0
    currentItem = 0

    print(data)

    for length in data:
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
        print(knots)

    print(knots)
    print(knots[0]*knots[1])