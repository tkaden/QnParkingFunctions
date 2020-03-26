Skip to content
Search or jump to…

Pull requests
Issues
Marketplace
Explore

@tkaden
JeremyTryon
/
G-parking-Functions
0
00
 Code Issues 0 Pull requests 0 Actions Projects 0 Wiki Security Insights
G-parking-Functions/ArrayParallelTries.c
@JeremyTryon JeremyTryon Add files via upload
0264f94 on Feb 2, 2019
312 lines (271 sloc)  9.85 KB

#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <sys/resource.h>
#include <sys/time.h>
#include <omp.h>
#include <mpi.h>
#include <string.h>

#define nil -2   //non-zero null-like filler DONT CHANGE

// QSize (calculated 2^(dimension)) of the graph is the where the nodes are the deepest in the tree. it is also the number of threads that you will use.
// n is the dimensions of the functions.

// Q2
/*
#define Qsize 4
#define n 2
int qgraph[Qsize][Qsize] = {0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0};
*/

// Q3
/*
#define Qsize 8
#define n 3
int qgraph[Qsize][Qsize] = {0,1,1,0,1,0,0,0,1,0,0,1,0,1,0,0,1,0,0,1,0,0,1,0,0,1,1,0,0,0,0,1,1,0,0,0,0,1,1,0,0,1,0,0,1,0,0,1,0,0,1,0,1,0,0,1,0,0,0,1,0,1,1,0};
*/

// Q4
/*
#define Qsize 16
#define n 4
int qgraph[Qsize][Qsize] = {0,1,0,1,1,0,0,0,0,0,0,0,1,0,0,0,1,0,1,0,0,1,0,0,0,0,0,0,0,1,0,0,0,1,0,1,0,0,1,0,0,0,0,0,0,0,1,0,1,0,1,0,0,0,0,1,0,0,0,0,0,0,0,1,1,0,0,0,0,1,0,1,1,0,0,0,0,0,0,0,0,1,0,0,1,0,1,0,0,1,0,0,0,0,0,0,0,0,1,0,0,1,0,1,0,0,1,0,0,0,0,0,0,0,0,1,1,0,1,0,0,0,0,1,0,0,0,0,0,0,0,0,1,0,0,0,0,1,0,1,1,0,0,0,0,0,0,0,0,1,0,0,1,0,1,0,0,1,0,0,0,0,0,0,0,0,1,0,0,1,0,1,0,0,1,0,0,0,0,0,0,0,0,1,1,0,1,0,0,0,0,1,1,0,0,0,0,0,0,0,1,0,0,0,0,1,0,1,0,1,0,0,0,0,0,0,0,1,0,0,1,0,1,0,0,0,1,0,0,0,0,0,0,0,1,0,0,1,0,1,0,0,0,1,0,0,0,0,0,0,0,1,1,0,1,0};
*/

// Q5

#define Qsize 32
#define n 5
int qgraph[Qsize][Qsize] = {0,1,0,1,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,1,0,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,1,0,1,0,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,1,0,1,0,0,0,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,1,0,0,0,0,1,0,1,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,1,0,0,1,0,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,1,0,0,1,0,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,1,1,0,1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,1,0,1,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,1,0,1,0,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,1,0,1,0,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,1,0,1,0,0,0,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,0,0,1,0,1,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,1,0,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,1,0,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,1,0,1,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,1,0,1,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,1,0,1,0,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,1,0,1,0,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,1,0,1,0,0,0,0,1,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,0,0,1,0,1,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,1,0,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,1,0,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,1,0,1,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,1,0,1,1,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,1,0,1,0,0,1,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,1,0,1,0,0,1,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,1,0,1,0,0,0,0,1,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,0,0,1,0,1,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,1,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,0,0,1,0,1,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,1,1,0,1,0};


//*****************************//
//********Tries****************//
//*****************************//
struct TrieNode
{
    struct TrieNode *children[n];
};

//function to create a new TrieNode
struct TrieNode *getNewTrieNode()
{
    int trieIterator;
    struct TrieNode *node = (struct TrieNode *)malloc(sizeof(struct TrieNode));

    for (trieIterator = 0; trieIterator < n; trieIterator++)
    {
        node->children[trieIterator] = NULL;
    }
    return node;
}

//Iterative function to insert a word into Trie from a pfunction
void insert(struct TrieNode **head, int *pfunction)
{
    int i;
    //start from root node
    struct TrieNode *curr = *head;
    for (i = 1; i < Qsize; i++)
    {
        //create a new node if path doesn't exist
        if (curr->children[pfunction[i]] == NULL)
        {
            curr->children[pfunction[i]] = getNewTrieNode();
        }
        //go to next node
        curr = curr->children[pfunction[i]];
    }
}

//returns 1 if given node has any children
int hasChildren(struct TrieNode *curr)
{
    int hasChildrenIterator;
    for (hasChildrenIterator = 0; hasChildrenIterator < n; hasChildrenIterator++)
    {
        if (curr->children[hasChildrenIterator])
        {
            return 1; //child found
        }
    }
    return 0;
}

