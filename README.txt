CMKS
====

An x64 kernel for UEFI systems written in Rust
Kernel is written in Rust and modules in C.

What have been done
===================
  * UART
  * PCI
  * Ramdisk
  * Paging
  * Allocator
  * IDT & GDT
  * ELF loader
  * PSF parsing
  * Modules
  * ACPI (partial)
  * GOP (partial)

Roadmap
=======
  x Multitasking
  x Basic network support
  x Full ACPI support (AML, MADT, DSDT, other tables...)
  x Full GOP support (print to screen)

Build ramdisk (required)
========================
  $ ./build_ramdisk.sh

Run
===
  $ cargo run
