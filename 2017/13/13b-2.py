scanners = {}
caught = False
delay = 0

with open("13input.txt", "r") as file:
    for line in file:
        data = line.replace(":", "").rstrip().split()
        scanners[int(data[0])] = int(data[1])

while True:
    caught = False

    
    for scanner, value in scanners.items():
        if (scanner+delay)%(value*2-2) == 0:
            #print "Caught by: " + str(scanner) + " delay: " + str(delay)
            caught = True
            break

    if not caught:
        print(delay)
        exit()
    else:
        delay += 1
