from parse import compile
import string

p = compile("Step {} must be finished before step {} can begin.")

dependencies = {}
for letter in list(string.ascii_uppercase):
    dependencies[letter] = []

with open("7input.txt") as file:
    for line in file:
        one, two = p.parse(line.rstrip())
        dependencies[two].append(one)

available = []
done = []
while len(dependencies) > 0:
    for letter in dependencies:
        canDo = True
        if letter not in done and letter not in available:
            for dependency in dependencies[letter]:
                if dependency not in done:
                    canDo = False
                    break
            if canDo and letter not in available:
                available.append(letter)

    available.sort()
    next = available.pop(0)
    done.append(next)
    print("".join(done))
    del dependencies[next]
