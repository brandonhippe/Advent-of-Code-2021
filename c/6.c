#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>
#include <ctype.h>
#include <math.h>
#define defaultInput "../../Inputs/2021_6.txt"
#include "../../Modules/input.h"


unsigned long long countFish(struct vector *fish) {
    unsigned long long count = 0;
    for (int i = 0; i < fish->len; i++) {
        count += *(unsigned long long*)fish->arr[i];
    }

    return count;
}

unsigned long long part1(char *fileName) {
    struct vector *input_data = singleLine(fileName, ",");
    struct vector *fish = createVector(ullsize, copyElement);

    for (int i = 0; i < 9; i++) {
        unsigned long long *n = (unsigned long long*)calloc(1, sizeof(int));
        *n = 0;
        appendVector(fish, n);
    }

    for (int i = 0; i < input_data->len; i++) {
        *(unsigned long long*)(fish->arr[atoi(input_data->arr[i])]) = *(unsigned long long*)(fish->arr[atoi(input_data->arr[i])]) + 1;
    }

    for (int i = 0; i < 80; i++) {
        unsigned long long *reproduced = (unsigned long long*)fish->arr[0];
        fish = sliceVector(fish, 1, fish->len, 1);
        *(unsigned long long*)fish->arr[6] += *reproduced;
        appendVector(fish, reproduced);
    }

    return countFish(fish);
}

unsigned long long part2(char *fileName) {
    struct vector *input_data = singleLine(fileName, ",");
    struct vector *fish = createVector(ullsize, copyElement);

    for (int i = 0; i < 9; i++) {
        unsigned long long *n = (unsigned long long*)calloc(1, sizeof(int));
        *n = 0;
        appendVector(fish, n);
    }

    for (int i = 0; i < input_data->len; i++) {
        *(unsigned long long*)(fish->arr[atoi(input_data->arr[i])]) = *(unsigned long long*)(fish->arr[atoi(input_data->arr[i])]) + 1;
    }

    for (int i = 0; i < 256; i++) {
        unsigned long long *reproduced = (unsigned long long*)fish->arr[0];
        fish = sliceVector(fish, 1, fish->len, 1);
        *(unsigned long long*)fish->arr[6] += *reproduced;
        appendVector(fish, reproduced);
    }

    return countFish(fish);
}

int main (int argc, char *argv[]) {
    char *inputPath = defaultInput;
    if (argc > 1) {
        inputPath = argv[1];
    }

    clock_t t;
    t = clock(); 
    unsigned long long p1 = part1(inputPath);
    t = clock() - t; 
    double t_p1 = ((double)t) / CLOCKS_PER_SEC;
    printf("\nPart 1:\nNumber of fish after 80 days: %llu\nRan in %f seconds\n", p1, t_p1);

    t = clock(); 
    unsigned long long p2 = part2(inputPath);
    t = clock() - t;
    double t_p2 = ((double)t) / CLOCKS_PER_SEC;
    printf("\nPart 2:\nNumber of fish after 256 days: %llu\nRan in %f seconds\n", p2, t_p2);

    return 0;
}