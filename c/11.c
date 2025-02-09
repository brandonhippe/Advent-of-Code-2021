#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>
#include <ctype.h>
#include <math.h>
#define defaultInput "../../Inputs/2021_11.txt"
#include "../../Modules/input.h"
#include "../../Modules/dict.h"


char *pointStr(int x, int y) {
    int sizex = ceil(log10(abs(x) + 1)), sizey = ceil(log10(abs(y) + 1));
    char *str = (char*)calloc(sizex + sizey + 2, sizeof(char));
    sprintf(str, "%d,%d", x, y);
    return str;
}


struct vector *strPoint(char *pStr) {
    struct vector *point = createVector(intsize, copyElement);
    char *cpy = (char*)calloc(strlen(pStr), sizeof(char));
    strcpy(cpy, pStr);

    char *p = strtok(cpy, ",");
    while (p) {
        int *n = (int*)calloc(1, sizeof(int));
        *n = atoi(p);
        appendVector(point, n);

        p = strtok(NULL, ",");
    }

    return point;
}


int flashOcto(struct dict *octopi, int fx, int fy) {
    int flashes = 1;

    for (int xoff = -1; xoff <= 1; xoff++) {
        for (int yoff = -1; yoff <= 1; yoff++) {
            if (xoff == 0 && yoff == 0) {
                continue;
            }

            char *nP = pointStr(fx + xoff, fy + yoff);
            if (inDict(octopi, nP)) {
                int *octo = (int*)getDictVal(octopi, nP);
                *octo = *octo + 1;

                if (*octo == 10) {
                    flashes += flashOcto(octopi, fx + xoff, fy + yoff);
                }
            }
        }
    }

    return flashes;
}

int part1(char *fileName) {
    struct vector *input_data = multiLine(fileName);
    struct dict *octopi = createDict(stringsize, intsize, copyElement);

    for (int y = 0; y < input_data->len; y++) {
        char *line = (char*)input_data->arr[y];
        for (int x = 0; x < strlen(line); x++) {
            char *c = (char*)calloc(2, sizeof(char));
            strncpy(c, line + x, 1);
            int *n = (int*)calloc(1, sizeof(int));
            *n = atoi(c);
            addDict(octopi, pointStr(x, y), n);
        }
    }

    struct vector *locs = keys2vector(octopi);
    int flashes = 0;
    for (int steps = 0; steps < 100; steps++) {
        for (int i = 0; i < locs->len; i++) {
            struct vector *posVec = strPoint((char*)locs->arr[i]);
            int *octo = (int*)getDictVal(octopi, locs->arr[i]), x = *(int*)posVec->arr[0], y = *(int*)posVec->arr[1];
            *octo = *octo + 1;

            if (*octo == 10) {
                flashes += flashOcto(octopi, x, y);
            }
        }

        for (int i = 0; i < locs->len; i++) {
            int *octo = (int*)getDictVal(octopi, locs->arr[i]);

            if (*octo >= 10) {
                *octo = 0;
            }
        }
    }

    return flashes;
}

int part2(char *fileName) {
    struct vector *input_data = multiLine(fileName);
    struct dict *octopi = createDict(stringsize, intsize, copyElement);

    for (int y = 0; y < input_data->len; y++) {
        char *line = (char*)input_data->arr[y];
        for (int x = 0; x < strlen(line); x++) {
            char *c = (char*)calloc(2, sizeof(char));
            strncpy(c, line + x, 1);
            int *n = (int*)calloc(1, sizeof(int));
            *n = atoi(c);
            addDict(octopi, pointStr(x, y), n);
        }
    }

    struct vector *locs = keys2vector(octopi);
    int steps = 0, flashes = 0;
    while (true) {
        for (int i = 0; i < locs->len; i++) {
            struct vector *posVec = strPoint((char*)locs->arr[i]);
            int *octo = (int*)getDictVal(octopi, locs->arr[i]), x = *(int*)posVec->arr[0], y = *(int*)posVec->arr[1];
            *octo = *octo + 1;

            if (*octo == 10) {
                flashes += flashOcto(octopi, x, y);
            }
        }

        int flashed = 0;
        for (int i = 0; i < locs->len; i++) {
            int *octo = (int*)getDictVal(octopi, locs->arr[i]);

            if (*octo >= 10) {
                *octo = 0;
                flashed++;
            }
        }

        steps++;
        if (flashed == octopi->len) {
            break;
        }
    }

    return steps;
}


int main (int argc, char *argv[]) {
    char *inputPath = defaultInput;
    if (argc > 1) {
        inputPath = argv[1];
    }

    clock_t t;
    t = clock(); 
    int p1 = part1(inputPath);
    t = clock() - t; 
    double t_p1 = ((double)t) / CLOCKS_PER_SEC;
    printf("\nPart 1:\nFlashes in 100 steps: %d\nRan in %f seconds\n", p1, t_p1);

    t = clock(); 
    int p2 = part2(inputPath);
    t = clock() - t;
    double t_p2 = ((double)t) / CLOCKS_PER_SEC;
    printf("\nPart 2:\nNumber of steps to synchronous octopi flash: %d\nRan in %f seconds\n", p2, t_p2);

    return 0;
}