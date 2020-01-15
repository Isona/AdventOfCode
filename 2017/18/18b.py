import string
#import Queue
import queue as Queue

bSent = 0

aRegisters = {}
bRegisters = {}
for character in string.ascii_lowercase:
    aRegisters[character] = 0
    bRegisters[character] = 0
bRegisters["p"] = 1

aQueue = Queue.Queue()
bQueue = Queue.Queue()

instructions = []

with open("18input.txt", "r") as file:
    for line in file:
        instructions.append(line.rstrip().split(" "))


aCurrent = 0
bCurrent = 0
aBlocking = False
bBlocking = False

while True:
    # A
    while True:
        if aCurrent >= 0 and aCurrent < len(instructions):
            i = instructions[aCurrent]
        else:
            aBlocking = True
            print("A went out of bounds with - " + str(aCurrent))
            if bBlocking:
                print(bSent)
                exit()
            break

        if i[0] == "snd":
            if i[1] not in string.ascii_lowercase:
                bQueue.put(i[1])
                print("Put " + str(i[1]) + " on B queue")
            else:
                bQueue.put(aRegisters[i[1]])
                print("Put " + str(aRegisters[i[1]]) + " on B queue")
            bBlocking = False
        elif i[0] == "set":
            if i[2] in string.ascii_lowercase:
                aRegisters[i[1]] = aRegisters[i[2]]
            else:
                aRegisters[i[1]] = int(i[2])
        elif i[0] == "add":
            if i[2] in string.ascii_lowercase:
                aRegisters[i[1]] += aRegisters[i[2]]
            else:
                aRegisters[i[1]] += int(i[2])
        elif i[0] == "mul":
            aRegisters[i[1]] *= int(i[2])
        elif i[0] == "mod":
            if i[2] in string.ascii_lowercase:
                aRegisters[i[1]] %= aRegisters[i[2]]
            else:
                aRegisters[i[1]] %= int(i[2])
        elif i[0] == "rcv":
            print("A RCV!")
            if aQueue.empty():
                aBlocking = True
                if bBlocking:
                    print(bSent)
                    exit()
                break
            else:
                aRegisters[i[1]] = aQueue.get()
                print("a received: " + str(aRegisters[i[1]]))

        if i[0] == "jgz":
            if i[1] in string.ascii_lowercase and aRegisters[i[1]] > 0:
                if i[2] in string.ascii_lowercase:
                    aCurrent += aRegisters[i[2]]
                else:
                    aCurrent += int(i[2])
            elif i[1] not in string.ascii_lowercase and int(i[1]) > 0:
                aCurrent += int(i[2])
            else:
                aCurrent += 1
        else:
            aCurrent += 1



    # B
    while True:
        if bCurrent >= 0 and bCurrent < len(instructions):
            i = instructions[bCurrent]
        else:
            bBlocking = True
            print("B went out of bounds with - " + str(bCurrent))
            if aBlocking:
                print(bSent)
                exit()
            break
            
        if i[0] == "snd":
            if i[1] not in string.ascii_lowercase:
                aQueue.put(i[1])
                print("Put " + str(i[1]) + " on A queue")
            else:
                aQueue.put(bRegisters[i[1]])
                print("Put " + str(bRegisters[i[1]]) + " on A queue")
            aBlocking = False
            bSent += 1
            print(bSent)
        elif i[0] == "set":
            if i[2] in string.ascii_lowercase:
                bRegisters[i[1]] = bRegisters[i[2]]
            else:
                bRegisters[i[1]] = int(i[2])
        elif i[0] == "add":
            if i[2] in string.ascii_lowercase:
                bRegisters[i[1]] += bRegisters[i[2]]
            else:
                bRegisters[i[1]] += int(i[2])
        elif i[0] == "mul":
            bRegisters[i[1]] *= int(i[2])
        elif i[0] == "mod":
            if i[2] in string.ascii_lowercase:
                bRegisters[i[1]] %= bRegisters[i[2]]
            else:
                bRegisters[i[1]] %= int(i[2])
        elif i[0] == "rcv":
            print("B RCV!")
            if bQueue.empty():
                bBlocking = True
                if aBlocking:
                    print(bSent)
                    exit()
                break
            else:
                bRegisters[i[1]] = bQueue.get()
                print("b received: " + str(bRegisters[i[1]]))

        if i[0] == "jgz":
            #print(i[1])
            if i[1] in string.ascii_lowercase and bRegisters[i[1]] > 0:
                if i[2] in string.ascii_lowercase:
                    bCurrent += bRegisters[i[2]]
                else:
                    bCurrent += int(i[2])
            elif i[1] not in string.ascii_lowercase and int(i[1]) > 0:
                bCurrent += int(i[2])
            else:
                bCurrent += 1
        else:
            bCurrent += 1