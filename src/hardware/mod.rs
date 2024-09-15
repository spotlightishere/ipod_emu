use unicorn_engine::Unicorn;

mod chipid;
mod gpio;
mod usb_phy;

/////////////////////////
// Registration Macros //
/////////////////////////

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
        ).expect(concat!("should be able to map ", $block_name));
    };
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
    // TODO(spotlightishere): What is this?
    map_generic!(engine, "UNKNOWN1", 0x3e700000);
    map_generic!(engine, "TIMER", 0x3c700000);
    map_generic!(engine, "NAND", 0x38a00000);
    map_generic!(engine, "SPI0", 0x3c300000);
    map_generic!(engine, "AES", 0x38c00000);
    map_generic!(engine, "SHA1", 0x38000000);

    // TODO(spotlightishere): This may get long very quickly...
    // Let's find a way to eventually migrate it to a macro.
    engine
        .mmio_map(
            chipid::CHIPID_BASE,
            0x10000,
            Some(chipid::chipid_read),
            Some(chipid::chipid_write),
        )
        .expect("should be able to map CHIPID");

    engine
        .mmio_map(
            gpio::GPIO_BASE,
            0x10000,
            Some(gpio::gpio_read),
            Some(gpio::gpio_write),
        )
        .expect("should be able to map GPIO");

    engine
        .mmio_map(
            usb_phy::USB_PHY_BASE,
            0x10000,
            Some(usb_phy::usb_phy_read),
            Some(usb_phy::usb_phy_write),
        )
        .expect("should be able to map GPIO");
}
