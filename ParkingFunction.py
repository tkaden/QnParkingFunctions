import itertools

def ParkingFunction():
  #creates parking function for all <1112>
  parkingX = list(itertools.permutations([0,0,0,1,1,1,2], 7))
  parkingX = list(set(parkingX))
  print len(parkingX)

  #creates parking functions for all <122>
  parkingY = list(itertools.permutations([0,0,0,0,1,2,2], 7))
  parkingY = list(set(parkingY))
  print len(parkingY)

  #adds both parking functions to create all parking functions
  parkingFull = list(parkingY+parkingX)
  print len(parkingFull)

  #creates a counter
  i = 0
  j = 0

  while i<len(parkingFull):
    #removes functions without a 0 in b,c,d
    if(parkingFull[i][0] != 0 and parkingFull[i][1] != 0 and parkingFull[i][2] !=0):
      parkingFull.pop(i)
      i = i+1
      continue

    #removes b=>ef
    if(parkingFull[i][0] == 2 and parkingFull[i][3] == 2) or (parkingFull[i][0] == 2 and parkingFull[i][4] == 2):
      parkingFull.pop(i)
      i = i + 1
      continue

    #removes c=>eg
    if (parkingFull[i][1] == 2 and parkingFull[i][3] == 2) or (parkingFull[i][1] == 2 and parkingFull[i][5] == 2):
      parkingFull.pop(i)
      i = i + 1
      continue

    #removes d=>fg
    if (parkingFull[i][2] == 2 and parkingFull[i][4] == 2) or (parkingFull[i][2] == 2 and parkingFull[i][5] == 2):
      parkingFull.pop(i)
      i = i + 1
      continue

    #removes e=>h
    if (parkingFull[i][3] == 2 and parkingFull[i][6] == 2):
      parkingFull.pop(i)
      i = i + 1
      continue

    #removes f=>h
    if (parkingFull[i][4] == 2 and parkingFull[i][6] == 2):
      parkingFull.pop(i)
      i = i + 1
      continue

    #removes f=>h
    if (parkingFull[i][5] == 2 and parkingFull[i][6] == 2):
      parkingFull.pop(i)
      i = i + 1
      continue

    i = i + 1

  while j < len(parkingFull):
    a = parkingFull[j]
    a = list(a)
    a.insert(0, -1)
    parkingFull[j] = a
    print a
    j = j + 1





