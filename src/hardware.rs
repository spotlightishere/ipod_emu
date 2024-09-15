use unicorn_engine::Unicorn;

////////////////////
// Generic Blocks //
////////////////////

/// Sets up a MMIO mapping with anonymous closures for read/write.
macro_rules! map_generic {
    ($engine:ident, $block_name:expr, $base_address:expr) => {
        $engine.mmio_map(
            // Base address
            $base_address,
            // Length
            0x10000,
            // Read callback
            Some(|engine: &mut Unicorn<()>, address: u64, size: usize| -> u64 {
                let current_pc = engine.pc_read().expect("should be able to get current PC");
                println!(
                    "[READ {}] PC {:08x} to address {:08x} for size {:08x}",
                    $block_name,
                    current_pc,
                    ($base_address + address),
                    size
                );
                0
            }),
            // Write callback
            Some(|engine: &mut Unicorn<()>, address: u64, size: usize, value: u64| {
                let current_pc = engine.pc_read().expect("should be able to get current PC");
                println!(
                    "[WRITE {}] PC {:08x}, writing to {:08x} for size {:08x} and value {:08x}",
                    $block_name,
                    current_pc,
                    ($base_address + address),
                    size,
                    value
                )
            })
        ).expect(format!("should be able to map {}", $block_name).as_str());
    };
}

/////////////
// Chip ID //
/////////////
const CHIPID_BASE: u64 = 0x3d100000;

fn chipid_read(_: &mut Unicorn<()>, address: u64, _size: usize) -> u64 {
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
        _ => panic!("[CHIP ID] Unknown write to {}", CHIPID_BASE + address),
    }
}

fn chipid_write(_: &mut Unicorn<()>, address: u64, size: usize, value: u64) {
    println!("[CHIP ID] Block was written to!");
    println!("\tAddress\t{:08x}", address);
    println!("\tSize\t{:08x}", size);
    println!("\tValue\t{:08x}", value);
    panic!("expected no writes to Chip ID block")
}

////////////////////
// Actual mapping //
////////////////////

/// Sets up mapped hardware.
pub fn map_hardware(engine: &mut Unicorn<()>) {
    // Some block types we don't care about.
    // Vector interrupt controller
    map_generic!(engine, "VIC0", 0x38e00000);
    // Watchdog Timer
    map_generic!(engine, "WDT", 0x3c800000);
    // Clock generator
    map_generic!(engine, "CLCKGEN", 0x3c500000);
    // TODO(spotlightishere): Maybe we do want to care about this?
    map_generic!(engine, "POWER", 0x39700000);
    map_generic!(engine, "GPIO", 0x3cf00000);
    // TODO(spotlightishere): What is this?
    map_generic!(engine, "UNKNOWN1", 0x3e700000);
    map_generic!(engine, "TIMER", 0x3c700000);
    map_generic!(engine, "NAND", 0x38a00000);
    map_generic!(engine, "SPI0", 0x3c300000);
    map_generic!(engine, "AES", 0x38c00000);
    map_generic!(engine, "SHA1", 0x38000000);
    map_generic!(engine, "USB_PHY", 0x3c400000);

    // We do want to specially handle Chip ID reads and writes.
    engine
        .mmio_map(CHIPID_BASE, 0x10000, Some(chipid_read), Some(chipid_write))
        .expect("should be able to map chip ID");
}
