#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>
#include <ctype.h>
#include "../../Modules/input.h"
#define defaultInput "../../Inputs/2021_1.txt"
#include "../../Modules/vector.h"

int part1(char *fileName) {
    struct vector *input_data = multiLine(fileName);
    struct vector *data = createVector(intsize, copyElement);

    for (int i = 0; i < input_data->len; i++) {
        int *val = (int*)calloc(1, sizeof(int));
        *(val) = atoi((char*)input_data->arr[i]);
        appendVector(data, val);
    }

    int increase_count = 0;
    for (int i = 1; i < data->len; i++) {
        increase_count += (*(int*)data->arr[i] > *(int*)data->arr[i - 1]) ? 1 : 0;
    }

    return increase_count;
}

int part2(char *fileName) {
    struct vector *input_data = multiLine(fileName);
    struct vector *data = createVector(intsize, copyElement);

    for (int i = 0; i < input_data->len; i++) {
        int *val = (int*)calloc(1, sizeof(int));
        *(val) = atoi((char*)input_data->arr[i]);
        appendVector(data, val);
    }

    struct vector *sum1 = sliceVector(data, 0, data->len - 2, 1);
    struct vector *sum2 = sliceVector(data, 1, data->len - 1, 1);
    struct vector *sum3 = sliceVector(data, 2, data->len, 1);
    
    int increase_count = 0;
    for (int i = 1; i < sum1->len; i++) {
        int val = *(int*)sum1->arr[i] + *(int*)sum2->arr[i] + *(int*)sum3->arr[i];
        int pval = *(int*)sum1->arr[i - 1] + *(int*)sum2->arr[i - 1] + *(int*)sum3->arr[i - 1];
        increase_count += (val > pval) ? 1 : 0;
    }

    return increase_count;
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
    printf("\nPart 1:\nNumber of measurements larger than the previous element: %d\nRan in %f seconds\n", p1, t_p1);
    
    t = clock(); 
    int p2 = part2(inputPath);
    t = clock() - t;
    double t_p2 = ((double)t) / CLOCKS_PER_SEC;
    printf("\nPart 2:\nNumber of windows larger than previous window: %d\nRan in %f seconds\n", p2, t_p2);    

    return 0;
}