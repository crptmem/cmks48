use core::mem;
use crate::{serial_println, serial_print};

const PSF1_MAGIC: [u8;2] = [0x36, 0x04];
const PSF2_MAGIC: [u8;4] = [0x72, 0xb5, 0x4a, 0x86];

#[derive(Debug)]
pub enum Error {
	OutOfBounds,
	InvalidMagic,
}

#[repr(C, packed)]
pub struct PSF1Header {
	magic: [u8;2],
	mode: u8,
	char_size: u8,
}

#[repr(C, packed)]
struct PSF2Header {
	magic: [u8;4],
	version: u32,
	header_size: u32,
	flags: u32,
	length: u32,
	char_size: u32,
	height: u32,
	width: u32,
}

enum PSFFont<'a> {
	PSF1Font(PSF1Font<'a>),
	PSF2Font(PSF2Font<'a>),
}

impl<'a> PSFFont<'a> {
	fn parse(data: &'a [u8]) -> Result<Self, Error> {
		if data.len() < mem::size_of::<PSF1Header>() {
			return Err(Error::OutOfBounds);
		}
		
		let header = unsafe {
			let ref header = *(data.as_ptr() as *const PSF1Header);
			header
		};
		
		if header.magic == PSF1_MAGIC {
			return Ok(PSFFont::PSF1Font(PSF1Font::parse(data)?));
		}
		
		if data.len() < mem::size_of::<PSF2Header>() {
			return Err(Error::OutOfBounds);
		}
		
		let header = unsafe {
			let ref header = *(data.as_ptr() as *const PSF2Header);
			header
		};
		
		if header.magic == PSF2_MAGIC {
			return Ok(PSFFont::PSF2Font(PSF2Font::parse(data)?));
		}
		
		return Err(Error::InvalidMagic);
	}

	pub fn glyph_size(&self) -> (u32, u32) {
		match self {
			PSFFont::PSF1Font(font) => font.glyph_size(),
			PSFFont::PSF2Font(font) => font.glyph_size(),
		}
	}
	
	pub fn glyph_count(&self) -> u32 {
		match self {
			PSFFont::PSF1Font(font) => font.glyph_count(),
			PSFFont::PSF2Font(font) => font.glyph_count(),
		}
	}
	
	pub fn glyph(&self, index: u32) -> Option<&[u8]> {
		match self {
			PSFFont::PSF1Font(font) => font.glyph(index),
			PSFFont::PSF2Font(font) => font.glyph(index),
		}
	}
}

pub struct PSF1Font<'a> {
	data: &'a [u8],
	header: &'a PSF1Header,
}

impl<'a> PSF1Font<'a> {
    pub fn parse(data: &'a [u8]) -> Result<Self, Error> {
		if data.len() < mem::size_of::<PSF1Header>() {
			return Err(Error::OutOfBounds);
		}
		
		let header = unsafe {
			let ref header = *(data.as_ptr() as *const PSF1Header);
			header
		};
		
		if header.magic != PSF1_MAGIC {
			return Err(Error::InvalidMagic);
		}
		
		let last_glyph_pos = mem::size_of::<PSF1Header>() + header.char_size as usize * 256;
		if data.len() < last_glyph_pos {
			return Err(Error::OutOfBounds);
		}
		
		Ok(PSF1Font {
			data,
			header,
		})
	}
	
	pub fn glyph_size(&self) -> (u32, u32) {
		(8, self.header.char_size as u32)
	}
	
	pub fn glyph_count(&self) -> u32 {
		256
	}
	
	pub fn glyph(&self, index: u32) -> Option<&[u8]> {
		if index >= 256 {
			return None
		}
		
		let length = self.header.char_size as usize;
		let offset = mem::size_of::<PSF1Header>() + index as usize * length;
		Some(&self.data[offset..(offset+length)])
	}
}

pub struct PSF2Font<'a> {
	data: &'a [u8],
	header: &'a PSF2Header,
}

impl<'a> PSF2Font<'a> {
	pub fn parse(data: &'a [u8]) -> Result<Self, Error> {
		if data.len() < mem::size_of::<PSF2Header>() {
			return Err(Error::OutOfBounds);
		}
		
		let header = unsafe {
			let ref header = *(data.as_ptr() as *const PSF2Header);
			header
		};
		
		if header.magic != PSF2_MAGIC {
			return Err(Error::InvalidMagic);
		}
		
		let last_glyph_pos = header.header_size + header.char_size * (header.length - 1);
		if data.len() < last_glyph_pos as usize {
			return Err(Error::OutOfBounds);
		}
		
		Ok(PSF2Font {
			data,
			header,
		})
	}
	
	pub fn glyph_size(&self) -> (u32, u32) {
		(self.header.width, self.header.height)
	}
	
	pub fn glyph_count(&self) -> u32 {
		self.header.length
	}
	
	pub fn glyph(&self, index: u32) -> Option<&[u8]> {
		if index >= self.header.length {
			return None
		}
		
		let length = self.header.char_size as usize;
		let offset = self.header.header_size as usize + index as usize * length;
		Some(&self.data[offset..(offset+length)])
	}
}

pub fn print_bitmap(bitmap: &[u8], size: (u32, u32)) {
	let stride = (size.0 + 7) / 8;
	serial_println!("stride {}", stride);
	
	for y in 0..size.1 {
		for x in 0..size.0 {
			let byte = y * stride + (x / 8);
			let bit = 7 - (x % 8);
			
			if (bitmap[byte as usize] & (1 << bit)) == 0 {
				serial_print!("_");
			} else {
				serial_print!("#");
			}
		}
		serial_println!();
	}
}

