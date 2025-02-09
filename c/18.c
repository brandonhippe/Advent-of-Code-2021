#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>
#include <ctype.h>
#include <math.h>
#define defaultInput "../../Inputs/2021_18.txt"
#include "../../Modules/input.h"
#include "../../Modules/set.h"


struct snailfish {
    int val;
    struct snailfish *parent, *left, *right;
};


size_t snailfishSize() {
    return sizeof(struct snailfish);
}


struct snailfish *copySf(struct snailfish *sf, struct snailfish *par) {
    struct snailfish *copy = (struct snailfish *)calloc(1, sizeof(struct snailfish));
    copy->val = sf->val;
    copy->parent = par;
    if (sf->left && sf->right) {
        copy->left = copySf(sf->left, copy);
        copy->right = copySf(sf->right, copy);
    }

    return copy;
}


char *printSf(struct snailfish *sf) {
    char *str;

    if (!sf->left && !sf->right) {
        int size = ceil(log10(abs(sf->val) + 1));
        str = (char*)calloc(size + 2, sizeof(char));
        sprintf(str, "%d", sf->val);
    } else {
        char *s1 = printSf(sf->left), *s2 = printSf(sf->right);
        str = (char*)calloc(strlen(s1) + strlen(s2) + 5, sizeof(char));
        sprintf(str, "[%s,%s]", s1, s2);
    }

    return str;
}


struct snailfish *readSf(char *line, struct snailfish *par) {
    struct snailfish* sf = (struct snailfish*)calloc(1, sizeof(struct snailfish));
    sf->parent = par;

    if (line[0] != '[') {
        sf->val = atoi(line);
        return sf;
    }

    char *p = line + 1;
    int brackets = 0, len = 0;
    while (strlen(p) && (brackets != 0 || p[0] != ',')) {
        if (p[0] == '[') {
            brackets++;
        } else if (p[0] == ']') {
            brackets--;
        }

        p++;
        len++;
    }

    char *l = (char*)calloc(len + 1, sizeof(char)), *r = (char*)calloc(strlen(p), sizeof(char));
    strncpy(l, line + 1, len);
    strncpy(r, p + 1, strlen(p) - 2);

    sf->left = readSf(l, sf);
    sf->right = readSf(r, sf);

    return sf;
}


bool explode(struct snailfish *sf, int depth) {
    if (!sf->left && !sf->right) {
        return false;
    }

    if (depth == 4) {
        struct snailfish *from = sf, *curr = sf->parent;
        while (curr->parent && curr->left == from) {
            from = curr;
            curr = curr->parent;
        }

        if (curr->left != from) {
            curr = curr->left;

            while (curr->right) {
                curr = curr->right;
            }

            curr->val += sf->left->val;
        }

        from = sf, curr = sf->parent;
        while (curr->parent && curr->right == from) {
            from = curr;
            curr = curr->parent;
        }

        if (curr->right != from) {
            curr = curr->right;

            while (curr->left) {
                curr = curr->left;
            }

            curr->val += sf->right->val;
        }

        sf->val = 0;
        sf->left = NULL;
        sf->right = NULL;

        return true;
    }

    if (explode(sf->left, depth + 1)) {
        return true;
    }

    return explode(sf->right, depth + 1);
}


bool split(struct snailfish *sf) {
    if (!sf->left && !sf->right) {
        if (sf->val >= 10) {
            sf->left = (struct snailfish *)calloc(1, sizeof(struct snailfish)), sf->right = (struct snailfish *)calloc(1, sizeof(struct snailfish));
            sf->left->parent = sf;
            sf->right->parent = sf;
            sf->left->val = sf->val / 2;
            sf->right->val = (sf->val + 1) / 2;
            sf->val = 0;
            return true;
        } else {
            return false;
        }
    }

    if (split(sf->left)) {
        return true;
    }

    return split(sf->right);
}


struct snailfish *reduceSf(struct snailfish *sf) {
    bool changed = true;
    while (changed) {
        changed = explode(sf, 0);
        if (!changed) {
            changed = split(sf);
        }
    }

    return sf;
}


int sfMag(struct snailfish *sf) {
    if (!sf->left && !sf->right) {
        return sf->val;
    }

    return 3 * sfMag(sf->left) + 2 * sfMag(sf->right);
}

int part1(char *fileName) {
    struct vector *input_data = multiLine(fileName), *snailfishes = createVector(snailfishSize, copyElement);

    for (int i = 0; i < input_data->len; i++) {
        appendVector(snailfishes, readSf((char*)input_data->arr[i], NULL));
    }

    struct snailfish *sf = copySf(snailfishes->arr[0], NULL);
    for (int i = 1; i < snailfishes->len; i++) {
        struct snailfish *newSf = (struct snailfish *)calloc(1, sizeof(struct snailfish));
        sf->parent = newSf;
        newSf->left = sf;
        newSf->right = copySf(snailfishes->arr[i], newSf);
        sf = reduceSf(newSf);
    }

    return sfMag(sf);
}

int part2(char *fileName) {
    struct vector *input_data = multiLine(fileName), *snailfishes = createVector(snailfishSize, copyElement);

    for (int i = 0; i < input_data->len; i++) {
        appendVector(snailfishes, readSf((char*)input_data->arr[i], NULL));
    }

    struct snailfish *sf = copySf(snailfishes->arr[0], NULL);
    int maxMag = 0;
    for (int i = 0; i < snailfishes->len; i++) {
        for (int j = 0; j < snailfishes->len; j++) {
            if (i == j) {
                continue;
            }

            free(sf);
            sf = (struct snailfish*)calloc(1, sizeof(struct snailfish));
            sf->left = copySf(snailfishes->arr[i], sf);
            sf->right = copySf(snailfishes->arr[j], sf);
            sf = reduceSf(sf);

            int mag = sfMag(sf);
            if (mag > maxMag) {
                maxMag = mag;
            }
        }
    }

    return maxMag;
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
    printf("\nPart 1:\nMagnitude of snailfish sum: %d\nRan in %f seconds\n", p1, t_p1);

    t = clock(); 
    int p2 = part2(inputPath);
    t = clock() - t;
    double t_p2 = ((double)t) / CLOCKS_PER_SEC;
    printf("\nPart 2:\nMaximum magnitude of sum of 2 snailfish numbers: %d\nRan in %f seconds\n", p2, t_p2);

    return 0;
}