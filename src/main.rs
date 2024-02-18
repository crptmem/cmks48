fn main() {
    // read env variables that were set in build script
    let uefi_path = env!("UEFI_PATH");
    
    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
    cmd.arg("-drive").arg(format!("format=raw,file={uefi_path}"));
    cmd.arg("-serial").arg("stdio");
    cmd.arg("-device").arg("rtl8139");
    cmd.arg("-device").arg("virtio-vga");
    cmd.arg("-m").arg("512M");
    //cmd.arg("-device").arg("piix3-ide,id=ide").arg("-drive").arg("id=disk,file=hdb.img,format=raw,if=none").arg("-device").arg("ide-hd,drive=disk,bus=ide.0");
    //cmd.arg("-d").arg("int");
    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();    
}
