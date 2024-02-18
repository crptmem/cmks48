#ifndef LIB_H
#define LIB_H
#include <stdint.h>
#include <stdarg.h>

int mprintf(const char *str, ...); 
int memcmp(const void *v1, const void *v2, unsigned long n);

#endif // !LIB_H
