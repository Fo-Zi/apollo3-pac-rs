#[doc = "Counter/Timer."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctimer {
    ptr: *mut u8,
}
unsafe impl Send for Ctimer {}
unsafe impl Sync for Ctimer {}
impl Ctimer {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Counter/Timer Register."]
    #[inline(always)]
    pub const fn tmr0(self) -> crate::common::Reg<regs::Tmr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Counter/Timer A0 Compare Registers."]
    #[inline(always)]
    pub const fn cmpra0(self) -> crate::common::Reg<regs::Cmpra0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Counter/Timer B0 Compare Registers."]
    #[inline(always)]
    pub const fn cmprb0(self) -> crate::common::Reg<regs::Cmprb0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Counter/Timer Control."]
    #[inline(always)]
    pub const fn ctrl0(self) -> crate::common::Reg<regs::Ctrl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Counter/Timer A0 Compare Registers."]
    #[inline(always)]
    pub const fn cmprauxa0(self) -> crate::common::Reg<regs::Cmprauxa0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Counter/Timer B0 Compare Registers."]
    #[inline(always)]
    pub const fn cmprauxb0(self) -> crate::common::Reg<regs::Cmprauxb0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Counter/Timer Auxiliary."]
    #[inline(always)]
    pub const fn aux0(self) -> crate::common::Reg<regs::Aux0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Counter/Timer Register."]
    #[inline(always)]
    pub const fn tmr1(self) -> crate::common::Reg<regs::Tmr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Counter/Timer A1 Compare Registers."]
    #[inline(always)]
    pub const fn cmpra1(self) -> crate::common::Reg<regs::Cmpra1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Counter/Timer B1 Compare Registers."]
    #[inline(always)]
    pub const fn cmprb1(self) -> crate::common::Reg<regs::Cmprb1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Counter/Timer Control."]
    #[inline(always)]
    pub const fn ctrl1(self) -> crate::common::Reg<regs::Ctrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Counter/Timer A1 Compare Registers."]
    #[inline(always)]
    pub const fn cmprauxa1(self) -> crate::common::Reg<regs::Cmprauxa1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Counter/Timer B1 Compare Registers."]
    #[inline(always)]
    pub const fn cmprauxb1(self) -> crate::common::Reg<regs::Cmprauxb1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Counter/Timer Auxiliary."]
    #[inline(always)]
    pub const fn aux1(self) -> crate::common::Reg<regs::Aux1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Counter/Timer Register."]
    #[inline(always)]
    pub const fn tmr2(self) -> crate::common::Reg<regs::Tmr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Counter/Timer A2 Compare Registers."]
    #[inline(always)]
    pub const fn cmpra2(self) -> crate::common::Reg<regs::Cmpra2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Counter/Timer B2 Compare Registers."]
    #[inline(always)]
    pub const fn cmprb2(self) -> crate::common::Reg<regs::Cmprb2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Counter/Timer Control."]
    #[inline(always)]
    pub const fn ctrl2(self) -> crate::common::Reg<regs::Ctrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Counter/Timer A2 Compare Registers."]
    #[inline(always)]
    pub const fn cmprauxa2(self) -> crate::common::Reg<regs::Cmprauxa2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Counter/Timer B2 Compare Registers."]
    #[inline(always)]
    pub const fn cmprauxb2(self) -> crate::common::Reg<regs::Cmprauxb2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Counter/Timer Auxiliary."]
    #[inline(always)]
    pub const fn aux2(self) -> crate::common::Reg<regs::Aux2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "Counter/Timer Register."]
    #[inline(always)]
    pub const fn tmr3(self) -> crate::common::Reg<regs::Tmr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Counter/Timer A3 Compare Registers."]
    #[inline(always)]
    pub const fn cmpra3(self) -> crate::common::Reg<regs::Cmpra3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Counter/Timer B3 Compare Registers."]
    #[inline(always)]
    pub const fn cmprb3(self) -> crate::common::Reg<regs::Cmprb3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Counter/Timer Control."]
    #[inline(always)]
    pub const fn ctrl3(self) -> crate::common::Reg<regs::Ctrl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Counter/Timer A3 Compare Registers."]
    #[inline(always)]
    pub const fn cmprauxa3(self) -> crate::common::Reg<regs::Cmprauxa3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "Counter/Timer B3 Compare Registers."]
    #[inline(always)]
    pub const fn cmprauxb3(self) -> crate::common::Reg<regs::Cmprauxb3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "Counter/Timer Auxiliary."]
    #[inline(always)]
    pub const fn aux3(self) -> crate::common::Reg<regs::Aux3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "Counter/Timer Register."]
    #[inline(always)]
    pub const fn tmr4(self) -> crate::common::Reg<regs::Tmr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Counter/Timer A4 Compare Registers."]
    #[inline(always)]
    pub const fn cmpra4(self) -> crate::common::Reg<regs::Cmpra4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Counter/Timer B4 Compare Registers."]
    #[inline(always)]
    pub const fn cmprb4(self) -> crate::common::Reg<regs::Cmprb4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Counter/Timer Control."]
    #[inline(always)]
    pub const fn ctrl4(self) -> crate::common::Reg<regs::Ctrl4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Counter/Timer A4 Compare Registers."]
    #[inline(always)]
    pub const fn cmprauxa4(self) -> crate::common::Reg<regs::Cmprauxa4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Counter/Timer B4 Compare Registers."]
    #[inline(always)]
    pub const fn cmprauxb4(self) -> crate::common::Reg<regs::Cmprauxb4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Counter/Timer Auxiliary."]
    #[inline(always)]
    pub const fn aux4(self) -> crate::common::Reg<regs::Aux4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Counter/Timer Register."]
    #[inline(always)]
    pub const fn tmr5(self) -> crate::common::Reg<regs::Tmr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Counter/Timer A5 Compare Registers."]
    #[inline(always)]
    pub const fn cmpra5(self) -> crate::common::Reg<regs::Cmpra5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Counter/Timer B5 Compare Registers."]
    #[inline(always)]
    pub const fn cmprb5(self) -> crate::common::Reg<regs::Cmprb5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "Counter/Timer Control."]
    #[inline(always)]
    pub const fn ctrl5(self) -> crate::common::Reg<regs::Ctrl5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "Counter/Timer A5 Compare Registers."]
    #[inline(always)]
    pub const fn cmprauxa5(self) -> crate::common::Reg<regs::Cmprauxa5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Counter/Timer B5 Compare Registers."]
    #[inline(always)]
    pub const fn cmprauxb5(self) -> crate::common::Reg<regs::Cmprauxb5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "Counter/Timer Auxiliary."]
    #[inline(always)]
    pub const fn aux5(self) -> crate::common::Reg<regs::Aux5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "Counter/Timer Register."]
    #[inline(always)]
    pub const fn tmr6(self) -> crate::common::Reg<regs::Tmr6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Counter/Timer A6 Compare Registers."]
    #[inline(always)]
    pub const fn cmpra6(self) -> crate::common::Reg<regs::Cmpra6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "Counter/Timer B6 Compare Registers."]
    #[inline(always)]
    pub const fn cmprb6(self) -> crate::common::Reg<regs::Cmprb6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "Counter/Timer Control."]
    #[inline(always)]
    pub const fn ctrl6(self) -> crate::common::Reg<regs::Ctrl6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "Counter/Timer A6 Compare Registers."]
    #[inline(always)]
    pub const fn cmprauxa6(self) -> crate::common::Reg<regs::Cmprauxa6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "Counter/Timer B6 Compare Registers."]
    #[inline(always)]
    pub const fn cmprauxb6(self) -> crate::common::Reg<regs::Cmprauxb6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "Counter/Timer Auxiliary."]
    #[inline(always)]
    pub const fn aux6(self) -> crate::common::Reg<regs::Aux6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "Counter/Timer Register."]
    #[inline(always)]
    pub const fn tmr7(self) -> crate::common::Reg<regs::Tmr7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "Counter/Timer A7 Compare Registers."]
    #[inline(always)]
    pub const fn cmpra7(self) -> crate::common::Reg<regs::Cmpra7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "Counter/Timer B7 Compare Registers."]
    #[inline(always)]
    pub const fn cmprb7(self) -> crate::common::Reg<regs::Cmprb7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "Counter/Timer Control."]
    #[inline(always)]
    pub const fn ctrl7(self) -> crate::common::Reg<regs::Ctrl7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "Counter/Timer A7 Compare Registers."]
    #[inline(always)]
    pub const fn cmprauxa7(self) -> crate::common::Reg<regs::Cmprauxa7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "Counter/Timer B7 Compare Registers."]
    #[inline(always)]
    pub const fn cmprauxb7(self) -> crate::common::Reg<regs::Cmprauxb7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Counter/Timer Auxiliary."]
    #[inline(always)]
    pub const fn aux7(self) -> crate::common::Reg<regs::Aux7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "Counter/Timer Global Enable."]
    #[inline(always)]
    pub const fn globen(self) -> crate::common::Reg<regs::Globen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Counter/Timer Output Config 0."]
    #[inline(always)]
    pub const fn outcfg0(self) -> crate::common::Reg<regs::Outcfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Counter/Timer Output Config 1."]
    #[inline(always)]
    pub const fn outcfg1(self) -> crate::common::Reg<regs::Outcfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "Counter/Timer Output Config 2."]
    #[inline(always)]
    pub const fn outcfg2(self) -> crate::common::Reg<regs::Outcfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "Counter/Timer Output Config 3."]
    #[inline(always)]
    pub const fn outcfg3(self) -> crate::common::Reg<regs::Outcfg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "Counter/Timer Input Config."]
    #[inline(always)]
    pub const fn incfg(self) -> crate::common::Reg<regs::Incfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "Configuration Register."]
    #[inline(always)]
    pub const fn stcfg(self) -> crate::common::Reg<regs::Stcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "System Timer Count Register (Real Time Counter)."]
    #[inline(always)]
    pub const fn sttmr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "Capture Control Register."]
    #[inline(always)]
    pub const fn capturecontrol(
        self,
    ) -> crate::common::Reg<regs::Capturecontrol, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "Compare Register A."]
    #[inline(always)]
    pub const fn scmpr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "Compare Register B."]
    #[inline(always)]
    pub const fn scmpr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "Compare Register C."]
    #[inline(always)]
    pub const fn scmpr2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "Compare Register D."]
    #[inline(always)]
    pub const fn scmpr3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "Compare Register E."]
    #[inline(always)]
    pub const fn scmpr4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "Compare Register F."]
    #[inline(always)]
    pub const fn scmpr5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "Compare Register G."]
    #[inline(always)]
    pub const fn scmpr6(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "Compare Register H."]
    #[inline(always)]
    pub const fn scmpr7(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "Capture Register A."]
    #[inline(always)]
    pub const fn scapt0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "Capture Register B."]
    #[inline(always)]
    pub const fn scapt1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "Capture Register C."]
    #[inline(always)]
    pub const fn scapt2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "Capture Register D."]
    #[inline(always)]
    pub const fn scapt3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "System Timer NVRAM_A Register."]
    #[inline(always)]
    pub const fn snvr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "System Timer NVRAM_B Register."]
    #[inline(always)]
    pub const fn snvr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "System Timer NVRAM_C Register."]
    #[inline(always)]
    pub const fn snvr2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "System Timer NVRAM_D Register."]
    #[inline(always)]
    pub const fn snvr3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "Counter/Timer Interrupts: Enable."]
    #[inline(always)]
    pub const fn inten(self) -> crate::common::Reg<regs::Inten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Counter/Timer Interrupts: Status."]
    #[inline(always)]
    pub const fn intstat(self) -> crate::common::Reg<regs::Intstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Counter/Timer Interrupts: Clear."]
    #[inline(always)]
    pub const fn intclr(self) -> crate::common::Reg<regs::Intclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "Counter/Timer Interrupts: Set."]
    #[inline(always)]
    pub const fn intset(self) -> crate::common::Reg<regs::Intset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "STIMER Interrupt registers: Enable."]
    #[inline(always)]
    pub const fn stminten(self) -> crate::common::Reg<regs::Stminten, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "STIMER Interrupt registers: Status."]
    #[inline(always)]
    pub const fn stmintstat(self) -> crate::common::Reg<regs::Stmintstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "STIMER Interrupt registers: Clear."]
    #[inline(always)]
    pub const fn stmintclr(self) -> crate::common::Reg<regs::Stmintclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "STIMER Interrupt registers: Set."]
    #[inline(always)]
    pub const fn stmintset(self) -> crate::common::Reg<regs::Stmintset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
}
pub mod regs;
pub mod vals;
