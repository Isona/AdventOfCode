with open("1input.txt") as file:
    input = file.readline()
    print(input.count("(") - input.count(")"))