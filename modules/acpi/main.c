#include <stdint.h>
#include <stdarg.h>
#include <string.h>
#include "lib.h"

void _start(void (*symreg)(char[24], uint64_t *), uint64_t *(*getsym)(char[24])) {
  exit(0);
}
