use unicorn_engine::Unicorn;

pub const USB_PHY_BASE: u64 = 0x3c400000;

pub fn usb_phy_read(_: &mut Unicorn<()>, address: u64, _size: usize) -> u64 {
    println!("[USB PHY] Block was read from!");
    println!("\tAddress\t{:08x}", USB_PHY_BASE + address);

    // TODO(spotlightishere): Implement USB
    //
    // For now, we'll have 0x3c400028 return 0x1.
    // It's used to indicate DFU.
    if address == 0x28 {
        1
    } else {
        0
    }
}

pub fn usb_phy_write(_: &mut Unicorn<()>, address: u64, _size: usize, value: u64) {
    println!("[USB PHY] Block was written to!");
    println!("\tAddress\t{:08x}", USB_PHY_BASE + address);
    println!("\tValue\t{:08x}", value);
}
