extern crate alloc;
use crate::serial_println;
use alloc::vec::Vec;

#[derive(Debug)]
struct Symbol {
    name: [u8; 24],
    ptr: *mut u64
}

static mut SYMBOLS: Vec<Symbol> = Vec::new();

pub fn symbol_register(name: [u8; 24], ptr: u64) {
    serial_println!("symbol_register: name={:?} ptr={:#016x}", name.as_ascii().unwrap(), ptr);
    unsafe { SYMBOLS.push(Symbol {
        name,
        ptr: ptr as *mut u64
    }); }
}

pub fn get_symbol_ptr(name: &[u8]) -> *mut u64 {
    unsafe { 
        let sym = SYMBOLS.iter().find(|r| r.name.starts_with(name)).unwrap();
        return sym.ptr;
    }
}
