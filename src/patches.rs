use unicorn_engine::{RegisterARM, Unicorn};

/// Create hooks to augment BootROM functionality.
pub fn add_patches(engine: &mut Unicorn<()>) {
    // Skip over NAND initialization.
    // TODO(spotlightishere): Is this actually SPI?
    //
    // This function starts at 0x20003544 and ends at 0x200036a8,
    // but we'll skip over four so that we can leverage its prolog/epilog.
    engine
        .add_code_hook(0x20003548, 0x200036a4, skip_nand_initialization)
        .expect("should be able to add NAND patch");
}

fn skip_nand_initialization(engine: &mut Unicorn<()>, _: u64, _: u32) {
    // Go ahead and skip to the end.
    engine
        .reg_write(RegisterARM::R0, 0x0)
        .expect("should be able to set R0");
    engine.set_pc(0x200036a8).expect("should be able to set PC");
}
