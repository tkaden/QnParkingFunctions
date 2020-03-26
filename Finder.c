#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <sys/resource.h>


#define nil -2 //non-zero null-like filler
#define Qsize 8 // THIS CHANGES WHEN DOING DIFFERENT DIMENSIONS CALCULATED BY 2^(DIM)
    int i,j;

//the adjacentcy matrix
int q3graph[Qsize][Qsize] = {0,1,1,0,1,0,0,0,1,0,0,1,0,1,0,0,1,0,0,1,0,0,1,0,0,1,1,0,0,0,0,1,1,0,0,0,0,1,1,0,0,1,0,0,1,0,0,1,0,0,1,0,1,0,0,1,0,0,0,1,0,1,1,0};

typedef struct Node{
        int * pfunction;
    int index;
}Node;

//printing function
void printPFunction(Node n)
{
    for (i = 0; i < Qsize; i++)
    {
        fflush(stdout);
        printf("%d, ", n.pfunction[i]);
    }
    printf("\n");
}

//this does all the work
void findAll(Node * n){
    for(i = 1; i < Qsize; i++){// parallel for reduce (stuff)
        if(n->pfunction[i] == nil){
            Node *newNode = malloc(sizeof(Node*));
            newNode->index = i;
            newNode->pfunction = malloc(Qsize*sizeof(int*));
            for (j = 0; j < Qsize; j++)
            {
                newNode->pfunction[j] = n->pfunction[j];
            }
            //free(n->pfunction);
            //free(n);
            newNode->pfunction[i] = -1;
            for(j = 0; j < Qsize; j++){
                if(newNode->pfunction[j] != nil){
                    newNode->pfunction[i] += q3graph[i][j];
                }
            }
            printPFunction(*newNode);
            if(newNode->pfunction[i] >= 0){
                findAll(newNode);
            }
            //free(newNode->pfunction);
            //free(newNode);
        }
    }
}
void printAdjMat()
{
    printf("Adj MAT\n");
    for (i = 0; i < Qsize; i++)
    {
        for (j = 0; j < Qsize; j++)
        {
            fflush(stdout);
            printf("%d", q3graph[i][j]);
        }
        printf("\n");
    }
}

int main(int argc, char * argv[])
{
    printAdjMat();

    Node * firstNode = malloc(sizeof(Node*));
    firstNode->index = 0;
    firstNode->pfunction = malloc(Qsize*sizeof(int*));
    firstNode->pfunction[0] = -1;
    for (i = 1; i < Qsize; i++)
    {
        firstNode->pfunction[i] = nil;
    }
    findAll(firstNode);
    free(firstNode);
    return 0;
}