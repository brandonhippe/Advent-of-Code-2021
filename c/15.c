#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>
#include <ctype.h>
#include <math.h>
#define defaultInput "../../Inputs/2021_15.txt"
#include "../../Modules/input.h"
#include "../../Modules/vector.h"
#include "../../Modules/heapq.h"

char *pos_to_str(int x, int y) {
    double x_len = ceil(log10(abs(x) + 1) + 2), y_len = ceil(log10(abs(y) + 1) + 2);

    char *pStr = (char*)calloc(x_len + y_len + 2, sizeof(char));
    sprintf(pStr, "%d,%d", x, y);
    return pStr;
}

int *str_to_pos(char *pStr) {
    char *temp = (char*)calloc(strlen(pStr) + 1, sizeof(char));
    strcpy(temp, pStr);
    int *pos = (int*)calloc(2, sizeof(int));
    char *p = strtok(temp, ",");
    pos[0] = atoi(p);
    p = strtok(NULL, ",");
    pos[1] = atoi(p);
    free(temp);

    return pos;
}

char *neighbors(char *pStr) {
    char *neighborStr = (char*)calloc(1, sizeof(char));
    int *pos = str_to_pos(pStr);

    for (int off = -1; off <= 1; off += 2) {
        char *temp = pos_to_str(pos[0] + off, pos[1]);
        neighborStr = (char*)realloc(neighborStr, strlen(neighborStr) + strlen(temp) + 2);
        strcat(neighborStr, temp);
        strcat(neighborStr, ";");

        temp = pos_to_str(pos[0], pos[1] + off);
        neighborStr = (char*)realloc(neighborStr, strlen(neighborStr) + strlen(temp) + 2);
        strcat(neighborStr, temp);
        strcat(neighborStr, ";");
    }

    return neighborStr;
}

char *costStr(int cost, int x, int y) {
    double len = ceil(log10(abs(cost) + 1) + 2) + ceil(log10(abs(x) + 1) + 2) + ceil(log10(abs(y) + 1) + 2);
    char *cStr = (char*)calloc(len + 3, sizeof(char));
    sprintf(cStr, "%d;%d,%d", cost, x, y);
    return cStr;
}

bool cmp(void *e1, void *e2) {
    char* p1 = (char*)e1, *p2 = (char*)e2;
    int l1 = 0, l2 = 0;
    while (p1[l1] != ';') {
        l1++;
    }

    while (p2[l2] != ';') {
        l2++;
    }

    char *c1 = (char*)calloc(l1 + 1, sizeof(char)), *c2 = (char*)calloc(l2 + 1, sizeof(char));
    strncpy(c1, p1, l1);
    strncpy(c2, p2, l2);
    int n1 = atoi(c1);
    int n2 = atoi(c2);
    free(c1);
    free(c2);

    return n1 <= n2;
}

int bfs(int x_dim, int y_dim, int *area) {
    struct heap *h = createHeap(createVector(stringsize, copyElement), cmp);
    int *open_list = (int*)calloc(x_dim * y_dim, sizeof(int)), *closed_list = (int*)calloc(x_dim * y_dim, sizeof(int));
    appendHeap(h, costStr(0, 0, 0));

    while (h->heapVec->len > 0) {
        char *cStr = (char*)popHeap(h);
        int pathCost = atoi(strtok(cStr, ";"));
        char *pStr = strtok(NULL, ";");
        int *pos = str_to_pos(pStr);
        int px = pos[0], py = pos[1];

        if (px == x_dim - 1 && py == y_dim - 1) {
            free(cStr);
            free(pos);
            return pathCost;
        }

        char *nStr = neighbors(pStr);
        char *neighbor = (char*)calloc(strlen(nStr), sizeof(char));
        while (strlen(nStr) > 0) {
            if (nStr[0] == ';') {                
                int *n = str_to_pos(neighbor);
                int x = n[0], y = n[1];
                if (x >= 0 && x < x_dim && y >= 0 && y < y_dim && (x != 0 || y != 0)) {
                    int nCost = pathCost + area[y * x_dim + x];

                    if ((nCost < open_list[y * x_dim + x] || open_list[y * x_dim + x] == 0) && (nCost < closed_list[y * x_dim + x] || closed_list[y * x_dim + x] == 0)) {
                        appendHeap(h, costStr(nCost, x, y));
                        open_list[y * x_dim + x] = nCost;
                    }
                }

                free(neighbor);
                free(n);
                neighbor = (char*)calloc(strlen(nStr), sizeof(char));
            } else {
                strncat(neighbor, nStr, 1);
            }

            nStr++;
        }

        free(cStr);
        free(neighbor);
        free(pos);
    }

    return -1;
}

int part1(char *fileName) {
    struct vector *input_data = multiLine(fileName);
    int x_dim = strlen((char*)input_data->arr[0]), y_dim = input_data->len;
    int *area = (int*)calloc(x_dim * y_dim, sizeof(int));
    for (int y = 0; y < y_dim; y++) {
        char *line = (char*)input_data->arr[y];
        for (int x = 0; x < x_dim; x++) {
            area[y * x_dim + x] = line[x] - '0';
        }
    }

    return bfs(x_dim, y_dim, area);
}

int part2(char *fileName) {
    struct vector *input_data = multiLine(fileName);
    int y_size = input_data->len, x_size = strlen((char*)input_data->arr[0]);
    int x_dim = x_size * 5, y_dim = y_size * 5;
    int *area = (int*)calloc(x_dim * y_dim, sizeof(int));

    for (int y = 0; y < y_size; y++) {
        char *line = (char*)input_data->arr[y];
        for (int x = 0; x < x_size; x++) {
            int n = line[x] - '0';
            for (int y_off = 0; y_off < 5; y_off++) {
                for (int x_off = 0; x_off < 5; x_off++) {
                    int nx = x + x_off * x_size, ny = y + y_off * y_size;
                    int val = (n - 1 + x_off + y_off) % 9 + 1;
                    area[ny * x_dim + nx] = val;
                }
            }
        }
    }

    return bfs(x_dim, y_dim, area);
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
    printf("\nPart 1:\nShortest Path: %d\nRan in %f seconds\n", p1, t_p1);

    t = clock(); 
    int p2 = part2(inputPath);
    t = clock() - t;
    double t_p2 = ((double)t) / CLOCKS_PER_SEC;
    printf("\nPart 2:\nShortest Path: %d\nRan in %f seconds\n", p2, t_p2);

    return 0;
}