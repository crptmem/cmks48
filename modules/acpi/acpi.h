#ifndef ACPI_H
#define ACPI_H
#include <stdint.h>

struct xsdp {
 char signature[8];
 uint8_t checksum;
 char oemid[6];
 uint8_t revision;
 uint32_t rsdt_address;      // deprecated since version 2.0
 uint32_t length;
 uint64_t xsdt_address;
 uint8_t extended_checksum;
 uint8_t reserved[3];
} __attribute__ ((packed));

#endif // !ACPI_H
