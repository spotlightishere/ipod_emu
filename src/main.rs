use unicorn_engine::unicorn_const::{Arch, HookType, Mode, Permission, SECOND_SCALE};
use unicorn_engine::RegisterARM;

mod debug;
mod hardware;
mod patches;

/// Where our code is running.
const BOOTROM_BASE: u64 = 0x20000000;
/// Where our global state is stored.
const STATE_BASE: u64 = 0x22020000;
/// Where our image is uploaded to.
const DFU_BASE: u64 = 0x22000000;

fn main() {
    let bootrom_bin = include_bytes!("../bootrom.bin");
    let bootrom_length = bootrom_bin.len();

    let mut engine = unicorn_engine::Unicorn::new(Arch::ARM, Mode::LITTLE_ENDIAN)
        .expect("failed to initialize Unicorn instance");

    // First, map in our actual BootROM code.
    engine
        .mem_map(BOOTROM_BASE, bootrom_length, Permission::ALL)
        .expect("failed to map BootROM");
    engine
        .mem_write(BOOTROM_BASE, bootrom_bin)
        .expect("failed to write instructions");

    // Next, we have where our normal osos is.
    // This is where our DFU image is uploaded.
    // It's... probably similar in length?
    engine
        .mem_map(DFU_BASE, 0x10000, Permission::ALL)
        .expect("failed to map DFU RAM");

    // We'll now map hardware.
    hardware::map_hardware(&mut engine);
    patches::add_patches(&mut engine);

    // Honestly, I have zero clue on what the SP should be.
    // We'll just assume it's towards the bottom here.
    // The upper half is used for global state.
    // TODO(spotlightishere): It'd be a really good idea to figure this out!
    engine
        .mem_map(STATE_BASE, 0x10000, Permission::ALL)
        .expect("failed to map stack RAM");
    engine
        .reg_write(RegisterARM::SP, STATE_BASE + 0xF000)
        .expect("should be able to force SP");

    // Finally, add some hooks for operations we're clueless on.
    engine
        .add_mem_hook(
            HookType::MEM_UNMAPPED,
            0x0,
            0xFFFFFFFF,
            debug::unexpected_write,
        )
        .expect("failed to add unmapped memory hook");

    engine
        .emu_start(
            // "SetUpGlobalState"
            0x20003aa0,
            (BOOTROM_BASE as usize + bootrom_length) as u64,
            10 * SECOND_SCALE,
            3000000,
        )
        .unwrap();
    debug::print_engine_state(&mut engine);
}
