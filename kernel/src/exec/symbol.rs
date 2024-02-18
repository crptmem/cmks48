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

fn convert_slice_to_array(slice: &[u8]) -> [u8; 24] {
    let mut array = [0u8; 24];
    let len = slice.len().min(24); // Get the minimum of slice length and 24
    array[..len].copy_from_slice(&slice[..len]);
    array
}

pub fn symbol_register(name: &[u8], ptr: *mut u64) {
    serial_println!("symbol: name={:?} ptr={:?}", name.as_ascii().unwrap(), ptr);
    serial_println!("{:?}", convert_slice_to_array(name));
    unsafe { SYMBOLS.push(Symbol {
        name: convert_slice_to_array(name),
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
            break;
        }
    } /* i feel like writing in C again */

    unsafe { 
        let sym = SYMBOLS.iter().find(|r| r.name.starts_with(new_name)).unwrap();
        return sym.ptr;
    }
}
