CMKS
====

An x64 kernel for UEFI systems written in Rust

What have been done
===================
  * UART
  * PCI
  * Ramdisk
  * Paging
  * Allocator
  * IDT & GDT
  * PSF parsing
  * ACPI (partial)
  * GOP (partial)

Roadmap
=======
  x ELF loader
  x Multitasking
  x Modules
  x Full ACPI support (AML, MADT, DSDT, other tables...)
  x Full GOP support (print to screen)

Run
===
  $ cargo run