//prints each word in the trie
void printTrieContents(struct TrieNode *head, char pfunction[], int level, int rank, FILE *fp)
{
    int i;

    if (level == 0)
    {
        pfunction[0] = 45;
        pfunction[1] = 1 + '0';
    }

    if (level == (Qsize - 1))
    {
        fprintf(fp, "%s\n", pfunction);
    }

    for (i = 0; i < n; i++)
    {
        if (head->children[i] != NULL)
        {
            pfunction[level + 2] = i + '0';

            printTrieContents(head->children[i], pfunction, level + 1, rank, fp);
        }
    }
}

void findAll(int *old, struct TrieNode *head)
{
    int i, j;
    int empty = 0;
    for (i = 1; i < Qsize; i++)
    { // parallel for reduce (stuff)
        if (old[i] == nil)
        {
            int *newArray = (int *)malloc(Qsize * sizeof(int));
            for (j = 0; j < Qsize; j++)
            {
                newArray[j] = old[j];
                if (old[j] == nil)
                {
                    empty++;
                }
            }
            newArray[i] = -1;
            for (j = 0; j < Qsize; j++)
            {
                if (newArray[j] != nil)
                {
                    newArray[i] += qgraph[i][j];
                }
            }
            empty--;

            if ((empty == 0))
            {
                insert(&head, newArray);
            }
            else if (newArray[i] >= 0)
            {
                findAll(newArray, head);
            }
            free(newArray);
        }
    }
}

void printAdjMat()
{
    int i, j;
    printf("Adj MAT\n");
    for (i = 0; i < Qsize; i++)
    {
        for (j = 0; j < Qsize; j++)
        {
            fflush(stdout);
            printf("%d", qgraph[i][j]);
        }
        printf("\n");
    }
}

int main(int argc, char **argv)
{
    struct timeval t1, t2;
    double elapsedTime;
    int numSlots, startPos, endPos, i, counter, j, rc, numtasks, rank;

    MPI_Status Status;

    MPI_Init(&argc, &argv);
    MPI_Comm_size(MPI_COMM_WORLD, &numtasks);
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);

    if (rank == 0) printAdjMat();

    int **iholder;
    struct TrieNode *head = getNewTrieNode();

    /// allocate dynamic double pointer to int
    iholder = (int **)malloc((Qsize - 1) * sizeof(int *));
    for (i = 0; i < (Qsize - 1); i++)
    {
        iholder[i] = (int *)malloc(Qsize*sizeof(int));
    }

    if (rank == 0)
    {
        gettimeofday(&t1, NULL);

        int *first = malloc(Qsize * sizeof(int));
        first[0] = -1;

        for (i = 1; i < Qsize; i++)
        {
            first[i] = nil;
        }

        counter = 0;
        int j ,k;

        for (i = 1; i < Qsize; i++)
        {
            if (first[i] == nil)
            {
                int *newArray = malloc(Qsize * sizeof(int*));
                for (j = 0; j < Qsize; j++)
                {
                    newArray[j] = first[j];
                }
                newArray[i] = -1;
                for (j = 0; j < Qsize; j++)
                {
                    if (newArray[j] != nil)
                    {
                        newArray[i] += qgraph[i][j];
                    }
                }

                if (newArray[i] >= 0)
                {
                    for(k = 0; k<Qsize; k++){
                        iholder[counter][k] = newArray[k];
                    }
                    counter++;
                }
                free(newArray);
            }
        }
    }

    //everyone needs counter
    MPI_Bcast(&counter, 1, MPI_INT, 0, MPI_COMM_WORLD);

    //everyone needs the contents of holder
    for (i = 0; i < counter; i++)
    {
        MPI_Bcast(iholder[i], Qsize, MPI_INT, 0, MPI_COMM_WORLD);
    }

    if (numtasks < counter)
    {
        if (rank > 0)
        {
            printf("number of threads must be 1 or number of threads must be >= dimension");
            MPI_Finalize();
            exit(-1);
        }
        startPos = 0;
        endPos = Qsize - 1;
    }
    else if (rank <= counter)
    {
        for (i = 0; i < Qsize - 1; i++)
        {
            if (rank == i)
            {
                startPos = i;
                endPos = i+1;
            }
        }
    }
    else
    {
        startPos = 0;
        endPos = 0;
    }

    for (i = startPos; i < endPos; i++)
    {
        findAll((iholder[i]), head); // Work method.
    }

    char *cpf = malloc((Qsize + 1) * sizeof(char));

    if (rank < counter)
    {
        FILE *fp;
        char *filename = malloc(15 * sizeof(char *));
        sprintf(filename, "Q%d-%d.out", n, rank); //change filename based on thread id
        fp = fopen(filename, "w");
        printTrieContents(head, cpf, 0, rank, fp);
        fclose(fp);
    }

    if (rank == 0)
    {
        gettimeofday(&t2, NULL);
        elapsedTime = (t2.tv_sec - t1.tv_sec) * 1000.0;    //sec to ms
        elapsedTime += (t2.tv_usec - t1.tv_usec) / 1000.0; // us to ms
        printf("DATA, %s, %f\n", getenv("SLURM_NTASKS"), elapsedTime);
        printf("DONE :)");
    }

    MPI_Finalize();
    return 0;
}
