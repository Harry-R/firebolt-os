#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;
mod serial;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    
    loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world!\n{}", 42);

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(test)]
/// runs the tests and exits qemu after finishing
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        serial_println!("Running {} tests", tests.len());
        test();

        exit_qemu(QemuExitCode::Success);
    }
}


#[test_case]
/// trivial function just for testing tests
fn trivial_assertion() {
    serial_println!("trivial assertion... ");
    assert_eq!(1, 0);
    serial_println!("[ok]");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
/// enum for qemu exit codes, 0x10 is mapped to success in bootimage configuration, see Cargo.toml
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// Exits qemu by writing to isa-debug-exit port
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}