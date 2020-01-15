width = 25
height = 6

layerLen = width*height
finalLayer = ["2"]*layerLen

with open("8input.txt") as file:
    contents = file.readline()

for layerIndex in range(0, len(contents), layerLen):
    #print(contents[index:index+layerLen])
    #for character in contents[index:index+layerLen]:
    for pixelIndex in range(layerLen):
        if finalLayer[pixelIndex] == "2":
            finalLayer[pixelIndex] = contents[layerIndex+pixelIndex]

# for i in range(0, len(finalLayer), width):
#     print("".join(finalLayer[i:i+layerLen]))
print(finalLayer)

for i in range(0, len(finalLayer), width):
    print("".join(finalLayer[i: i+layerLen]))