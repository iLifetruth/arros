#![no_std]
#![no_main]

extern crate stm32f4;
extern crate panic_halt;
extern crate cortex_m_rt;

use cortex_m_rt::entry;
use stm32f4::stm32f407;

#[entry]
fn main() -> ! {
    // get handles to the hardware
    let peripherals = stm32f407::Peripherals::take().unwrap();
    let _gpiod = &peripherals.GPIOD;
    let rcc = &peripherals.RCC;

    // enable system clock RCC for gpiof
    rcc.ahb1enr.write(|w|{
        w.gpiofen().set_bit()
    });

    // TODO: GPIO_init
    {
        
    }
    // Toggle the LEDs
    loop{
        // gpiod.odr.write(|w| {
        //     // Todo:
        // });
        cortex_m::asm::delay(168*10000*3);

        // gpiod.odr.write(|w| {
        //     // Todo:
        // });
        cortex_m::asm::delay(168*10000*3);
    }
}

