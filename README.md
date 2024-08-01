# ipod_emu
A testing environment for toying around with an iPod nano 7th generation's BootROM.
Vaguely inspired by similar projects floating about.

## Setup
You can use this Rust code as-is, or use the included Nix development environment via `nix develop`.

You'll need to dump the embedded ROM within the SoC in order to leverage this program. OK Google, `securerom fun "s5l8740" rom boot`.