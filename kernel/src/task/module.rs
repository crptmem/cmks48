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
    let file = ElfBytes::<AnyEndian>::minimal_parse(data).expect("chego blya");
    let common_sections = file.find_common_data().unwrap();
    load_elf(data, paging)
}
