use unicorn_engine::{unicorn_const::MemType, RegisterARM, Unicorn};

/// Dumps the engine's state.
// TODO(spotlightishere): There has to be a better way to do this...
pub fn print_engine_state(engine: &mut Unicorn<()>) {
    let r0 = engine
        .reg_read(RegisterARM::R0)
        .expect("should be able to get R0");
    let r1 = engine
        .reg_read(RegisterARM::R1)
        .expect("should be able to get R1");
    let r2 = engine
        .reg_read(RegisterARM::R2)
        .expect("should be able to get R2");
    let r3 = engine
        .reg_read(RegisterARM::R3)
        .expect("should be able to get R3");
    let r4 = engine
        .reg_read(RegisterARM::R4)
        .expect("should be able to get R4");
    let r5 = engine
        .reg_read(RegisterARM::R5)
        .expect("should be able to get R5");
    let r6 = engine
        .reg_read(RegisterARM::R6)
        .expect("should be able to get R6");
    let r7 = engine
        .reg_read(RegisterARM::R7)
        .expect("should be able to get R7");
    let r8 = engine
        .reg_read(RegisterARM::R8)
        .expect("should be able to get R8");
    let r9 = engine
        .reg_read(RegisterARM::R9)
        .expect("should be able to get R9");
    let r10 = engine
        .reg_read(RegisterARM::R11)
        .expect("should be able to get R10");
    let r11 = engine
        .reg_read(RegisterARM::R11)
        .expect("should be able to get R11");
    let r12 = engine
        .reg_read(RegisterARM::R12)
        .expect("should be able to get R12");
    let sp = engine
        .reg_read(RegisterARM::SP)
        .expect("should be able to get SP");
    let lr = engine
        .reg_read(RegisterARM::LR)
        .expect("should be able to get LR");
    let pc = engine
        .reg_read(RegisterARM::PC)
        .expect("should be able to get PC");

    println!("=== ENGINE STATE ===");
    println!("\tR0:  {:08x}\tR2:  {:08x}", r0, r1);
    println!("\tR2:  {:08x}\tR4:  {:08x}", r2, r3);
    println!("\tR4:  {:08x}\tR6:  {:08x}", r4, r5);
    println!("\tR6:  {:08x}\tR8:  {:08x}", r6, r7);
    println!("\tR8:  {:08x}\tR10: {:08x}", r8, r9);
    println!("\tR10: {:08x}\tR11: {:08x}", r10, r11);
    println!("\tR12: {:08x}", r12);
    println!();
    println!("\tCurrent PC: {:08x}", pc);
    println!("\tCurrent LR: {:08x}", lr);
    println!("\tCurrent SP: {:08x}", sp);
}

/// Called when an unmapped read, write, or fetch occurs.
/// We refuse to handle such.
pub fn unexpected_write(
    engine: &mut Unicorn<()>,
    mem_type: MemType,
    address: u64,
    size: usize,
    written_value: i64,
) -> bool {
    let access_type = match mem_type {
        MemType::READ_UNMAPPED => "READ",
        MemType::WRITE_UNMAPPED => "WRITE",
        MemType::FETCH_UNMAPPED => "FETCH",
        _ => todo!(),
    };
    let current_pc = engine.pc_read().expect("should be able to get PC");

    println!("UNMAPPED {} ENCOUNTERED!", access_type.to_ascii_uppercase());
    println!("\tCurrent PC:\t{:08x}", current_pc);
    println!("\tAddress:\t{:08x}", address);
    println!("\tAccess length:\t{}", size);
    println!("\tWritten value:\t{:08x}", written_value);
    false
}
