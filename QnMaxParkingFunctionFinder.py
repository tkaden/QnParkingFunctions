q2graph = [[0, 1, 1, 0],
           [1, 0, 0, 1],
           [1, 0, 0, 1],
           [0, 1, 1, 0]]

def UserMatrix():
    rowCol = None
    while not (isinstance(rowCol, int)) or rowCol < 0 or rowCol == "":
        print ("Entry must be a non-negative integer value.")
        try:
            rowCol = int(input("Enter integer value. "))
        except NameError:
            print ("Invalid input.")

    #creates the 2D array for the matrix
    adjMatrix = [[0 for x in range(rowCol)] for y in range(rowCol)]

    for i in range(0, rowCol, 1):
        for j in range(i+1, rowCol, 1):
            entry = None
            while not (isinstance(entry, int)) or entry < 0:
                try:
                    entry = int(input("How many edges are between vertices "+str(i + 1)+" and "+str(j + 1)+"?"))
                except NameError:
                    print ("Invalid entry")
            adjMatrix[i][j] = entry
            adjMatrix[j][i] = entry
    return adjMatrix

def CreateQGraph(n):
    currentGraph = q2graph
    for i in range(3, n+1, 1):
        newGraph = [[0 for x in range(len(currentGraph*2))] for y in range(len(currentGraph*2))]
        for j in range(len(currentGraph)):
            for k in range(len(currentGraph)):
                newGraph[j][k] = currentGraph[j][k]
                newGraph[2**(i-1) + j][2**(i-1) + k] = currentGraph[j][k]
                newGraph[2**(i-1) + j][j] = 1
                newGraph[j][2**(i-1) + j] = 1
        currentGraph = newGraph
    return currentGraph

class Node(object):
#creating node objects that hold information about that node at that height
    def __init__(self, index, height, pfunction, parent):
        self.index = index
        self.height = height
        self.pfunction = pfunction
        self.parent = parent


sampleMatrix = [[0, 1, 0, 2], [1, 0, 1, 1], [0, 1, 0, 3], [2, 1, 3, 0]]
#should return pfunctions: [-1, 0, 0, 5], [-1, 0, 3, 2], [-1, 1, 3, 1], [-1, 2, 2, 1]

#this is what will be used to iterate through parent nodes and do the summation
def getValue(node, matrix):
    current = node.index
    value = 0
    while True:
        value += matrix[current][node.index] #starts by adding itself [i, i], but since the diagnonal is 0, it'll always just add 0
        #print(value)
        if node.parent is None:
            break
        node = node.parent
    return (value - 1)

#will check to see if a node is an ancestor to parent node and therefore not a valid node
#returns a bool of whether or not it is a valid node
def hasAncestor(node):
    ancestor = False
    index = node.index
    while True:
        if node.parent is None:
            break
        if index == node.parent.index:
            ancestor = True
            break
        node = node.parent
    return ancestor

#returns of a list of all nodes with the given height
def hasHeight(height, nodelist):
    newlist = []
    for Node in nodelist:
        if Node.height == height:
            newlist.append(Node)
    return newlist

def FindAll(matrix):
    nodeList = []
    currHeight = 0
    loop_counter = 0
    maxHeight = len(matrix) - 1 #tells us when to stop the BFS
    #None represents to be determined (TBD) values in the parking function
    tempPfunc = [None for x in range(len(matrix))]
    #Gives the root of the search that gives just [0,0,0,0,...,0]
    root = Node(0, 0, tempPfunc, None)
    #adds the root to the nodelist
    nodeList.append(root)
    #assins the first value to -1 for all arrays
    tempPfunc[root.index] = - 1
    root.pfunction = tempPfunc[:]
    while currHeight <= maxHeight:
        heightList = hasHeight(currHeight, nodeList)
        for node in heightList:
            for i in range(len(matrix)):
                loop_counter += 1
                childNode = Node(i, currHeight + 1, None, node)
                if not hasAncestor(childNode) and getValue(childNode, matrix) >= 0: #and (____ or _____)
                    childNode.pfunction = node.pfunction[:] #chloe henderson is python god
                    childNode.pfunction[i] = getValue(childNode, matrix)
                    nodeList.append(childNode)
                    print(childNode.pfunction)
        currHeight += 1
    print(str(loop_counter))
    return nodeList


def PrintAll(matrix):
    newlist = FindAll(matrix)
    newerlist = hasHeight(len(matrix) - 1, newlist)
    newestlist = []
    count = 0
    for Node in newerlist:
        newestlist.append(tuple(Node.pfunction))
    s = list(set(newestlist))
    for i in range(len(s)):
        print(str(s[i])+"\n")
        count += 1
    print("There are "+str(count)+" total parking functions.")
    return s

def CheckAll(lists, adjMatrix):
    for i in range(len(lists)):
        input = list(lists[i])
        dummy = input
        bool = True
        pFunc = True
        count = 0
        while (bool == True):
            count = count + 1
            for i in range(len(dummy)):
                if dummy[i] < 0:
                    dummy[i] = 'B'
                    for j in range(len(adjMatrix)):
                        if dummy[j] != 'B':
                            dummy[j] = dummy[j] - adjMatrix[i][j]
                if count == len(dummy):
                    bool = False

        for i in range(len(dummy)):
            if (dummy[i] != 'B' and dummy[i] >= 0):
                pFunc = False

        for i in range(len(dummy)):
            if (pFunc):
                dummy[i] = 'B'

        print(pFunc)

#so we know for q_2 if indexes 1 AND 2 are > 0 then that graph can not be a parking function
#similary, for q_3, if 1, 2, 3 are > 0 then that graph can not be a parking function
#this just checks those cases

def cheap_check(pfunc, n):
    list_conv = list(pfunc)
    #if this is equal to n after the loop, we know the test has failed
    greater_counter = 0
    for i in range(1, n+1, 1):
        if (list_conv[i] > 0):
            greater_counter += 1
    if greater_counter == n:
        print(pfunc)

def cheap_check_mult(lists, n):
    lists_conv = list(lists)
    for i in range(len(lists_conv)):
        cheap_check(lists_conv[i], n)

def maybe(list):
    zero = False
    if list[1] == 0 or list[2] == 0 or list[4] == 0:
        zero = True
    return True

def maybe_mult(lists):
    lists_conv = list(lists)
    for i in range(len(lists_conv)):
        print(maybe(lists_conv[i]))

def format_levels(list):
    newList = []
    newList.append(list[0])
    newList.append(list[1])
    newList.append(list[4])
    newList.append(list[2])
    newList.append(list[5])
    newList.append(list[3])
    newList.append(list[6])
    newList.append(list[7])
    print(newList)

def format_levels_mult(lists):
    lists_conv = list(lists)
    for i in range(len(lists_conv)):
        format_levels(lists_conv[i])



matrix = CreateQGraph(2)
gross_matrix = [[0,2,2,0],
                [2,0,0,2],
                [2,0,0,2],
                [0,2,2,0]]
even_nastier_matrix = [[0,3,2],
                       [3,0,1],
                       [2,1,0]]
allPfunc = PrintAll(even_nastier_matrix)
#format_levels_mult(allPfunc)
#maybe_mult(allPfunc)
#CheckAll(allPfunc, matrix)
#cheap_check_mult(allPfunc, 3)
