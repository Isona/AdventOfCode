with open("12input.txt", "r") as file:
    nodes = {}
    zeroGroup = set("0")
    prevLen = -1
    zeroQueue = set()

    for line in file:
        node = line.rstrip().replace(",","").split()
        nodes[node[0]] = node

        for i in range(2, len(node)):
            if node[i] == "0":
                zeroGroup.add(node[0])

    while True:
        for node in zeroGroup:
            for i in range(2, len(nodes[node])):
                zeroQueue.add(nodes[node][i])

        for item in zeroQueue:
            zeroGroup.add(item)
        zeroQueue.clear()
        if prevLen == len(zeroGroup):
            print(zeroGroup)
            print(len(zeroGroup))
            exit()
        else:
            prevLen = len(zeroGroup)



