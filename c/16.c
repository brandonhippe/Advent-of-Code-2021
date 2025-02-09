#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <stdbool.h>
#include <ctype.h>
#include <math.h>
#define defaultInput "../../Inputs/2021_16.txt"
#include "../../Modules/input.h"


int bin2int(char *b, int len) {
    int n = 0;

    for (int i = 0; i < len; i++) {
        n *= 2;
        n += b[i] == '1' ? 1 : 0;
    }

    return n;
}


char *parse_packet(char *packet, int *version_sum, long long int *packet_value) {
    int version = bin2int(packet, 3);
    int type_id = bin2int(packet + 3, 3);

    *version_sum += version;

    char *p = packet + 6;

    if (type_id == 4) {
        *packet_value = 0;
        bool continuing = true;

        while (continuing) {
            continuing = p[0] == '1';
            p++;
            *packet_value <<= 4;
            *packet_value += bin2int(p, 4);
            p += 4;
        }
    } else {
        int length_type = p[0] - '0';
        p++;

        long long int *sub_vals = (long long int*)calloc(1, sizeof(long long int));
        int allocated = 1, ix = 0;
        if (length_type == 0) {
            int length = bin2int(p, 15);
            p += 15;

            while (length > 0) {
                while (ix >= allocated) {
                    sub_vals = (long long int*)realloc(sub_vals, 2 * allocated * sizeof(long long int));
                    allocated *= 2;
                }

                int p_len = strlen(p);
                p = parse_packet(p, version_sum, sub_vals + ix);
                ix++;
                length -= p_len - strlen(p);
            }
        } else {
            int num_subpackets = bin2int(p, 11);
            p += 11;

            while (num_subpackets > 0) {
                while (ix >= allocated) {
                    sub_vals = (long long int*)realloc(sub_vals, 2 * allocated * sizeof(long long int));
                    allocated *= 2;
                }

                p = parse_packet(p, version_sum, sub_vals + ix);
                ix++;
                num_subpackets--;
            }
        }

        switch (type_id) {
            case 0:
                *packet_value = 0;
                for (int i = 0; i < ix; i++) {
                    *packet_value += sub_vals[i];
                }
                break;
            case 1:
                *packet_value = 1;
                for (int i = 0; i < ix; i++) {
                    *packet_value *= sub_vals[i];
                }
                break;
            case 2:
                *packet_value = sub_vals[0];
                for (int i = 1; i < ix; i++) {
                    if (sub_vals[i] < *packet_value) {
                        *packet_value = sub_vals[i];
                    }
                }
                break;
            case 3:
                *packet_value = sub_vals[0];
                for (int i = 1; i < ix; i++) {
                    if (sub_vals[i] > *packet_value) {
                        *packet_value = sub_vals[i];
                    }
                }
                break;
            case 5:
                *packet_value = sub_vals[0] > sub_vals[1];
                break;
            case 6:
                *packet_value = sub_vals[0] < sub_vals[1];
                break;
            case 7:
                *packet_value = sub_vals[0] == sub_vals[1];
                break;
        }
    }

    return p;
}

int part1(char *fileName) {
    FILE *fp = fopen(fileName, "r");
    if (!fp) {
        return -1;
    }

    char *input_line = (char*)calloc(2, sizeof(char));
    int ix = 0, max_chars = 1;

    while (!feof(fp)) {
        char c = fgetc(fp);

        if (c != -1) {
            int n = c - '0';
            if (n > 9) {
                n = c - 'A' + 10;
            }

            char *bits = (char*)calloc(5, sizeof(char));
            for (int i = 3; i >= 0; i--) {
                bits[i] = (n % 2) + '0';
                n /= 2;
            }

            while ((ix + 1) * 4 >= max_chars) {
                input_line = (char*)realloc(input_line, (2 * max_chars + 1) * sizeof(char));
                max_chars *= 2;
            }

            strncat(input_line, bits, 4 * sizeof(char));
            ix++;
        }
    }

    int *vSum = (int*)calloc(1, sizeof(int));
    long long int *packetValue = (long long int*)calloc(1, sizeof(long long int));
    parse_packet(input_line, vSum, packetValue);
    return *vSum;
}

long long int part2(char *fileName) {
    FILE *fp = fopen(fileName, "r");
    if (!fp) {
        return -1;
    }

    char *input_line = (char*)calloc(2, sizeof(char));
    int ix = 0, max_chars = 1;

    while (!feof(fp)) {
        char c = fgetc(fp);

        if (c != -1) {
            int n = c - '0';
            if (n > 9) {
                n = c - 'A' + 10;
            }

            char *bits = (char*)calloc(5, sizeof(char));
            for (int i = 3; i >= 0; i--) {
                bits[i] = (n % 2) + '0';
                n /= 2;
            }

            while ((ix + 1) * 4 >= max_chars) {
                input_line = (char*)realloc(input_line, (2 * max_chars + 1) * sizeof(char));
                max_chars *= 2;
            }

            strncat(input_line, bits, 4 * sizeof(char));
            ix++;
        }
    }

    int *vSum = (int*)calloc(1, sizeof(int));
    long long int *packetValue = (long long int*)calloc(1, sizeof(long long int));
    parse_packet(input_line, vSum, packetValue);
    return *packetValue;
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
    printf("\nPart 1:\nSum of version numbers: %d\nRan in %f seconds\n", p1, t_p1);

    t = clock(); 
    long long int p2 = part2(inputPath);
    t = clock() - t;
    double t_p2 = ((double)t) / CLOCKS_PER_SEC;
    printf("\nPart 2:\nPacket Value: %lld\nRan in %f seconds\n", p2, t_p2);

    return 0;
}