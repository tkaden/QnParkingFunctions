#!/bin/bash -l
module load OpenMPI
## -np *n = x for Qn graph* path
## Example:
mpirun -np 5 /path/to/FinderParallel.c