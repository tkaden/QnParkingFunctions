# Algorithm for Finding all Q<sub>n</sub> Maximal Parking Functions

## Problem

Given a connected graph, G, we prove an algorithm to efficiently list all maximal G-parking functions. Since many combinatorial bijections between G-parking functions and spanning trees have been established in the literature, enumeration of spanning trees of G and G-parking functions can be viewed as related problems.

As an application, we implement our algorithm into Python and C to consider the enumeration of G-parking functions on Q<sub>n</sub> graphs with n > 2 in order to consider a simple combinatorial proof of the formula for the number of spanning trees of Q<sub>n</sub> which Stanley refers to as not being known in [Enumerative Combinatorics, Vol. II](https://klein.mit.edu/~rstan/ec/). Our algorithm allows us to consider an experimental approach to this problem by expanding on [Dhar's Burning Algorithm](https://link-url-here.org)

The original goal was to develop an algorithm specifically for Q<sub>n</sub> graphs, but this algorithm works on any graph. However, depending on the symmetry of the graph, you'll have a different collection of maximal parking functions depending on the vertex you start at. Q<sub>n</sub> graphs being symmetric for all n and at any starting vertex, will always yield the same maximal parking functions for a given n.

The end goal of this project is/was to find a bijection between the number of max parking functions of a graph and the number of spanning trees of a graph. This was previously impossible to even study due to the computational complexity of finding a single maximal parking function of a graph. Refer to 'Draft.pdf' for more information and references.

## Code

QnMaxParkingFunctionFinder.py was the first interation of the algorithm. This was developed first in 2016 as my first venture into programming. Computational complexity of the algorithm used in the python code is around n<sup>n</sup> making it unusable for n > 3.

Finder.c and FinderParellel.c was developed as part of a high performance computing course at Kansas State University's high performance computing course, CIS 605. The C code is optimized and ran in parrellel to run on Q<sub>n</sub> graphs with n > 3.

To run the code, reference [Beocat](https://support.beocat.ksu.edu/BeocatDocs/index.php?title=Main_Page) or any other HPC; updating the shell scrips as needed.