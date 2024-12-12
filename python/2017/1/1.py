with open("1input.txt", "r") as file:
    input = file.read()
    total = 0
    previous = 0
    for i in input:
        if i == previous:
            total += int(i)
        previous = i

    if input[len(input)-1] == input[0]:
        total+= int(i[0])

    print(total)
