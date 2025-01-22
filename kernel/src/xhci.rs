use crate::pci::PCIAddress;

// Directory use physical memory
#[derive(Clone)]
struct IdentityMapper;

impl xhci::accessor::Mapper for IdentityMapper {
    unsafe fn map(&mut self, phys_start: usize, _bytes: usize) -> core::num::NonZeroUsize {
        core::num::NonZeroUsize::new_unchecked(phys_start)
    }
    fn unmap(&mut self, _virt_start: usize, _bytes: usize) {
        // Do nothing
    }
}

pub fn init(pci_addr: PCIAddress) {
    let mapper = IdentityMapper {};
    let mmio_base = pci_addr.read_bar_64(0).unwrap() as usize;
    crate::serial_println!("mmio_base: {:x}", mmio_base);
    let mut registers = unsafe { xhci::Registers::new(mmio_base, mapper.clone()) };
    crate::serial_println!("Succesfully initialized xhci registers!");
}
