#include <stdint.h>
#include <stdarg.h>
#include <string.h>
#include "lib.h"
#include "acpi.h"

void _start(void (*symreg)(char[24], uint64_t *), uint64_t *(*getsym)(char[24])) {
  /** this fucking piece of shit is SO unsafe
    * i just cannot handle it anymore i hate c, rust and os development
    * BUT I CANT STOP DEVELOPING IT I AM SO ADDICTED
    * help
    */
  mprintf("acpi: acpi module v0.1 by corruptmemory  %s %s\n", __DATE__, __TIME__);
  asm volatile(
             "movq $0x60, %%rbx;"
              "int $0x80" 
              ::: "rdi", "rbx"
            );

}
