#This method is not called in this class, but it will print the adjMatrix
#in a very readable format if you want to call it. To call it just type:
#  'printMantrix(adjMatrix)'   on one of the lines.
def printMatrix(matrix):
    for i in range(len(matrix)):
       print(str(matrix[i]))

#a=0 b=1 c=2 d=3 e=4 f=5 g=6 h=7
edge_X = [0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7]
edge_Y = [1, 2, 3, 0, 4, 5, 0, 4, 6, 0, 5, 6, 1, 2, 7, 1, 3, 7, 2, 3, 7, 4, 5, 6]

#there are 8 nodes
n = 8

#adjacency matrix - initialize with 0
adjMatrix = [[0 for i in range(n)] for k in range(n)]

#places '1' in matrix
for i in range(len(edge_X)):
    x = edge_X[i]
    y = edge_Y[i]
    adjMatrix[x][y] = 1

#input will end up coming from the user. I'm currently just hard coding in one example
input = [-1, 2, 0, 0, 0, 0, 1, 2]
dummy = input
bool = True
pFunc = True
while (bool == True):
    for i in range(len(dummy)):
       if dummy[i] < 0:
           dummy[i] = 'B'
           for j in range(len(adjMatrix)):
               if dummy[j] != 'B':
                    dummy[j] = dummy[j]-adjMatrix[i][j]
    for i in range(len(dummy)):
        if (dummy[i] < 0 and not 'B'):
            bool = True
            continue
        else:
            bool = False

for i in range(len(dummy)):
    if (dummy[i] != 'B' and dummy[i] >= 0):
        pFunc = False

for i in range(len(dummy)):
    if (pFunc):
        dummy[i] = 'B'

print dummy
print pFunc