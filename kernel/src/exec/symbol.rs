extern crate alloc;
use crate::serial_println;
use alloc::vec::Vec;
use core::ascii::Char;

#[derive(Debug)]
struct Symbol {
    name: [u8; 24],
    ptr: *mut u64
}

static mut SYMBOLS: Vec<Symbol> = Vec::new();

pub fn symbol_register(name: [u8; 24], ptr: u64) {
    serial_println!("symbol: name={:?} ptr={:#016x}", name.as_ascii().unwrap(), ptr);
    unsafe { SYMBOLS.push(Symbol {
        name,
        ptr: ptr as *mut u64
    }); }
}

pub fn get_symbol_ptr(name: &[u8]) -> *mut u64 {
    let mut new_name = name;
    if name.len() >= 24 {
        new_name = &name[..24];
    }
    let mut i = 0;
    for c in new_name {
        i += 1;
        if *c == '\0'.as_ascii().unwrap().to_u8() {
            new_name = &new_name[..i - 1];
            serial_println!("terminating zero found");
            break;
        }
    } /* i feel like writing in C again */

    serial_println!("{:?}", new_name.as_ascii());
    unsafe { 
        let sym = SYMBOLS.iter().find(|r| r.name.starts_with(new_name)).unwrap();
        return sym.ptr;
    }
}
