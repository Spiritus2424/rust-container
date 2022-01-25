#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use stm32f0::stm32f0x2::Peripherals;

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();

    let peripherals = Peripherals::take().unwrap();
    let gpioa = peripherals.GPIOA;

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    // debug::exit(debug::EXIT_SUCCESS);
    let reader = gpioa.odr.read();

    loop {
        reader.odr0().bit();
        gpioa.odr.write(|writer| writer.odr0().set_bit());
    }
}
