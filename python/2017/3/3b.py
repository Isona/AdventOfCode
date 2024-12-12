inputNum = 347991
spiral = {"0,0":1}
spiralLength = 1
currentX = 1
currentY = 0

def strCoord(x, y):
    return str(x) + "," + str(y)

def getValue(x, y):
    value = 0
    print("Calculating " + strCoord(x,y))
    for i in [x-1, x, x+1]:
        for j in [y-1, y, y+1]:
            if strCoord(i, j) in spiral:
                print("Adding " + strCoord(i, j) + ": " + str(spiral[strCoord(i,j)]))
                value += spiral[strCoord(i,j)]

    if value > inputNum:
        print(value)
        exit()
    else:
        print(strCoord(x, y) + ": " + str(value))
        return(value)
        
while True:
    # go north until the boundary
    while currentY < spiralLength:
        spiral[strCoord(currentX,currentY)] = getValue(currentX,currentY)
        currentY+=1

    #go west
    while currentX > -spiralLength:
        spiral[strCoord(currentX,currentY)] = getValue(currentX,currentY)
        currentX-=1

    #go south
    while currentY > -spiralLength:
        spiral[strCoord(currentX,currentY)] = getValue(currentX,currentY)
        currentY-=1

    #go east
    while currentX <= spiralLength:
        spiral[strCoord(currentX,currentY)] = getValue(currentX,currentY)
        currentX+=1

    spiralLength+=1