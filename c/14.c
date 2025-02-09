#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>
#include <ctype.h>
#include <math.h>
#define defaultInput "../../Inputs/2021_14.txt"
#include "../../Modules/input.h"
#include "../../Modules/dict.h"


bool ullcmp(void *e1, void *e2) {
    return *(unsigned long long*)e1 <= *(unsigned long long*)e2;
}

unsigned long long part1(char *fileName) {
    struct vector *input_data = multiLine(fileName);
    struct dict *pairAmts = createDict(stringsize, ullsize, copyElement), *pairs = createDict(stringsize, sizeofVector, createCopyVector), *letterAmts = createDict(charsize, ullsize, copyElement);

    char *initial = (char*)input_data->arr[0];
    while (strlen(initial) >= 2) {
        char *p = (char*)calloc(3, sizeof(char));
        strncpy(p, initial, 2);
        unsigned long long *n;
        if (!inDict(pairAmts, p)) {
            n = (unsigned long long*)calloc(1, sizeof(unsigned long long));
            addDict(pairAmts, p, n);
        }

        n = (unsigned long long*)getDictVal(pairAmts, p);
        *n = *n + 1;

        if (!inDict(letterAmts, &initial[0])) {
            n = (unsigned long long*)calloc(1, sizeof(unsigned long long));
            addDict(letterAmts, &initial[0], n);
        }

        n = (unsigned long long*)getDictVal(letterAmts, &initial[0]);
        *n = *n + 1;

        initial++;
    }

    if (!inDict(letterAmts, &initial[0])) {
        unsigned long long *n = (unsigned long long*)calloc(1, sizeof(unsigned long long));
        addDict(letterAmts, &initial[0], n);
    }

    unsigned long long *n = (unsigned long long*)getDictVal(letterAmts, &initial[0]);
    *n = *n + 1;

    for (int i = 2; i < input_data->len; i++) {
        char *line = (char*)input_data->arr[i], *p = (char*)calloc(3, sizeof(char));
        strncpy(p, line, 2);
        
        if (!inDict(pairs, p)) {
            struct vector *reps = createVector(stringsize, copyElement);
            addDict(pairs, p, reps);
        }

        struct vector *reps = getDictVal(pairs, p);
        while (strlen(p)) {
            char *insert = (char*)calloc(3, sizeof(char));
            if (strlen(p) > 1) {
                strncpy(insert, p, 1);
                strncat(insert, line + strlen(line) - 1, 1);
            } else {
                strncpy(insert, line + strlen(line) - 1, 1);
                strncat(insert, p, 1);
            }

            appendVector(reps, insert);
            p++;
        }
    }

    for (int i = 0; i < 10; i++) {
        if (i == 10) {
            
        }

        struct dict *newPairAmts = createDict(stringsize, ullsize, copyElement);
        struct vector *allPairs = keys2vector(pairAmts);
        for (int j = 0; j < allPairs->len; j++) {
            unsigned long long *amt = getDictVal(pairAmts, allPairs->arr[j]);

            struct vector *subs = (struct vector *)getDictVal(pairs, allPairs->arr[j]);
            
            unsigned long long *n;
            for (int k = 0; k < subs->len; k++) {
                if (!inDict(newPairAmts, subs->arr[k])) {
                    n = (unsigned long long*)calloc(1, sizeof(unsigned long long));
                    addDict(newPairAmts, subs->arr[k], n);
                }

                n = (unsigned long long*)getDictVal(newPairAmts, subs->arr[k]);
                *n = *n + *amt;
            }

            char *inserted = (char*)calloc(2, sizeof(char));
            strncpy(inserted, subs->arr[1], 1);

            if (!inDict(letterAmts, inserted)) {
                n = (unsigned long long*)calloc(1, sizeof(unsigned long long));
                addDict(letterAmts, inserted, n);
            }

            n = (unsigned long long *)getDictVal(letterAmts, inserted);
            *n = *n + *amt;
        }

        pairAmts = newPairAmts;
    }

    struct vector *amts = values2vector(letterAmts);
    amts = sortVector(amts, ullcmp);
    return *(unsigned long long*)amts->arr[amts->len - 1] - *(unsigned long long*)amts->arr[0];
}

