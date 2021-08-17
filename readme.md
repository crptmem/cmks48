<p align="center">
<img src="https://img.shields.io/github/contributors/NeticTeam/AirKernel.svg"/>
<img src="https://img.shields.io/github/forks/NeticTeam/AirKernel.svg"/>
<img src="https://img.shields.io/github/stars/NeticTeam/AirKernel.svg"/>
<img src="https://img.shields.io/github/issues/NeticTeam/AirKernel.svg"/>
<img src="https://github.com/NeticTeam/AirKernel/actions/workflows/build.yml/badge.svg"/>
<img src="https://app.codacy.com/project/badge/Grade/fd5a7833d434455e8c455fce709f0b78"/>
</p>
  <h3 align="center">AirKernel</h3>
  <p align="center">
  ⚡ Kernel written for NeticOS (UEFI bootloader)
  <br/>
  <a href="https://github.com/NeticTeam/AirKernel/issues/new?labels=bug">Report bug</a>
  |
  <a href="https://github.com/NeticTeam/AirKernel/issues/new?labels=feature">Request a feature</a>
  </p>
<br/>


## ❗️ Requirements
### Debian/Debian-based
```sh
sudo apt install gcc g++ qemu qemu-system make mtools nasm git moreutils
```
### Arch/Arch-based
```sh
sudo pacman -S base-devel qemu make mtools nasm git moreutils
```
### Gentoo
```sh
sudo emerge --ask app-emulation/qemu sys-fs/mtools dev-vcs/git dev-lang/nasm sys-apps/moreutils
```

## ❓ How to run
```sh
make
make run
```

## Problems
### The 2100 year problem
At this time we using strcat(ret, "20"); in rtc.c, so current year (2021, but dt->year returns 21) will be 2021. But on January 1 2100 outputted time will be like Jun 1 2000.

## 📚 Credits
Kernel Base: [PonchoOS](https://github.com/absurdponcho/ponchoos)

Tutorials: [OSDev](https://wiki.osdev.org)

## 👥 Developers
Leader: [z3r0memory](https://github.com/z3r0memory)

Maintainer: [TheAirBlow](https://github.com/theairblow)
