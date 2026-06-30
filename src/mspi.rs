#[doc = "Multibit SPI Master."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mspi {
    ptr: *mut u8,
}
unsafe impl Send for Mspi {}
unsafe impl Sync for Mspi {}
impl Mspi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "MSPI PIO Transfer Control/Status Register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "MSPI Transfer Configuration Register."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "MSPI Transfer Address Register."]
    #[inline(always)]
    pub const fn addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "MSPI Transfer Instruction."]
    #[inline(always)]
    pub const fn instr(self) -> crate::common::Reg<regs::Instr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "TX Data FIFO."]
    #[inline(always)]
    pub const fn txfifo(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "RX Data FIFO."]
    #[inline(always)]
    pub const fn rxfifo(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "TX FIFO Entries."]
    #[inline(always)]
    pub const fn txentries(self) -> crate::common::Reg<regs::Txentries, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "RX FIFO Entries."]
    #[inline(always)]
    pub const fn rxentries(self) -> crate::common::Reg<regs::Rxentries, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "TX/RX FIFO Threshhold Levels."]
    #[inline(always)]
    pub const fn threshold(self) -> crate::common::Reg<regs::Threshold, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "MSPI Module Configuration."]
    #[inline(always)]
    pub const fn mspicfg(self) -> crate::common::Reg<regs::Mspicfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "MSPI Output Pad Configuration."]
    #[inline(always)]
    pub const fn padcfg(self) -> crate::common::Reg<regs::Padcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "MSPI Output Enable Pad Configuration."]
    #[inline(always)]
    pub const fn padouten(self) -> crate::common::Reg<regs::Padouten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Configuration for XIP/DMA support of SPI flash modules."]
    #[inline(always)]
    pub const fn flash(self) -> crate::common::Reg<regs::Flash, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "External Flash Scrambling Controls."]
    #[inline(always)]
    pub const fn scrambling(self) -> crate::common::Reg<regs::Scrambling, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "MSPI Master Interrupts: Enable."]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "MSPI Master Interrupts: Status."]
    #[inline(always)]
    pub const fn intstat(self) -> crate::common::Reg<regs::Intstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "MSPI Master Interrupts: Clear."]
    #[inline(always)]
    pub const fn intclr(self) -> crate::common::Reg<regs::Intclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "MSPI Master Interrupts: Set."]
    #[inline(always)]
    pub const fn intset(self) -> crate::common::Reg<regs::Intset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "DMA Configuration Register."]
    #[inline(always)]
    pub const fn dmacfg(self) -> crate::common::Reg<regs::Dmacfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "DMA Status Register."]
    #[inline(always)]
    pub const fn dmastat(self) -> crate::common::Reg<regs::Dmastat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "DMA Target Address Register."]
    #[inline(always)]
    pub const fn dmatargaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "DMA Device Address Register."]
    #[inline(always)]
    pub const fn dmadevaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "DMA Total Transfer Count."]
    #[inline(always)]
    pub const fn dmatotcount(self) -> crate::common::Reg<regs::Dmatotcount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "DMA BYTE Transfer Count."]
    #[inline(always)]
    pub const fn dmabcount(self) -> crate::common::Reg<regs::Dmabcount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "DMA Transmit Trigger Threshhold."]
    #[inline(always)]
    pub const fn dmathresh(self) -> crate::common::Reg<regs::Dmathresh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0278usize) as _) }
    }
    #[doc = "Command Queue Configuration Register."]
    #[inline(always)]
    pub const fn cqcfg(self) -> crate::common::Reg<regs::Cqcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[doc = "CQ Target Read Address Register."]
    #[inline(always)]
    pub const fn cqaddr(self) -> crate::common::Reg<regs::Cqaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "Command Queue Status Register."]
    #[inline(always)]
    pub const fn cqstat(self) -> crate::common::Reg<regs::Cqstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02acusize) as _) }
    }
    #[doc = "Command Queue Flag Register."]
    #[inline(always)]
    pub const fn cqflags(self) -> crate::common::Reg<regs::Cqflags, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[doc = "Command Queue Flag Set/Clear Register."]
    #[inline(always)]
    pub const fn cqsetclear(self) -> crate::common::Reg<regs::Cqsetclear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b4usize) as _) }
    }
    #[doc = "Command Queue Pause Mask Register."]
    #[inline(always)]
    pub const fn cqpause(self) -> crate::common::Reg<regs::Cqpause, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b8usize) as _) }
    }
    #[doc = "Command Queue Current Index."]
    #[inline(always)]
    pub const fn cqcuridx(self) -> crate::common::Reg<regs::Cqcuridx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[doc = "Command Queue End Index."]
    #[inline(always)]
    pub const fn cqendidx(self) -> crate::common::Reg<regs::Cqendidx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