unsigned long long part2(char *fileName) {
    struct vector *input_data = multiLine(fileName);
    struct dict *pairAmts = createDict(stringsize, ullsize, copyElement), *pairs = createDict(stringsize, sizeofVector, createCopyVector), *letterAmts = createDict(charsize, ullsize, copyElement);

    char *initial = (char*)input_data->arr[0];
    while (strlen(initial) >= 2) {
        char *p = (char*)calloc(3, sizeof(char));
        strncpy(p, initial, 2);
        unsigned long long *n;
        if (!inDict(pairAmts, p)) {
            n = (unsigned long long*)calloc(1, sizeof(unsigned long long));
            addDict(pairAmts, p, n);
        }

        n = (unsigned long long*)getDictVal(pairAmts, p);
        *n = *n + 1;

        if (!inDict(letterAmts, &initial[0])) {
            n = (unsigned long long*)calloc(1, sizeof(unsigned long long));
            addDict(letterAmts, &initial[0], n);
        }

        n = (unsigned long long*)getDictVal(letterAmts, &initial[0]);
        *n = *n + 1;

        initial++;
    }

    if (!inDict(letterAmts, &initial[0])) {
        unsigned long long *n = (unsigned long long*)calloc(1, sizeof(unsigned long long));
        addDict(letterAmts, &initial[0], n);
    }

    unsigned long long *n = (unsigned long long*)getDictVal(letterAmts, &initial[0]);
    *n = *n + 1;

    for (int i = 2; i < input_data->len; i++) {
        char *line = (char*)input_data->arr[i], *p = (char*)calloc(3, sizeof(char));
        strncpy(p, line, 2);
        
        if (!inDict(pairs, p)) {
            struct vector *reps = createVector(stringsize, copyElement);
            addDict(pairs, p, reps);
        }

        struct vector *reps = getDictVal(pairs, p);
        while (strlen(p)) {
            char *insert = (char*)calloc(3, sizeof(char));
            if (strlen(p) > 1) {
                strncpy(insert, p, 1);
                strncat(insert, line + strlen(line) - 1, 1);
            } else {
                strncpy(insert, line + strlen(line) - 1, 1);
                strncat(insert, p, 1);
            }

            appendVector(reps, insert);
            p++;
        }
    }

    for (int i = 0; i < 40; i++) {
        struct dict *newPairAmts = createDict(stringsize, ullsize, copyElement);
        struct vector *allPairs = keys2vector(pairAmts);
        for (int j = 0; j < allPairs->len; j++) {
            unsigned long long *amt = getDictVal(pairAmts, allPairs->arr[j]);

            struct vector *subs = (struct vector *)getDictVal(pairs, allPairs->arr[j]);
            
            unsigned long long *n;
            for (int k = 0; k < subs->len; k++) {
                if (!inDict(newPairAmts, subs->arr[k])) {
                    n = (unsigned long long*)calloc(1, sizeof(unsigned long long));
                    addDict(newPairAmts, subs->arr[k], n);
                }

                n = (unsigned long long*)getDictVal(newPairAmts, subs->arr[k]);
                *n = *n + *amt;
            }

            char *inserted = (char*)calloc(2, sizeof(char));
            strncpy(inserted, subs->arr[1], 1);

            if (!inDict(letterAmts, inserted)) {
                n = (unsigned long long*)calloc(1, sizeof(unsigned long long));
                addDict(letterAmts, inserted, n);
            }

            n = (unsigned long long *)getDictVal(letterAmts, inserted);
            *n = *n + *amt;
        }

        pairAmts = newPairAmts;
    }

    struct vector *amts = values2vector(letterAmts);
    amts = sortVector(amts, ullcmp);

    return *(unsigned long long*)amts->arr[amts->len - 1] - *(unsigned long long*)amts->arr[0];
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
    printf("\nPart 1:\nMost common - least common: %llu\nRan in %f seconds\n", p1, t_p1);

    t = clock(); 
    unsigned long long p2 = part2(inputPath);
    t = clock() - t;
    double t_p2 = ((double)t) / CLOCKS_PER_SEC;
    printf("\nPart 2:\nMost common - least common: %llu\nRan in %f seconds\n", p2, t_p2);

    return 0;
}