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

struct XHC {
    mmio_base: usize,
    registers: xhci::Registers<IdentityMapper>,
    mapper: IdentityMapper,
}

impl XHC {
    pub fn new(mmio_base: usize) -> Self {
        let mapper = IdentityMapper {};
        let mut registers = unsafe { xhci::Registers::new(mmio_base, mapper.clone()) };
        Self {
            mmio_base,
            registers,
            mapper,
        }
    }
    fn init(&mut self) {
        self.request_xhc_ownership()
            .unwrap_or_else(|s| crate::serial_println!("{}", s));
    }
    fn request_xhc_ownership(&mut self) -> Result<(), &'static str> {
        let hccp = self.registers.capability.hccparams1.read_volatile();
        let mut ext_caps = unsafe {
            xhci::extended_capabilities::List::new(self.mmio_base, hccp, self.mapper.clone())
        }
        .unwrap();
        let mut iter = (&mut ext_caps).into_iter();
        let usblegsup = loop {
            let cap = iter.next();
            match cap {
                Some(cap) => {
                    if let xhci::ExtendedCapability::UsbLegacySupport(legsup) = cap.unwrap() {
                        break Some(legsup);
                    }
                }
                None => break None,
            }
        }
        .ok_or("USB legacy support capability is not implemented.")?;

        // In QEMU environment, USB legacy support capability does not seem to be supported.
        // So we will postpone implementing BIOS to OS handoff until actually needed...
        unimplemented!();
    }
}

pub fn init(pci_addr: PCIAddress) {
    let mmio_base = pci_addr.read_bar_64(0).unwrap() as usize;
    crate::serial_println!("mmio_base: {:x}", mmio_base);
    let mut xhc = XHC::new(mmio_base);
    xhc.init();
}
