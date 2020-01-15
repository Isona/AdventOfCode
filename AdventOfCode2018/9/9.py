from parse import *

players = 0
lastMarble = 0

with open("9input.txt") as file:
    players, lastMarble = parse("{:d} players; last marble is worth {:d} points", file.readline())



currentValue = 1
currentIndex = 0
playerScores = [0]*players
currentPlayer = 0
marbles = [0]

previousScore = 0

#for i in range(25):
while previousScore != lastMarble:
    if currentValue % 23 == 0:
        currentScore = currentValue
        currentIndex = (currentIndex - 7) % len(marbles)
        currentScore += marbles.pop(currentIndex)
        if currentIndex == len(marbles):
            currentIndex = 0
        playerScores[currentPlayer] += currentScore
        previousScore = currentScore
    else:
        newIndex = (currentIndex+2) % len(marbles)
        marbles.insert(newIndex, currentValue)
        currentIndex = newIndex
        previousScore = currentValue

    currentValue += 1
    currentPlayer = (currentPlayer + 1) % players
    #print(marbles)
    #print(marbles[currentIndex])

playerScores.sort()
print(playerScores[players-1])