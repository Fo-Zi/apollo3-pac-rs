#[doc = "General Purpose IO."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Pad Configuration Register A (Pads 0-3)."]
    #[inline(always)]
    pub const fn padrega(self) -> crate::common::Reg<regs::Padrega, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Pad Configuration Register B (Pads 4-7)."]
    #[inline(always)]
    pub const fn padregb(self) -> crate::common::Reg<regs::Padregb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Pad Configuration Register C (Pads 8-11)."]
    #[inline(always)]
    pub const fn padregc(self) -> crate::common::Reg<regs::Padregc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Pad Configuration Register D (Pads 12-15)."]
    #[inline(always)]
    pub const fn padregd(self) -> crate::common::Reg<regs::Padregd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Pad Configuration Register E (Pads 16-19)."]
    #[inline(always)]
    pub const fn padrege(self) -> crate::common::Reg<regs::Padrege, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Pad Configuration Register F (Pads 20-23)."]
    #[inline(always)]
    pub const fn padregf(self) -> crate::common::Reg<regs::Padregf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Pad Configuration Register G (Pads 24-27)."]
    #[inline(always)]
    pub const fn padregg(self) -> crate::common::Reg<regs::Padregg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Pad Configuration Register H (Pads 28-31)."]
    #[inline(always)]
    pub const fn padregh(self) -> crate::common::Reg<regs::Padregh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Pad Configuration Register I (Pads 32-25)."]
    #[inline(always)]
    pub const fn padregi(self) -> crate::common::Reg<regs::Padregi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Pad Configuration Register J (Pads 36-39)."]
    #[inline(always)]
    pub const fn padregj(self) -> crate::common::Reg<regs::Padregj, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Pad Configuration Register K (Pads 40-43)."]
    #[inline(always)]
    pub const fn padregk(self) -> crate::common::Reg<regs::Padregk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Pad Configuration Register L (Pads 44-47)."]
    #[inline(always)]
    pub const fn padregl(self) -> crate::common::Reg<regs::Padregl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Pad Configuration Register M (Pads 47-48)."]
    #[inline(always)]
    pub const fn padregm(self) -> crate::common::Reg<regs::Padregm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "GPIO Configuration Register A (Pads 0-7)."]
    #[inline(always)]
    pub const fn cfga(self) -> crate::common::Reg<regs::Cfga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "GPIO Configuration Register B (Pads 8-15)."]
    #[inline(always)]
    pub const fn cfgb(self) -> crate::common::Reg<regs::Cfgb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "GPIO Configuration Register C (Pads 16-23)."]
    #[inline(always)]
    pub const fn cfgc(self) -> crate::common::Reg<regs::Cfgc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "GPIO Configuration Register D (Pads 24-31)."]
    #[inline(always)]
    pub const fn cfgd(self) -> crate::common::Reg<regs::Cfgd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "GPIO Configuration Register E (Pads 32-39)."]
    #[inline(always)]
    pub const fn cfge(self) -> crate::common::Reg<regs::Cfge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "GPIO Configuration Register F (Pads 40 -47)."]
    #[inline(always)]
    pub const fn cfgf(self) -> crate::common::Reg<regs::Cfgf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "GPIO Configuration Register G (Pads 48-49)."]
    #[inline(always)]
    pub const fn cfgg(self) -> crate::common::Reg<regs::Cfgg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Key Register for all pad configuration registers."]
    #[inline(always)]
    pub const fn padkey(self) -> crate::common::Reg<regs::Padkey, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "GPIO Input Register A."]
    #[inline(always)]
    pub const fn rda(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "GPIO Input Register B."]
    #[inline(always)]
    pub const fn rdb(self) -> crate::common::Reg<regs::Rdb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "GPIO Output Register A."]
    #[inline(always)]
    pub const fn wta(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "GPIO Output Register B."]
    #[inline(always)]
    pub const fn wtb(self) -> crate::common::Reg<regs::Wtb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "GPIO Output Register A Set."]
    #[inline(always)]
    pub const fn wtsa(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "GPIO Output Register B Set."]
    #[inline(always)]
    pub const fn wtsb(self) -> crate::common::Reg<regs::Wtsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "GPIO Output Register A Clear."]
    #[inline(always)]
    pub const fn wtca(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "GPIO Output Register B Clear."]
    #[inline(always)]
    pub const fn wtcb(self) -> crate::common::Reg<regs::Wtcb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "GPIO Enable Register A."]
    #[inline(always)]
    pub const fn ena(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "GPIO Enable Register B."]
    #[inline(always)]
    pub const fn enb(self) -> crate::common::Reg<regs::Enb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "GPIO Enable Register A Set."]
    #[inline(always)]
    pub const fn ensa(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "GPIO Enable Register B Set."]
    #[inline(always)]
    pub const fn ensb(self) -> crate::common::Reg<regs::Ensb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "GPIO Enable Register A Clear."]
    #[inline(always)]
    pub const fn enca(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "GPIO Enable Register B Clear."]
    #[inline(always)]
    pub const fn encb(self) -> crate::common::Reg<regs::Encb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "STIMER Capture Control."]
    #[inline(always)]
    pub const fn stmrcap(self) -> crate::common::Reg<regs::Stmrcap, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "IOM0 Flow Control IRQ Select."]
    #[inline(always)]
    pub const fn iom0irq(self) -> crate::common::Reg<regs::Iom0irq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "IOM1 Flow Control IRQ Select."]
    #[inline(always)]
    pub const fn iom1irq(self) -> crate::common::Reg<regs::Iom1irq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "IOM2 Flow Control IRQ Select."]
    #[inline(always)]
    pub const fn iom2irq(self) -> crate::common::Reg<regs::Iom2irq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "IOM3 Flow Control IRQ Select."]
    #[inline(always)]
    pub const fn iom3irq(self) -> crate::common::Reg<regs::Iom3irq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "IOM4 Flow Control IRQ Select."]
    #[inline(always)]
    pub const fn iom4irq(self) -> crate::common::Reg<regs::Iom4irq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "IOM5 Flow Control IRQ Select."]
    #[inline(always)]
    pub const fn iom5irq(self) -> crate::common::Reg<regs::Iom5irq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "BLEIF Flow Control IRQ Select."]
    #[inline(always)]
    pub const fn bleifirq(self) -> crate::common::Reg<regs::Bleifirq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "GPIO Observation Mode Sample register."]
    #[inline(always)]
    pub const fn gpioobs(self) -> crate::common::Reg<regs::Gpioobs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Alternate Pad Configuration reg0 (Pads 3,2,1,0)."]
    #[inline(always)]
    pub const fn altpadcfga(self) -> crate::common::Reg<regs::Altpadcfga, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Alternate Pad Configuration reg1 (Pads 7,6,5,4)."]
    #[inline(always)]
    pub const fn altpadcfgb(self) -> crate::common::Reg<regs::Altpadcfgb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Alternate Pad Configuration reg2 (Pads 11,10,9,8)."]
    #[inline(always)]
    pub const fn altpadcfgc(self) -> crate::common::Reg<regs::Altpadcfgc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Alternate Pad Configuration reg3 (Pads 15,14,13,12)."]
    #[inline(always)]
    pub const fn altpadcfgd(self) -> crate::common::Reg<regs::Altpadcfgd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Alternate Pad Configuration reg4 (Pads 19,18,17,16)."]
    #[inline(always)]
    pub const fn altpadcfge(self) -> crate::common::Reg<regs::Altpadcfge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "Alternate Pad Configuration reg5 (Pads 23,22,21,20)."]
    #[inline(always)]
    pub const fn altpadcfgf(self) -> crate::common::Reg<regs::Altpadcfgf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Alternate Pad Configuration reg6 (Pads 27,26,25,24)."]
    #[inline(always)]
    pub const fn altpadcfgg(self) -> crate::common::Reg<regs::Altpadcfgg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Alternate Pad Configuration reg7 (Pads 31,30,29,28)."]
    #[inline(always)]
    pub const fn altpadcfgh(self) -> crate::common::Reg<regs::Altpadcfgh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Alternate Pad Configuration reg8 (Pads 35,34,33,32)."]
    #[inline(always)]
    pub const fn altpadcfgi(self) -> crate::common::Reg<regs::Altpadcfgi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Alternate Pad Configuration reg9 (Pads 39,38,37,36)."]
    #[inline(always)]
    pub const fn altpadcfgj(self) -> crate::common::Reg<regs::Altpadcfgj, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Alternate Pad Configuration reg10 (Pads 43,42,41,40)."]
    #[inline(always)]
    pub const fn altpadcfgk(self) -> crate::common::Reg<regs::Altpadcfgk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Alternate Pad Configuration reg11 (Pads 47,46,45,44)."]
    #[inline(always)]
    pub const fn altpadcfgl(self) -> crate::common::Reg<regs::Altpadcfgl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Alternate Pad Configuration reg12 (Pads 49,48)."]
    #[inline(always)]
    pub const fn altpadcfgm(self) -> crate::common::Reg<regs::Altpadcfgm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "SCARD Card Detect select."]
    #[inline(always)]
    pub const fn scdet(self) -> crate::common::Reg<regs::Scdet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Counter/Timer Enable Config."]
    #[inline(always)]
    pub const fn ctencfg(self) -> crate::common::Reg<regs::Ctencfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "GPIO Interrupt Registers 31-0: Enable."]
    #[inline(always)]
    pub const fn int0en(self) -> crate::common::Reg<regs::Int0en, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "GPIO Interrupt Registers 31-0: Status."]
    #[inline(always)]
    pub const fn int0stat(self) -> crate::common::Reg<regs::Int0stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "GPIO Interrupt Registers 31-0: Clear."]
    #[inline(always)]
    pub const fn int0clr(self) -> crate::common::Reg<regs::Int0clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "GPIO Interrupt Registers 31-0: Set."]
    #[inline(always)]
    pub const fn int0set(self) -> crate::common::Reg<regs::Int0set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "GPIO Interrupt Registers 49-32: Enable."]
    #[inline(always)]
    pub const fn int1en(self) -> crate::common::Reg<regs::Int1en, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "GPIO Interrupt Registers 49-32: Status."]
    #[inline(always)]
    pub const fn int1stat(self) -> crate::common::Reg<regs::Int1stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "GPIO Interrupt Registers 49-32: Clear."]
    #[inline(always)]
    pub const fn int1clr(self) -> crate::common::Reg<regs::Int1clr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "GPIO Interrupt Registers 49-32: Set."]
    #[inline(always)]
    pub const fn int1set(self) -> crate::common::Reg<regs::Int1set, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
