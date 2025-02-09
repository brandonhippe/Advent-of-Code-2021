#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>
#include <ctype.h>
#include <math.h>
#define defaultInput "../../Inputs/2021_10.txt"
#include "../../Modules/input.h"


bool ullcmp(void *e1, void *e2) {
    return *(unsigned long long *)e1 <= *(unsigned long long *)e2;
}

int part1(char *fileName) {
    struct vector *input_data = multiLine(fileName), *scores = createVector(ullsize, copyElement);
    int corruptedScore = 0;

    for (int i = 0; i < input_data->len; i++) {
        char *line = (char*)input_data->arr[i];

        struct vector *stack = createVector(charsize, copyElement);

        int j;
        for (j = 0; j < strlen(line); j++) {
            char *c;
            if (line[j] == '(' || line[j] == '[' || line[j] == '{' || line[j] == '<') {
                c = (char*)calloc(1, sizeof(char));
                *c = line[j];
                appendVector(stack, c);
            } else {
                c = popVector(stack);
                if (!((*c == '(' && line[j] == ')') || (*c == '[' && line[j] == ']') || (*c == '{' && line[j] == '}') || (*c == '<' && line[j] == '>'))) {
                    break;
                }
            }
        }

        if (j != strlen(line)) {
            switch (line[j]) {
                case ')':
                    corruptedScore += 3;
                    break;
                case ']':
                    corruptedScore += 57;
                    break;
                case '}':
                    corruptedScore += 1197;
                    break;
                case '>':
                    corruptedScore += 25137;
                    break;
            }

            continue;
        }

        unsigned long long *score = (unsigned long long *)calloc(1, sizeof(unsigned long long));
        while (stack->len != 0) {
            char *c = (char*)popVector(stack);
            *score *= 5;
            switch (*c) {
                case '(':
                    *score += 1;
                    break;
                case '[':
                    *score += 2;
                    break;
                case '{':
                    *score += 3;
                    break;
                case '<':
                    *score += 4;
                    break;
            }
        }

        appendVector(scores, score);
    }

    scores = sortVector(scores, ullcmp);
    return corruptedScore;
}

unsigned long long part2(char *fileName) {
    struct vector *input_data = multiLine(fileName), *scores = createVector(ullsize, copyElement);
    int corruptedScore = 0;

    for (int i = 0; i < input_data->len; i++) {
        char *line = (char*)input_data->arr[i];

        struct vector *stack = createVector(charsize, copyElement);

        int j;
        for (j = 0; j < strlen(line); j++) {
            char *c;
            if (line[j] == '(' || line[j] == '[' || line[j] == '{' || line[j] == '<') {
                c = (char*)calloc(1, sizeof(char));
                *c = line[j];
                appendVector(stack, c);
            } else {
                c = popVector(stack);
                if (!((*c == '(' && line[j] == ')') || (*c == '[' && line[j] == ']') || (*c == '{' && line[j] == '}') || (*c == '<' && line[j] == '>'))) {
                    break;
                }
            }
        }

        if (j != strlen(line)) {
            switch (line[j]) {
                case ')':
                    corruptedScore += 3;
                    break;
                case ']':
                    corruptedScore += 57;
                    break;
                case '}':
                    corruptedScore += 1197;
                    break;
                case '>':
                    corruptedScore += 25137;
                    break;
            }

            continue;
        }

        unsigned long long *score = (unsigned long long *)calloc(1, sizeof(unsigned long long));
        while (stack->len != 0) {
            char *c = (char*)popVector(stack);
            *score *= 5;
            switch (*c) {
                case '(':
                    *score += 1;
                    break;
                case '[':
                    *score += 2;
                    break;
                case '{':
                    *score += 3;
                    break;
                case '<':
                    *score += 4;
                    break;
            }
        }

        appendVector(scores, score);
    }

    scores = sortVector(scores, ullcmp);
    return *(unsigned long long *)scores->arr[scores->len / 2];
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
    printf("\nPart 1:\nTotal Syntax Error for Corrupted Lines: %d\nRan in %f seconds\n", p1, t_p1);

    t = clock(); 
    unsigned long long p2 = part2(inputPath);
    t = clock() - t;
    double t_p2 = ((double)t) / CLOCKS_PER_SEC;
    printf("\nPart 2:\nMiddle Incomplete Score: %llu\nRan in %f seconds\n", p2, t_p2);

    return 0;
}