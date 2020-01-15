nodes = {}

def findGroup():
    prevLen = -1
    group = set()
    startNode, startNodeData = nodes.popitem()
    nodes[startNode] = startNodeData
    group.add(startNode)
    groupQueue = set()

    while True:
        for node in group:
            if node in nodes:
                for i in range(2, len(nodes[node])):
                    groupQueue.add(nodes[node][i])

        for item in groupQueue:
            group.add(item)
        groupQueue.clear()

        if prevLen == len(group):
            for item in group:
                if item in nodes:
                    del(nodes[item])
            return
        else:
            prevLen = len(group)

    #find and remove all elements in a group

with open("12input.txt", "r") as file:
    for line in file:
        node = line.rstrip().replace(",", "").split()
        nodes[node[0]] = node

    groupCount = 0
    while len(nodes) > 0:
        findGroup()
        groupCount+=1

    print(groupCount)