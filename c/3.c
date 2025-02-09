#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>
#include <ctype.h>
#include "../../Modules/input.h"
#define defaultInput "../../Inputs/2021_3.txt"
#include "../../Modules/vector.h"

int part1(char *fileName) {
    struct vector *input_data = multiLine(fileName);
    struct vector *data = createVector(intsize, copyElement);

    for (int i = 0; i < input_data->len; i++) {
        int *d = (int*)calloc(1, sizeof(1));
        char *str = input_data->arr[i];
        for (int j = 0; j < strlen(str); j++) {
            *d *= 2;
            *d += (str[j] == '1') ? 1 : 0;
        }

        appendVector(data, d);
    }

    int gamma = 0, epsilon = 0, mask = 0b100000000000;
    while (mask > 0) {
        int zcount = 0, ocount = 0;
        for (int i = 0; i < data->len; i++) {
            int *n = data->arr[i];

            if (*n & mask) {
                ocount += 1;
            } else {
                zcount += 1;
            }
        }

        gamma *= 2;
        gamma += (ocount > zcount) ? 1 : 0;

        epsilon *= 2;
        epsilon += (ocount > zcount) ? 0 : 1;

        mask /= 2;
    }

    return gamma * epsilon;
}

int part2(char *fileName) {
    struct vector *input_data = multiLine(fileName);
    struct vector *data = createVector(intsize, copyElement);

    for (int i = 0; i < input_data->len; i++) {
        int *d = (int*)calloc(1, sizeof(1));
        char *str = input_data->arr[i];
        for (int j = 0; j < strlen(str); j++) {
            *d *= 2;
            *d += (str[j] == '1') ? 1 : 0;
        }

        appendVector(data, d);
    }

    struct vector *mostCommon = createCopyVector(data, sizeof(struct vector));
    struct vector *leastCommon = createCopyVector(data, sizeof(struct vector));

    int mask = 0b100000000000;

    while (mask > 0) {
        if (mostCommon->len > 1) {
            struct vector *oneset = createVector(intsize, copyElement);
            struct vector *zeroset = createVector(intsize, copyElement);
        
            for (int i = 0; i < mostCommon->len; i++) {
                if (*(int*)mostCommon->arr[i] & mask) {
                    appendVector(oneset, mostCommon->arr[i]);
                } else {
                    appendVector(zeroset, mostCommon->arr[i]);
                }
            }

            deleteVector(mostCommon, false);
            if (oneset->len >= zeroset->len) {
                deleteVector(zeroset, false);
                mostCommon = oneset;
            } else {
                deleteVector(oneset, false);
                mostCommon = zeroset;
            }
        }

        if (leastCommon->len > 1) {
            struct vector *oneset = createVector(intsize, copyElement);
            struct vector *zeroset = createVector(intsize, copyElement);
            
            for (int i = 0; i < leastCommon->len; i++) {
                if (*(int*)leastCommon->arr[i] & mask) {
                    appendVector(oneset, leastCommon->arr[i]);
                } else {
                    appendVector(zeroset, leastCommon->arr[i]);
                }
            }

            deleteVector(leastCommon, false);
            if (zeroset->len <= oneset->len) {
                deleteVector(oneset, false);
                leastCommon = zeroset;
            } else {
                deleteVector(zeroset, false);
                leastCommon = oneset;
            }
        }

        mask /= 2;
    }

    int o2 = *(int*)mostCommon->arr[0], co2 = *(int*)leastCommon->arr[0];
    return o2 * co2;
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
    printf("\nPart 1:\nGamma * Epsilon: %d\nRan in %f seconds\n", p1, t_p1);

    t = clock(); 
    int p2 = part2(inputPath);
    t = clock() - t;
    double t_p2 = ((double)t) / CLOCKS_PER_SEC;
    printf("\nPart 2:\nO2 Rating * CO2 Rating: %d\nRan in %f seconds\n", p2, t_p2);

    return 0;
}