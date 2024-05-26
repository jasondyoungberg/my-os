use x86_64::{PhysAddr, VirtAddr};

use crate::HHDM_RESPONSE;

pub fn hhdm_phys_to_virt(phys: PhysAddr) -> VirtAddr {
    VirtAddr::new(phys.as_u64() + HHDM_RESPONSE.offset())
}

pub fn hhdm_virt_to_phys(virt: VirtAddr) -> PhysAddr {
    PhysAddr::new(virt.as_u64() - HHDM_RESPONSE.offset())
}
