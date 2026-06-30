#[doc = "IO Peripheral Master."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iom0 {
    ptr: *mut u8,
}
unsafe impl Send for Iom0 {}
unsafe impl Sync for Iom0 {}
impl Iom0 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "FIFO Access Port."]
    #[inline(always)]
    pub const fn fifo(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "FIFO size and remaining slots open values."]
    #[inline(always)]
    pub const fn fifoptr(self) -> crate::common::Reg<regs::Fifoptr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "FIFO Threshold Configuration."]
    #[inline(always)]
    pub const fn fifothr(self) -> crate::common::Reg<regs::Fifothr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "FIFO POP register."]
    #[inline(always)]
    pub const fn fifopop(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "FIFO PUSH register."]
    #[inline(always)]
    pub const fn fifopush(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "FIFO Control Register."]
    #[inline(always)]
    pub const fn fifoctrl(self) -> crate::common::Reg<regs::Fifoctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "FIFO Pointers."]
    #[inline(always)]
    pub const fn fifoloc(self) -> crate::common::Reg<regs::Fifoloc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "IO Master Interrupts: Enable."]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "IO Master Interrupts: Status."]
    #[inline(always)]
    pub const fn intstat(self) -> crate::common::Reg<regs::Intstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "IO Master Interrupts: Clear."]
    #[inline(always)]
    pub const fn intclr(self) -> crate::common::Reg<regs::Intclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "IO Master Interrupts: Set."]
    #[inline(always)]
    pub const fn intset(self) -> crate::common::Reg<regs::Intset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "I/O Clock Configuration."]
    #[inline(always)]
    pub const fn clkcfg(self) -> crate::common::Reg<regs::Clkcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "Submodule control."]
    #[inline(always)]
    pub const fn submodctrl(self) -> crate::common::Reg<regs::Submodctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "Command and offset Register."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "DCX Control Register."]
    #[inline(always)]
    pub const fn dcx(self) -> crate::common::Reg<regs::Dcx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "High order 2 bytes of 3 byte offset for IO transaction."]
    #[inline(always)]
    pub const fn offsethi(self) -> crate::common::Reg<regs::Offsethi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "Command status."]
    #[inline(always)]
    pub const fn cmdstat(self) -> crate::common::Reg<regs::Cmdstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "DMA Trigger Enable Register."]
    #[inline(always)]
    pub const fn dmatrigen(self) -> crate::common::Reg<regs::Dmatrigen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "DMA Trigger Status Register."]
    #[inline(always)]
    pub const fn dmatrigstat(self) -> crate::common::Reg<regs::Dmatrigstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "DMA Configuration Register."]
    #[inline(always)]
    pub const fn dmacfg(self) -> crate::common::Reg<regs::Dmacfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "DMA Total Transfer Count."]
    #[inline(always)]
    pub const fn dmatotcount(self) -> crate::common::Reg<regs::Dmatotcount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "DMA Target Address Register."]
    #[inline(always)]
    pub const fn dmatargaddr(self) -> crate::common::Reg<regs::Dmatargaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x028cusize) as _) }
    }
    #[doc = "DMA Status Register."]
    #[inline(always)]
    pub const fn dmastat(self) -> crate::common::Reg<regs::Dmastat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[doc = "Command Queue Configuration Register."]
    #[inline(always)]
    pub const fn cqcfg(self) -> crate::common::Reg<regs::Cqcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[doc = "CQ Target Read Address Register."]
    #[inline(always)]
    pub const fn cqaddr(self) -> crate::common::Reg<regs::Cqaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0298usize) as _) }
    }
    #[doc = "Command Queue Status Register."]
    #[inline(always)]
    pub const fn cqstat(self) -> crate::common::Reg<regs::Cqstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x029cusize) as _) }
    }
    #[doc = "Command Queue Flag Register."]
    #[inline(always)]
    pub const fn cqflags(self) -> crate::common::Reg<regs::Cqflags, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[doc = "Command Queue Flag Set/Clear Register."]
    #[inline(always)]
    pub const fn cqsetclear(self) -> crate::common::Reg<regs::Cqsetclear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "Command Queue Pause Enable Register."]
    #[inline(always)]
    pub const fn cqpauseen(self) -> crate::common::Reg<regs::Cqpauseen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "IOM Command Queue current index value . Compared to the CQENDIDX reg contents to generate the IDXEQ Pause event for command queue."]
    #[inline(always)]
    pub const fn cqcuridx(self) -> crate::common::Reg<regs::Cqcuridx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02acusize) as _) }
    }
    #[doc = "IOM Command Queue current index value . Compared to the CQCURIDX reg contents to generate the IDXEQ Pause event for command queue."]
    #[inline(always)]
    pub const fn cqendidx(self) -> crate::common::Reg<regs::Cqendidx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[doc = "IOM Module Status Register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b4usize) as _) }
    }
    #[doc = "SPI module master configuration."]
    #[inline(always)]
    pub const fn mspicfg(self) -> crate::common::Reg<regs::Mspicfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "I2C Master configuration."]
    #[inline(always)]
    pub const fn mi2ccfg(self) -> crate::common::Reg<regs::Mi2ccfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "I2C Device Configuration register."]
    #[inline(always)]
    pub const fn devcfg(self) -> crate::common::Reg<regs::Devcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "IOM Debug Register."]
    #[inline(always)]
    pub const fn iomdbg(self) -> crate::common::Reg<regs::Iomdbg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
}
pub mod regs;
pub mod vals;
