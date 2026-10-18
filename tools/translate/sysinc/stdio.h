#ifndef _BT3_STDIO_H
#define _BT3_STDIO_H
#define SEEK_SET 0
#define SEEK_CUR 1
#define SEEK_END 2
#define EOF (-1)
#include <stddef.h>
#include <stdarg.h>
typedef struct _BT3_FILE FILE;
int printf(const char *, ...);
int sprintf(char *, const char *, ...);
int snprintf(char *, size_t, const char *, ...);
int vsprintf(char *, const char *, va_list);
int puts(const char *);
FILE *fopen(const char *, const char *); size_t fread(void *, size_t, size_t, FILE *); int fclose(FILE *); int fseek(FILE *, long, int); long ftell(FILE *); size_t fwrite(const void *, size_t, size_t, FILE *);
extern FILE *stderr; extern FILE *stdout;
int fprintf(FILE *, const char *, ...);
int vfprintf(FILE *, const char *, va_list);
#endif
