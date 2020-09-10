#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
// pick a panicking behavior
use cortex_m_semihosting::hprintln;
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use stm32f7::stm32f7x6::Peripherals; // text on openODC console

// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

#[entry]
fn main() -> ! {
    // print text on openODC console
    hprintln!("Hello, world!").unwrap();

    // get he hardware handler
    let peripherals = Peripherals::take().unwrap();
    let gpio_b = &peripherals.GPIOB;
    let rcc = &peripherals.RCC;

    // GPIO must be driven by AHB1 clock
    // see section 5.3.10 in dm00124865
    // https://stm32-rs.github.io/stm32-rs/STM32F7x6.html#GPIOB
    // Enable clock for GPIOB
    rcc.ahb1enr.write(|w| w.gpioben().set_bit());

    // Configure LEDs on pin PB0, PB7, PB14 as output
    // Only modify pins that used by LEDs
    gpio_b
        .moder
        .modify(|_, w| w.moder0().output().moder7().output().moder14().output());

    loop {
        // toggle LEDs
        gpio_b.odr.modify(|r, w| {
            w.odr0()
                .bit(r.odr0().bit_is_clear())
                .odr7()
                .bit(r.odr7().bit_is_clear())
                .odr14()
                .bit(r.odr14().bit_is_clear())
        });

        // clock delay
        asm::delay(9_000_000);
        // nop delay
        // for _i in 0..5000 {
        //      asm::nop()
        // }
    }
}
