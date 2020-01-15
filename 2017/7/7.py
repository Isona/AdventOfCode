with open("7input.txt", "r") as file:
    parents = set()
    children = set()
    for line in file:
        things = line.rstrip().split()
        parents.add(things[0])
        count = 3
        while count < len(things):
            children.add(things[count].rstrip(','))
            count += 1
            
    print(set.difference(parents, children))
