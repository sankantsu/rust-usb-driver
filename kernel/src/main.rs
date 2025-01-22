#![no_std]
#![no_main]

mod pci;
mod serial;
mod xhci;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_print!("Panic!: info: {}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("Hello, kernel!");
    let mut pci_bus_scanner = pci::PCIBusScanner::new();
    pci_bus_scanner.scan_all();
    serial_println!("PCI Bus enumeration done.");
    let xhci_controller_addr = pci_bus_scanner.get_xhci_controller_address().unwrap();
    xhci::init(xhci_controller_addr);
    serial_println!("All done.");
    loop {}
}
