# `stm32f746zg-quickstart`

> A template for building applications for NUCLEO-F746ZG(STM32F746ZG)

This project is created for NUCLEO-F746ZG dev board based on [cortex-m-quickstart](https://github.com/rust-embedded/cortex-m-quickstart). It shows `Hello, world!` on openOCD console and blinks user LEDs on the target board.

The STM32 **NUCLEO-F746ZG** board is a low-cost and easy-to-use development kit, used to evaluate and start a development quickly with an STM32 microcontroller in LQFP144 package. it does not require any separate probe as it integrates the ST-LINK/V2-1 debugger/programmer.

Hardware:  
* ARM 32-bit Cortex-M7 CPU with FPU
* 216 MHz max CPU frequency
* 3 user LEDs connected on `PB0`, `PB7`, `PB14` pins
* VDD from 1.7 V to 3.6 V
* 1 MB Flash and 320 KB SRAM
* 16-bit timers(`10`) and 32-bit timers(`2`)
* SPI(`6`), I2C(`4`), I2S (`3`), USART(`4`), UART(`4`)
* USB OTG Full Speed(`1`) and High Speed(`1`)
* CAN(`2`), SAI(`2`), SPDIF_Rx(`4`), HDMI_CEC(`1`)
* Dual Mode Quad SPI(`1`)
* Camera Interface
* GPIO(up to 168) with external interrupt capability
* 12-bit ADC(`3`) with 24 channels / 2.4 MSPS
* 12-bit DAC with 2 channels(`2`)
* True Random Number Generator (RNG)
* 16-channel DMA
* LCD-TFT Controller with XGA(1024x768) resolution, 2 display layers with FIFO (64x32-bit)

![](https://i.imgur.com/slfzsqf.png)

[STM32F746ZG Datasheet](https://www.st.com/resource/en/datasheet/stm32f746zg.pdf)  
[Reference manual STM32F75xxx and STM32F74xxx](https://www.st.com/resource/en/reference_manual/dm00124865.pdf)  
[User manual STM32 Nucleo-144 boards](https://www.st.com/resource/en/user_manual/dm00244518.pdf)  
[Rust STM32F7x6 Peripheral Coverage](https://stm32-rs.github.io/stm32-rs/STM32F7x6.html)  

## Dependencies

**NOTE**: This is the very short version that only covers building programs. For
the long version, which additionally covers flashing, running and debugging
programs, check [the embedded Rust book][book] and [Rust STM32F3Discovery][discovery].

[book]: https://rust-embedded.github.io/book
[discovery]: https://docs.rust-embedded.org/discovery

To build embedded programs using this template you'll need:

- OpenOCD and ARM GDB. Run:

```bash
$ sudo apt install gdb-multiarch openocd
```

- Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain. e.g. `rustup
  default beta`

- The `cargo generate` subcommand. [Installation
  instructions](https://github.com/ashleygwilliams/cargo-generate#installation).

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  targets. Run:

``` bash
$ rustup target add thumbv7em-none-eabihf
```

## Using this template

1. Instantiate the template.

``` bash
$ cargo generate --git https://github.com/oopsmonk/stm32f746zg-quickstart
 Project Name: app
 Creating project called `app`...
 Done! New project created /tmp/app

$ cd app
```

2. Make sure compilation target is `thumbv7em-none-eabihf` in the `.cargo/config` file.

``` bash
$ tail -n6 .cargo/config
```

``` toml
[build]
# Pick ONE of these compilation targets
# target = "thumbv6m-none-eabi"    # Cortex-M0 and Cortex-M0+
# target = "thumbv7m-none-eabi"    # Cortex-M3
# target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)
```

3. Check memory region information into the `memory.x` file.

``` bash
$ cat memory.x
/* Linker script for the STM32F746ZG */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* NUCLEO-F746ZG memory layout */
  /* Flash 1M bytes on AXIM interface */
  FLASH : ORIGIN = 0x08000000, LENGTH = 1M
  /* System SRAM 320K = DTCM(64K) + SRAM1(240K) + SRAM2(16K) */
  RAM : ORIGIN = 0x20000000, LENGTH = 320K
}
```

![](https://i.imgur.com/fpvlhVp.png)

4. Build the template application or one of the examples.

``` bash
$ cargo build
```

5. Debug on the target board.

You will need two consoles to run OpenOCD and application.  

Runs openOCD in the root of this project where the location of `openocd.cfg` and `openocd.gdb` files. 

```bash
$ sudo openocd
Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
        http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
adapter speed: 2000 kHz
adapter_nsrst_delay: 100
srst_only separate srst_nogate srst_open_drain connect_deassert_srst
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : clock speed 1800 kHz
Info : STLINK v2 JTAG v29 API v2 SWIM v18 VID 0x0483 PID 0x374B
Info : using stlink api v2
Info : Target voltage: 3.233071
Warn : Silicon bug: single stepping will enter pending exception handler!
Info : stm32f7x.cpu: hardware has 8 breakpoints, 4 watchpoints
```

Runs application in another console.  

```bash
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `gdb-multiarch -q -x openocd.gdb target/thumbv7em-none-eabihf/debug/stm32f746zg-quickstart`
Reading symbols from target/thumbv7em-none-eabihf/debug/stm32f746zg-quickstart...done.
0x00000000 in ?? ()
Breakpoint 1 at 0x80012ac: file /home/sam/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs, line 570.
Breakpoint 2 at 0x80014c4: file /home/sam/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs, line 560.
Breakpoint 3 at 0x800104c: file /home/sam/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-halt-0.2.0/src/lib.rs, line 32.
Breakpoint 4 at 0x80004d0: file src/main.rs, line 14.
semihosting is enabled
Loading section .vector_table, size 0x400 lma 0x8000000
Loading section .text, size 0x10d0 lma 0x8000400
Loading section .rodata, size 0x348 lma 0x80014d0
Start address 0x8000400, load size 6168
Transfer rate: 8 KB/sec, 2056 bytes/write.
Note: automatically using hardware breakpoints for read-only addresses.
0x08000402 in cortex_m_rt::Reset () at /home/sam/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.13/src/lib.rs:497
497     pub unsafe extern "C" fn Reset() -> ! {
(gdb) c
Continuing.

Breakpoint 4, main () at src/main.rs:14
14      #[entry]
(gdb) c
```

You will see the output from openocd terminal and user LEDs are blinking on target board.  

```bash
undefined debug reason 7 - target needs reset
semihosting is enabled
target halted due to debug-request, current mode: Thread 
xPSR: 0x01000000 pc: 0x08002490 msp: 0x20050000, semihosting
target halted due to breakpoint, current mode: Thread 
xPSR: 0x61000000 pc: 0x20000046 msp: 0x20050000, semihosting
target halted due to debug-request, current mode: Thread 
xPSR: 0x01000000 pc: 0x08000400 msp: 0x20050000, semihosting
Hello, world!
```

![](https://i.imgur.com/5HC9gzy.gif)

## VS Code

This template includes launch configurations for debugging CortexM programs with Visual Studio Code located in the `.vscode/` directory.  
See [.vscode/README.md](./.vscode/README.md) for more information.  
If you're not using VS Code, you can safely delete the directory from the generated project.

# License

This template is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
