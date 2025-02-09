#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>
#include <ctype.h>
#include "../../Modules/input.h"
#define defaultInput "../../Inputs/2021_4.txt"
#include "../../Modules/vector.h"


int calcScore(struct vector *board, struct vector *calledNums) {
    int score = 0, lastCalled = *(int*)calledNums->arr[calledNums->len - 1];

    for (int i = 0; i < board->len; i++) {
        struct vector *row = (struct vector*)board->arr[i];
        for (int j = 0; j < row->len; j++) {
            if (!inVector(calledNums, row->arr[j], row->e_size(row->arr[j]))) {
                score += *(int*)row->arr[j] * lastCalled;
            }
        }
    }

    return score;
}


void determineWin(struct vector *boardScores, struct vector *winningTurns, struct vector *rows, struct vector *cols, struct vector *numbers) {
    int *winningTurn = (int*)calloc(1, sizeof(int)), *score = (int*)calloc(1, sizeof(int));
    *winningTurn = numbers->len;

    for (int i = 0; i < rows->len; i++) {
        struct vector *row = (struct vector *)rows->arr[i], *col = (struct vector *)cols->arr[i];
        int rowWin = -1, colWin = -1;

        for (int j = 0; j < cols->len; j++) {
            int rowIx = indexVector(numbers, row->arr[j], row->e_size(row->arr[j]));

            if (rowIx < 0) {
                rowWin = numbers->len;
            }

            if (rowIx > rowWin) {
                rowWin = rowIx;
            }

            int colIx = indexVector(numbers, col->arr[j], col->e_size(col->arr[j]));

            if (colIx < 0) {
                colWin = numbers->len;
            }

            if (colIx > colWin) {
                colWin = colIx;
            }
        }

        if (rowWin < *winningTurn) {
            *winningTurn = rowWin;
            *score = calcScore(rows, sliceVector(numbers, 0, *winningTurn + 1, 1));
        }

        if (colWin < *winningTurn) {
            *winningTurn = colWin;
            *score = calcScore(rows, sliceVector(numbers, 0, *winningTurn + 1, 1));
        }
    }

    appendVector(winningTurns, winningTurn);
    appendVector(boardScores, score);
}

int part1(char *fileName) {
    struct vector *input_data = multiLine(fileName);

    struct vector *numbers = createVector(intsize, copyElement);
    char *numberStr = input_data->arr[0], *p = strtok(numberStr, ",");
    while (p) {
        int *n = (int*)calloc(1, sizeof(int));
        *n = atoi(p);
        appendVector(numbers, n);
        p = strtok(NULL, ",");
    }

    input_data = sliceVector(input_data, 2, input_data->len, 1);

    struct vector *boardScores = createVector(intsize, copyElement);
    struct vector *winningTurns = createVector(intsize, copyElement);

    for (int board = 0; board <= input_data->len; board += 6) {
        struct vector *rows = createVector(sizeofVector, createCopyVector);
        struct vector *cols = createVector(sizeofVector, createCopyVector);

        for (int i = 0; i < 5; i++) {
            appendVector(rows, createVector(intsize, copyElement));
            appendVector(cols, createVector(intsize, copyElement));
        }

        for (int i = 0; i < 5; i++) {
            char *boardStr = (char*)input_data->arr[board + i], *p = strtok(boardStr, " ");
            int j = 0;

            while (p) {
                int *num = (int*)calloc(1, sizeof(int));
                *(num) = atoi(p);

                appendVector(rows->arr[i], num);
                appendVector(cols->arr[j], num);

                j++;
                p = strtok(NULL, " ");
            }
        }

        determineWin(boardScores, winningTurns, rows, cols, numbers);
    }

    int first = *(int*)winningTurns->arr[0], firstScore = *(int*)boardScores->arr[0];
    for (int i = 1; i < winningTurns->len; i++) {
        int turn = *(int*)winningTurns->arr[i], score = *(int*)boardScores->arr[i];
        if (turn < first) {
            first = turn;
            firstScore = score;
        }
    }

    return firstScore;
}

int part2(char *fileName) {
    struct vector *input_data = multiLine(fileName);

    struct vector *numbers = createVector(intsize, copyElement);
    char *numberStr = input_data->arr[0], *p = strtok(numberStr, ",");
    while (p) {
        int *n = (int*)calloc(1, sizeof(int));
        *n = atoi(p);
        appendVector(numbers, n);
        p = strtok(NULL, ",");
    }

    input_data = sliceVector(input_data, 2, input_data->len, 1);

    struct vector *boardScores = createVector(intsize, copyElement);
    struct vector *winningTurns = createVector(intsize, copyElement);

    for (int board = 0; board <= input_data->len; board += 6) {
        struct vector *rows = createVector(sizeofVector, createCopyVector);
        struct vector *cols = createVector(sizeofVector, createCopyVector);

        for (int i = 0; i < 5; i++) {
            appendVector(rows, createVector(intsize, copyElement));
            appendVector(cols, createVector(intsize, copyElement));
        }

        for (int i = 0; i < 5; i++) {
            char *boardStr = (char*)input_data->arr[board + i], *p = strtok(boardStr, " ");
            int j = 0;

            while (p) {
                int *num = (int*)calloc(1, sizeof(int));
                *(num) = atoi(p);

                appendVector(rows->arr[i], num);
                appendVector(cols->arr[j], num);

                j++;
                p = strtok(NULL, " ");
            }
        }

        determineWin(boardScores, winningTurns, rows, cols, numbers);
    }

    int last = *(int*)winningTurns->arr[0], lastScore = *(int*)boardScores->arr[0];
    for (int i = 1; i < winningTurns->len; i++) {
        int turn = *(int*)winningTurns->arr[i], score = *(int*)boardScores->arr[i];
        if (turn > last) {
            last = turn;
            lastScore = score;
        }
    }

    return lastScore;
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
    printf("\nPart 1:\nScore of board that wins first: %d\nRan in %f seconds\n", p1, t_p1);

    t = clock(); 
    int p2 = part2(inputPath);
    t = clock() - t;
    double t_p2 = ((double)t) / CLOCKS_PER_SEC;
    printf("\nPart 2:\nScore of board that wins last: %d\nRan in %f seconds\n", p2, t_p2);

    return 0;
}