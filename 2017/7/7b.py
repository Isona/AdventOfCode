with open("7input.txt", "r") as file:
    calculated = {}
    uncalculated = {}

    for line in file:
        things = line.rstrip().split()
        if len(things) == 2:
            calculated[things[0]] = int(things[1][1:-1])
        else:
            for i in range(3,len(things)):
                things[i] = things[i].rstrip(',')
            uncalculated[things[0]] = things

    print(calculated)
    print(uncalculated)
    while len(uncalculated) > 0:
        deletedNodes = {}
        for key, node in uncalculated.items():
            print("Looking at "  + node[1])
            childrenCalculated = True
            for i in range(3, len(node)):
                if not node[i] in calculated:
                    childrenCalculated = False
            if childrenCalculated:
                compValue = calculated[node[3]]
                childrenValue = compValue
                for i in range(4, len(node)):
                    if compValue == calculated[node[i]]:
                        childrenValue += compValue
                    else:
                        for j in range(3, len(node)):
                            print(node[j], calculated[node[j]])
                        exit()

                calculated[key] = int(node[1][1:-1]) + childrenValue
                deletedNodes[key] = 0
                print("dealt with " + node[0])

        for key in deletedNodes:
            del(uncalculated[key])
            # If all children are in calculated
                # if all children are equal
                    # add children together with value of this node
                    # put in calculated
                    # delete from uncalculated
                # else print info on this imbalance and exit()
