with open("4input.txt", "r") as file:
    total = 0
    for line in file:
        line.rstrip()
        words = line.split()
        wordSet = set()
        for word in words:
            wordSet.add(word)
        if len(wordSet) == len(words):
            total+=1

    print(total)