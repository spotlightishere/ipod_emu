use unicorn_engine::Unicorn;

pub const CHIPID_BASE: u64 = 0x3d100000;

pub fn chipid_read(_: &mut Unicorn<()>, address: u64, _size: usize) -> u64 {
    // Values taken from a 2009 iPod nano 5th generation.
    // TODO(spotlightishere): Update from a genuine 7th generation.
    //
    // The values are also listed far more clearly
    // within the ChipId EFI driver within diagnostics.
    match address {
        // Possibly "Enabled"? Seems to be checked
        // to determine whether on a physical CPU
        // or running under e.g. ARM RealView.
        // 0x00 => 0x00000001,
        // Unknown information. Contains security fusing.
        0x04 => 0x19000011,
        // CPU information: type, stepping, revision.
        0x08 => 0x8740000B,
        // The device's 40-bit ECID.
        // We're not going to hardcode a real one.
        0x0C => 0x00000000,
        0x10 => 0x00000000,
        // Unknown.
        0x14 => 0x00000004,
        _ => panic!("[CHIP ID] Unknown read to {}", CHIPID_BASE + address),
    }
}

pub fn chipid_write(_: &mut Unicorn<()>, address: u64, size: usize, value: u64) {
    println!("[CHIP ID] Block was written to!");
    println!("\tAddress\t{:08x}", CHIPID_BASE + address);
    println!("\tSize\t{:08x}", size);
    println!("\tValue\t{:08x}", value);
    panic!("expected no writes to Chip ID block")
}
