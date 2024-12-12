import string
import difflib

def hamming(a, b):
    distance = 0
    for i in range(0, len(a)):
        if a[i] != b[i]:
            distance += 1
    return(distance)


values = []

with open("2input.txt") as file:
    values = file.readlines()

for value in values:
    for comp in values:
        if comp == value:
            continue
        distance = hamming(value, comp)
        if distance == 1:
            print(str(distance) + ": " + value + " " + comp)