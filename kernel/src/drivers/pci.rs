extern crate alloc;

use alloc::vec::Vec;

use crate::serial_println;

pub struct pci_device {
    pub vendor: u16,
    pub device: u16,
    pub class: u16,
    pub func: u16,
    pub subclass: u16
}

static mut PCI_DEVICES: Vec<pci_device> = Vec::new();

pub fn read_word(bus: u16, slot: u16, func: u16, offset: u16) -> u32 {
    let address: u32 = ((((bus as u32) << 16u32) | ((slot as u32) << 11u32) | ((func as u32) << 8u32) | ((offset as u32) & 0xFC)) as u32 | 0x80000000u32);
    unsafe {
        x86_64::instructions::port::PortWrite::write_to_port(0xCF8, address);
        <u32 as x86_64::instructions::port::PortRead>::read_from_port(0xCFC) >> ((offset & 2) * 8) & 0xffff
    }
}

fn get_vendor(bus: u16, slot: u16, func: u16) -> u32 {
    return read_word(bus, slot, func, 0)
}

fn get_device(bus: u16, slot: u16, func: u16) -> u32 {
    return read_word(bus, slot, func, 2)
}

fn get_class(bus: u16, slot: u16, func: u16) -> u32 {
    return (read_word(bus, slot, func, 10) & !0x00FF) >> 8
}

fn get_subclass(bus: u16, slot: u16, func: u16) -> u32 {
    return (read_word(bus, slot, func, 10) & !0xFF00)
}

pub fn init() {
   for bus in 0..256 {
       for slot in 0..32 {
           for function in 0..8 {
                let vendor = get_vendor(bus, slot, function);
                if vendor == 0xFFFF {
                    continue;
                };
                let device = get_device(bus, slot, function);
                let class = get_class(bus, slot, function);
                let subclass = get_subclass(bus, slot, function);
                serial_println!("pci: vendor={:#06x}, device={:#06x}, class={:#06x}, subclass={:#06x}", vendor, device, class, subclass);
           }
       }
   } 
}
