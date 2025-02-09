#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>
#include <ctype.h>
#include <math.h>
#define defaultInput "../../Inputs/2021_7.txt"
#include "../../Modules/input.h"


bool compareInt(void *e1, void *e2) {
    return *(int*)e1 <= *(int*)e2;
}


int triangle(int n) {
    return n * (n + 1) / 2;
}

int part1(char *fileName) {
    struct vector *input_data = singleLine(fileName, ",");

    for (int i = 0; i < input_data->len; i++) {
        int *e = (int*)calloc(1, sizeof(int));
        *e = atoi((char*)input_data->arr[i]);
        input_data->arr[i] = e;
    }

    input_data->e_size = intsize;
    input_data = sortVector(input_data, compareInt);

    int minFuel = -1;

    for (int pos = *(int*)input_data->arr[0]; pos <= *(int*)input_data->arr[input_data->len - 1]; pos++) {
        int fuel = 0;
        for (int i = 0; i < input_data->len; i++) {
            int dist = abs(pos - *(int*)input_data->arr[i]);
            fuel += dist;
        }

        if (minFuel == -1 || fuel < minFuel) {
            minFuel = fuel;
        }
    }

    return minFuel;
}

int part2(char *fileName) {
    struct vector *input_data = singleLine(fileName, ",");

    for (int i = 0; i < input_data->len; i++) {
        int *e = (int*)calloc(1, sizeof(int));
        *e = atoi((char*)input_data->arr[i]);
        input_data->arr[i] = e;
    }

    input_data->e_size = intsize;
    input_data = sortVector(input_data, compareInt);

    int minFuel = -1;

    for (int pos = *(int*)input_data->arr[0]; pos <= *(int*)input_data->arr[input_data->len - 1]; pos++) {
        int fuel = 0;
        for (int i = 0; i < input_data->len; i++) {
            int dist = abs(pos - *(int*)input_data->arr[i]);
            fuel += triangle(dist);
        }

        if (minFuel == -1 || fuel < minFuel) {
            minFuel = fuel;
        }
    }

    return minFuel;
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
    printf("\nPart 1:\nMinimum fuel: %d\nRan in %f seconds\n", p1, t_p1);

    t = clock(); 
    int p2 = part2(inputPath);
    t = clock() - t;
    double t_p2 = ((double)t) / CLOCKS_PER_SEC;
    printf("\nPart 2:\nMinimum fuel: %d\nRan in %f seconds\n", p2, t_p2);

    return 0;
}