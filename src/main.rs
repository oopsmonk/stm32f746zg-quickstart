#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
use stm32f7::stm32f7x6::Peripherals;

// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
// use cortex_m_semihosting::hprintln; // for openODC debug

#[entry]
fn main() -> ! {
    // print text on openOCD terminal
    // hprintln!("Hello, world!").unwrap();

    // get he hardware handler
    let peripherals = Peripherals::take().unwrap();
    let gpio_b = &peripherals.GPIOB;
    let rcc = &peripherals.RCC;

    // GPIO must be driven by AHB1 clock
    // see section 5.3.10 in dm00124865
    // https://stm32-rs.github.io/stm32-rs/STM32F7x6.html#GPIOB
    // Enable clock for GPIOB
    rcc.ahb1enr.write(|w| w.gpioben().set_bit());

    // Configure LED1 on pin PB0 as output
    gpio_b.moder.modify(|_, w| w.moder0().output());
    // Configure LED2 on pin PB7 as output
    gpio_b.moder.modify(|_, w| w.moder7().output());
    // Configure LED3 on pin PB14 as output
    gpio_b.moder.modify(|_, w| w.moder14().output());

    loop {
        // toggle LEDs
        gpio_b
            .odr
            .modify(|r, w| w.odr0().bit(r.odr0().bit_is_clear()));
        gpio_b
            .odr
            .modify(|r, w| w.odr7().bit(r.odr7().bit_is_clear()));
        gpio_b
            .odr
            .modify(|r, w| w.odr14().bit(r.odr14().bit_is_clear()));

        // clock delay
        asm::delay(9_000_000);
        // nop delay
        // for _i in 0..5000 {
        //      asm::nop()
        // }
    }
}
