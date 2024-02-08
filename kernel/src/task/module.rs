use elf::endian::AnyEndian;
use elf::ElfBytes;
use elf::note::Note;
use elf::note::NoteGnuBuildId;
use elf::section::SectionHeader;
use crate::init::init::Paging;
use crate::serial_println;
use crate::task::elf::load_elf;

pub fn load(name: &str, data: &[u8], paging: &mut Paging) {
    serial_println!("mod: loading, name={}", name);   
    load_elf(data, paging)
}
