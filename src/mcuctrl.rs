#[doc = "MCU Miscellaneous Control Logic."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcuctrl {
    ptr: *mut u8,
}
unsafe impl Send for Mcuctrl {}
unsafe impl Sync for Mcuctrl {}
impl Mcuctrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Chip Information Register."]
    #[inline(always)]
    pub const fn chippn(self) -> crate::common::Reg<regs::Chippn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Unique Chip ID 0."]
    #[inline(always)]
    pub const fn chipid0(self) -> crate::common::Reg<regs::Chipid0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Unique Chip ID 1."]
    #[inline(always)]
    pub const fn chipid1(self) -> crate::common::Reg<regs::Chipid1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Chip Revision."]
    #[inline(always)]
    pub const fn chiprev(self) -> crate::common::Reg<regs::Chiprev, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Unique Vendor ID."]
    #[inline(always)]
    pub const fn vendorid(self) -> crate::common::Reg<regs::Vendorid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Unique Chip SKU."]
    #[inline(always)]
    pub const fn sku(self) -> crate::common::Reg<regs::Sku, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Feature Enable on Burst and BLE."]
    #[inline(always)]
    pub const fn featureenable(self) -> crate::common::Reg<regs::Featureenable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Debugger Control."]
    #[inline(always)]
    pub const fn debugger(self) -> crate::common::Reg<regs::Debugger, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "BOD control Register."]
    #[inline(always)]
    pub const fn bodctrl(self) -> crate::common::Reg<regs::Bodctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "ADC Power Up Delay Control."]
    #[inline(always)]
    pub const fn adcpwrdly(self) -> crate::common::Reg<regs::Adcpwrdly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "ADC Calibration Control."]
    #[inline(always)]
    pub const fn adccal(self) -> crate::common::Reg<regs::Adccal, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "ADC Battery Load Enable."]
    #[inline(always)]
    pub const fn adcbattload(self) -> crate::common::Reg<regs::Adcbattload, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "ADC Trims."]
    #[inline(always)]
    pub const fn adctrim(self) -> crate::common::Reg<regs::Adctrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "ADC Referece Keeper and Comparator Control."]
    #[inline(always)]
    pub const fn adcrefcomp(self) -> crate::common::Reg<regs::Adcrefcomp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "XTAL Oscillator Control."]
    #[inline(always)]
    pub const fn xtalctrl(self) -> crate::common::Reg<regs::Xtalctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "XTAL Oscillator General Control."]
    #[inline(always)]
    pub const fn xtalgenctrl(self) -> crate::common::Reg<regs::Xtalgenctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "Miscellaneous control register."]
    #[inline(always)]
    pub const fn miscctrl(self) -> crate::common::Reg<regs::Miscctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "Bootloader and secure boot functions."]
    #[inline(always)]
    pub const fn bootloader(self) -> crate::common::Reg<regs::Bootloader, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "Register to indicate whether the shadow registers have been successfully loaded from the Flash Information Space."]
    #[inline(always)]
    pub const fn shadowvalid(self) -> crate::common::Reg<regs::Shadowvalid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "Scratch register that is not reset by any reset."]
    #[inline(always)]
    pub const fn scratch0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "Scratch register that is not reset by any reset."]
    #[inline(always)]
    pub const fn scratch1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "ICODE bus address which was present when a bus fault occurred."]
    #[inline(always)]
    pub const fn icodefaultaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "DCODE bus address which was present when a bus fault occurred."]
    #[inline(always)]
    pub const fn dcodefaultaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "System bus address which was present when a bus fault occurred."]
    #[inline(always)]
    pub const fn sysfaultaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "Reflects the status of the bus decoders' fault detection. Any write to this register will clear all of the status bits within the register."]
    #[inline(always)]
    pub const fn faultstatus(self) -> crate::common::Reg<regs::Faultstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "Enable the fault capture registers."]
    #[inline(always)]
    pub const fn faultcaptureen(
        self,
    ) -> crate::common::Reg<regs::Faultcaptureen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "Read-only debug register 1."]
    #[inline(always)]
    pub const fn dbgr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Read-only debug register 2."]
    #[inline(always)]
    pub const fn dbgr2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Control bit to enable/disable the PMU."]
    #[inline(always)]
    pub const fn pmuenable(self) -> crate::common::Reg<regs::Pmuenable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "TPIU Control Register. Determines the clock enable and frequency for the M4's TPIU interface."]
    #[inline(always)]
    pub const fn tpiuctrl(self) -> crate::common::Reg<regs::Tpiuctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "OTA (Over the Air) Update Pointer/Status. Reset only by POA."]
    #[inline(always)]
    pub const fn otapointer(self) -> crate::common::Reg<regs::Otapointer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "DMA Control Register. Determines misc settings for DMA operation."]
    #[inline(always)]
    pub const fn apbdmactrl(self) -> crate::common::Reg<regs::Apbdmactrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "SRAM Controller mode bits."]
    #[inline(always)]
    pub const fn srammode(self) -> crate::common::Reg<regs::Srammode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "Key Register to enable the use of external clock selects via the EXTCLKSEL reg."]
    #[inline(always)]
    pub const fn kextclksel(self) -> crate::common::Reg<regs::Kextclksel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0348usize) as _) }
    }
    #[doc = "SIMO Buck Control Reg1."]
    #[inline(always)]
    pub const fn simobuck4(self) -> crate::common::Reg<regs::Simobuck4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x035cusize) as _) }
    }
    #[doc = "BLEBUCK2 Control Reg."]
    #[inline(always)]
    pub const fn blebuck2(self) -> crate::common::Reg<regs::Blebuck2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0368usize) as _) }
    }
    #[doc = "Flash Write Protection Bits."]
    #[inline(always)]
    pub const fn flashwprot0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a0usize) as _) }
    }
    #[doc = "Flash Write Protection Bits."]
    #[inline(always)]
    pub const fn flashwprot1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a4usize) as _) }
    }
    #[doc = "Flash Read Protection Bits."]
    #[inline(always)]
    pub const fn flashrprot0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b0usize) as _) }
    }
    #[doc = "Flash Read Protection Bits."]
    #[inline(always)]
    pub const fn flashrprot1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b4usize) as _) }
    }
    #[doc = "SRAM write-protection bits."]
    #[inline(always)]
    pub const fn dmasramwriteprotect0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c0usize) as _) }
    }
    #[doc = "SRAM write-protection bits."]
    #[inline(always)]
    pub const fn dmasramwriteprotect1(
        self,
    ) -> crate::common::Reg<regs::Dmasramwriteprotect1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c4usize) as _) }
    }
    #[doc = "SRAM read-protection bits."]
    #[inline(always)]
    pub const fn dmasramreadprotect0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d0usize) as _) }
    }
    #[doc = "SRAM read-protection bits."]
    #[inline(always)]
    pub const fn dmasramreadprotect1(
        self,
    ) -> crate::common::Reg<regs::Dmasramreadprotect1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
