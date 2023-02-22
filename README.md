# Bare Metal Rust

## What does it do?
This is a simple example of how to write a bare metal Rust program for the Raspberry Pi 3. 
As this only provides the necessary bits to get you started, it can also serve as a template for new projects.

## How do I use it?
1) Make sure you have installed the following:
- Rust
- `armv7a-none-eabi` rustup target (use `rustup target add armv7a-none-eabi`)
- `arm-none-eabi` toolchain (use `sudo apt install binutils-arm-none-eabi`)
2) Clone this repository
3) Run `cargo build` to build the program. 
4) In the root of the project, run `arm-none-eabi-objcopy -O binary  target/armv7a-none-eabi/debug/bare-metal-rust ./kernel7.img` to output a .img file without any additional binary information.
5) Go to the Raspberry Pi [Firmware Repo](https://github.com/raspberrypi/firmware/tree/master/boot)
5) Download `fixup.dat`, `start.elf`, and `bootcode.bin` and place them in an empty folder on your system.
6) In the same folder, create a `config.txt` file. In this file, add the following line: `arm_64bit=0`
7) Copy the `kernel7.img` file you created before to the same folder.
8) Verify that your folder now contains exactly 5 files: `config.txt`, `fixup.dat`, `start.elf`, `bootcode.bin`, and `kernel7.img`
9) Copy the folder to the root of your FAT32 formatted microSD card.
10) Insert the microSD card into your Raspberry Pi 3 and power it on.

## Further Documentation
If you want to do simple stuff like blinking an LED, or maybe use the UART or SPI bus, refer to the following manual: [BCM2837 ARM Peripherals](https://cs140e.sergio.bz/docs/BCM2837-ARM-Peripherals.pdf)