use core::ptr;

use bootloader_api::info::FrameBufferInfo;

use crate::{serial_println, FRAMEBUFFER};

use super::psf::PSF1Font;

pub fn init(fb_info: FrameBufferInfo) {
    serial_println!("GOP: Initializing framebuffer: {:#?}", fb_info);
    for x in 0..fb_info.width {
        for y in 0..fb_info.height {
            put_pixel(x as u64, y as u64, 0x000000u64);
        }
    }
}

pub fn put_pixel(x: u64, y: u64, color: u64) {
    let pixel: u64 = FRAMEBUFFER + y * (1280 * 3) + x * 4;
    unsafe { ptr::write(pixel as *mut u64, color); }
}
