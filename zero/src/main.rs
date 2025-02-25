#![no_std]
#![no_main]

use core::arch::global_asm;

mod system;
mod panic;

global_asm!(include_str!("start.s"));

#[unsafe(no_mangle)]
pub extern "C" fn not_main() {

    system::log::print("\n\n booting 00 \n\n");
}

