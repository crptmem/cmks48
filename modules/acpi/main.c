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

  uint64_t *(*get_rsdp)() = getsym("get_rsdp"); 
  uint64_t *(*get_paging)() = getsym("get_paging");
  uint64_t *(*mmap)(uint64_t *, uint64_t *, uint64_t *) = getsym("memory_map");

  uint64_t *rsdp = get_rsdp();
  mmap(rsdp, rsdp, get_paging());
  struct xsdp *xsdt = (struct xsdp *)get_rsdp();
  mprintf("acpi: rsdp=0x%x, xsdt_address=0x%x, oemid=%s\n", get_rsdp(), xsdt->xsdt_address, xsdt->oemid);
}
