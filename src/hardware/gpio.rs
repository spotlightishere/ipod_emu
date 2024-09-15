use std::sync::{LazyLock, Mutex};

use unicorn_engine::Unicorn;

/// The base address of the GPIO block.
pub const GPIO_BASE: u64 = 0x3cf00000;

/// Similar to the first-generation iPod touch, we have 12 (0xc) pins per GPIO pad.
const GPIO_PAD_COUNT: u32 = 12;

// Once again, similar to the first-generation iPod touch, we have 32 (0x20) GPIO pins.
const GPIO_PIN_COUNT: usize = 32;

/// Global state for GPIO pads.
static GPIO_STATE: LazyLock<Mutex<[u32; GPIO_PIN_COUNT]>> =
    LazyLock::new(|| Mutex::new([0; GPIO_PIN_COUNT]));

// We expect one GPIO pad to have 32 pins.
// Divide the address to find its respective pad.
fn find_gpio_pad(address: u64) -> u32 {
    (address / 0x20) as u32
}

pub fn gpio_read(_: &mut Unicorn<()>, address: u64, _size: usize) -> u64 {
    println!("[GPIO] Block was read from!");
    println!("\tAddress\t{:08x}", address);

    // Please refer to the opening comment within `gpio_write`
    // for why we return on values greater than our known pad count.
    //
    // TODO(spotlightishere): Some things read at non-0x20 aligned addresses (e.g. 0x174).
    // Why?
    let gpio_pad = find_gpio_pad(address);
    if gpio_pad > GPIO_PAD_COUNT {
        return 0;
    }

    // As a special case, for pad 0xc, we will permanently
    // return a non-zero value in order to emulate DFU.
    if gpio_pad == 0xb {
        println!("\tReturning DFU mode.");
        return 0xFFFFFFFFFFFFFFFF;
    }

    let current_state = GPIO_STATE.lock().unwrap()[gpio_pad as usize];
    println!("\tState\t{:08x}", current_state);

    current_state as u64
}

pub fn gpio_write(_: &mut Unicorn<()>, address: u64, _size: usize, value: u64) {
    println!("[GPIO] Block was written to!");
    println!("\tAddress\t{:08x}", address);

    // TODO(spotlightishere): It seems that that values above may be
    // handled differently beyond our 0xc pads.
    //
    // The value 0x000b0300 is written to the address 0x3cf001e0
    // by the function at 0x20001dcc.
    // 0xb is the pad, 0x03 is likely the pin, and 0x00 must be the state.
    // (Perhaps this is to indicate direction?)
    //
    // For now, we'll only handle writes corresponding directly to our pads.
    // (For the purpose of this program, we only need to handle 0xb for DFU.)
    let gpio_pad = find_gpio_pad(address);
    // Ignore unknown pads for now.
    if gpio_pad > GPIO_PAD_COUNT {
        return;
    }
    println!("\tPad\t{:x}", gpio_pad);
    println!("\tValue\t{:08x}", value);

    GPIO_STATE.lock().unwrap()[gpio_pad as usize] = value as u32;
}
