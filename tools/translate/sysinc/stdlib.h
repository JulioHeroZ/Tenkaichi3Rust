#ifndef _BT3_STDLIB_H
#define _BT3_STDLIB_H
#include <stddef.h>
void *malloc(size_t); void free(void *); void *calloc(size_t, size_t);
int abs(int); int atoi(const char *); long strtol(const char *, char **, int);
void exit(int); void qsort(void *, size_t, size_t, int (*)(const void *, const void *));
int rand(void); char *getenv(const char *); void srand(unsigned);
#endif
