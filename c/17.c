#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>
#include <ctype.h>
#include <math.h>
#define defaultInput "../../Inputs/2021_17.txt"
#include "../../Modules/input.h"
#include "../../Modules/set.h"

char *pointStr(int x, int y) {
    int sizex = x == 0 ? 1 : ceil(log10(abs(x) + 1));
    int sizey = y == 0 ? 1 : ceil(log10(abs(y) + 1));
    char *str = (char*)calloc(sizex + sizey + 4, sizeof(char));
    if (str == NULL) {
        printf("Memory allocation failed\n");
        exit(1);
    }
    sprintf(str, "%d,%d", x, y);
    return str;
}


int triangleNum(int x) {
    return x * (x + 1) / 2;
}

int *min_max(char *str) {
    int *arr = (int*)calloc(2, sizeof(int));
    char *temp = (char*)calloc(strlen(str) + 1, sizeof(char));
    strcpy(temp, str);

    char *p = strtok(temp, "..");
    arr[0] = atoi(p);
    p = strtok(NULL, "..");
    arr[1] = atoi(p);

    free(temp);
    return arr;
}

int part1(char *fileName) {
    struct vector *input_data = multiLine(fileName);

    int xMin, xMax, yMin, yMax;

    char *p = (char*)calloc(strlen(input_data->arr[0]) + 1, sizeof(char));
    strcpy(p, input_data->arr[0]);
    char *str1 = strtok(p, "=");
    str1 = strtok(NULL, "=");
    int *x_min_max = min_max(str1);
    free(p);

    char *str2 = strtok(input_data->arr[0], "=");
    str2 = strtok(NULL, "=");
    str2 = strtok(NULL, "=");
    int *y_min_max = min_max(str2);

    xMin = x_min_max[0];
    xMax = x_min_max[1];
    yMin = y_min_max[0];
    yMax = y_min_max[1];

    free(x_min_max);
    free(y_min_max);

    struct set *target = createSet(stringsize, copyElement);
    for (int y = yMin; y <= yMax; y++) {
        for (int x = xMin; x <= xMax; x++) {
            char *str = pointStr(x, y);
            addSet(target, str);
        }
    }

    int sxVel = 1;
    while (triangleNum(sxVel) < xMin) {
        sxVel++;
    }

    int landed = 0, highest = 0;
    for (int yV = yMin; yV <= 500; yV++) {
        for (int xV = sxVel; xV <= xMax; xV++) {
            int xVel = xV, yVel = yV, x = 0, y = 0, highestY = 0;

            while (x <= xMax && y >= yMin) {
                if (y > highestY) {
                    highestY = y;
                }

                if (inSet(target, pointStr(x, y))) {
                    landed++;

                    if (highestY > highest) {
                        highest = highestY;
                    }

                    break;
                }

                x += xVel;
                y += yVel;
                xVel--;
                yVel--;

                if (xVel < 0) {
                    xVel = 0;
                }
            }
        }
    }

    deleteSet(target, true);

    return highest;
}

int part2(char *fileName) {
    struct vector *input_data = multiLine(fileName);

    int xMin, xMax, yMin, yMax;
    
    char *p = (char*)calloc(strlen(input_data->arr[0]) + 1, sizeof(char));
    strcpy(p, input_data->arr[0]);
    char *str1 = strtok(p, "=");
    str1 = strtok(NULL, "=");
    int *x_min_max = min_max(str1);
    free(p);

    char *str2 = strtok(input_data->arr[0], "=");
    str2 = strtok(NULL, "=");
    str2 = strtok(NULL, "=");
    int *y_min_max = min_max(str2);

    xMin = x_min_max[0];
    xMax = x_min_max[1];
    yMin = y_min_max[0];
    yMax = y_min_max[1];

    free(x_min_max);
    free(y_min_max);

    struct set *target = createSet(stringsize, copyElement);
    for (int y = yMin; y <= yMax; y++) {
        for (int x = xMin; x <= xMax; x++) {
            addSet(target, pointStr(x, y));
        }
    }

    int sxVel = 1;
    while (triangleNum(sxVel) < xMin) {
        sxVel++;
    }

    int landed = 0, highest = 0;
    for (int yV = yMin; yV <= 500; yV++) {
        for (int xV = sxVel; xV <= xMax; xV++) {
            int xVel = xV, yVel = yV, x = 0, y = 0, highestY = 0;

            while (x <= xMax && y >= yMin) {
                if (y > highestY) {
                    highestY = y;
                }

                if (inSet(target, pointStr(x, y))) {
                    landed++;

                    if (highestY > highest) {
                        highest = highestY;
                    }

                    break;
                }

                x += xVel;
                y += yVel;
                xVel--;
                yVel--;

                if (xVel < 0) {
                    xVel = 0;
                }
            }
        }
    }

    deleteSet(target, true);

    return landed;
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
    printf("\nPart 1:\nHighest point reached: %d\nRan in %f seconds\n", p1, t_p1);

    t = clock(); 
    int p2 = part2(inputPath);
    t = clock() - t;
    double t_p2 = ((double)t) / CLOCKS_PER_SEC;
    printf("\nPart 2:\nVelocites that land: %d\nRan in %f seconds\n", p2, t_p2);

    return 0;
}