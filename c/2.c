#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>
#include <ctype.h>
#include "../../Modules/input.h"
#define defaultInput "../../Inputs/2021_2.txt"
#include "../../Modules/vector.h"


struct dataline {
    int amt;
    char *str;
};

size_t datalineSize(void *e) {
    struct dataline *dline = (struct dataline*)e;
    return sizeof(int) + strlen(dline->str);
}

void *copy_dataline(void *e, size_t size) {
    struct dataline *old = (struct dataline*)e;
    struct dataline *new = (struct dataline*)calloc(1, sizeof(struct dataline));
    new->str = (char*)calloc(strlen(old->str) + 1, sizeof(char));
    strcpy(new->str, old->str);
    new->amt = old->amt;

    return new;
}

int part1(char *fileName) {
    struct vector *input_data = multiLine(fileName);
    struct vector *data = createVector(datalineSize, copy_dataline);

    for (int i = 0; i < input_data->len; i++) {
        struct dataline *dline = (struct dataline*)calloc(1, sizeof(struct dataline));
        char *line = (char*)input_data->arr[i];
        char *p = strtok(line, " ");
        dline->str = (char*)calloc(strlen(p), sizeof(char));
        strcpy(dline->str, p);
        p = strtok(NULL, " ");
        dline->amt = atoi(p);
        appendVector(data, dline);
    }

    int x = 0, y = 0;
    for (int i = 0; i < data->len; i++) {
        struct dataline *dline = (struct dataline*)data->arr[i];
        if (strcmp(dline->str, "forward") == 0) {
            x += dline->amt;
        } else {
            y += (strcmp(dline->str, "up") == 0) ? -dline->amt : dline->amt;
        }
    }

    return x * y;
}

int part2(char *fileName) {
    struct vector *input_data = multiLine(fileName);
    struct vector *data = createVector(datalineSize, copy_dataline);

    for (int i = 0; i < input_data->len; i++) {
        struct dataline *dline = (struct dataline*)calloc(1, sizeof(struct dataline));
        char *line = (char*)input_data->arr[i];
        char *p = strtok(line, " ");
        dline->str = (char*)calloc(strlen(p), sizeof(char));
        strcpy(dline->str, p);
        p = strtok(NULL, " ");
        dline->amt = atoi(p);
        appendVector(data, dline);
    }


    int aim = 0, x = 0, y = 0;
    for (int i = 0; i < data->len; i++) {
        struct dataline *dline = (struct dataline*)data->arr[i];
        if (strcmp(dline->str, "forward") == 0) {
            x += dline->amt;
            y += aim * dline->amt;
        } else {
            aim += (strcmp(dline->str, "up") == 0) ? -dline->amt : dline->amt;
        }
    }

    return x * y;
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
    printf("\nPart 1:\nAnswer: %d\nRan in %f seconds\n", p1, t_p1);

    t = clock(); 
    int p2 = part2(inputPath);
    t = clock() - t;
    double t_p2 = ((double)t) / CLOCKS_PER_SEC;
    printf("\nPart 2:\nAnswer: %d\nRan in %f seconds\n", p2, t_p2);


    return 0;
}