with open("1input.txt", "r") as file:
    input = file.read()
    total = 0
    for i in range(0, len(input)//2):
        if input[i] == input[(i+len(input)//2)]:
            total += int(input[i])*2

    print(total)