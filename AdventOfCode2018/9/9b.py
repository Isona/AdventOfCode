from parse import *
import collections

players = 0
lastMarble = 0

with open("9input.txt") as file:
    players, lastMarble = parse("{:d} players; last marble is worth {:d} points", file.readline())

lastMarble*=100

playerScores = [0]*players
currentPlayer = 0
marbles = collections.deque([0])

#for i in range(25):
for currentValue in range(1, lastMarble+1):
    if currentValue % 23 == 0:
        currentIndex = marbles.rotate(7)
        playerScores[currentPlayer] += currentValue + marbles.pop()
        marbles.rotate(-1)
    else:
        marbles.rotate(-1)
        marbles.append(currentValue)
        previousScore = currentValue

    currentValue += 1
    currentPlayer = (currentPlayer + 1) % players
    #print(marbles)
    #print(marbles[currentIndex])

playerScores.sort()
print(playerScores[players-1])