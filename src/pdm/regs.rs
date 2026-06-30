#[doc = "DMA Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmacfg(pub u32);
impl Dmacfg {
    #[doc = "DMA Enable."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Enable."]
    #[inline(always)]
    pub const fn set_dmaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn dmadir(&self) -> super::vals::Dmadir {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dmadir::from_bits(val as u8)
    }
    #[doc = "Direction."]
    #[inline(always)]
    pub const fn set_dmadir(&mut self, val: super::vals::Dmadir) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Sets the Priority of the DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn dmapri(&self) -> super::vals::Dmapri {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Dmapri::from_bits(val as u8)
    }
    #[doc = "Sets the Priority of the DMA request."]
    #[inline(always)]
    pub const fn set_dmapri(&mut self, val: super::vals::Dmapri) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Raise priority to high on fifo full, and DMAPRI set to low."]
    #[must_use]
    #[inline(always)]
    pub const fn dautohip(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Raise priority to high on fifo full, and DMAPRI set to low."]
    #[inline(always)]
    pub const fn set_dautohip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Power Off the ADC System upon DMACPL."]
    #[must_use]
    #[inline(always)]
    pub const fn dpwroff(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Power Off the ADC System upon DMACPL."]
    #[inline(always)]
    pub const fn set_dpwroff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Dmacfg {
    #[inline(always)]
    fn default() -> Dmacfg {
        Dmacfg(0)
    }
}
impl core::fmt::Debug for Dmacfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmacfg")
            .field("dmaen", &self.dmaen())
            .field("dmadir", &self.dmadir())
            .field("dmapri", &self.dmapri())
            .field("dautohip", &self.dautohip())
            .field("dpwroff", &self.dpwroff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmacfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dmacfg {{ dmaen: {=bool:?}, dmadir: {:?}, dmapri: {:?}, dautohip: {=bool:?}, dpwroff: {=bool:?} }}" , self . dmaen () , self . dmadir () , self . dmapri () , self . dautohip () , self . dpwroff ())
    }
}
#[doc = "DMA Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmastat(pub u32);
impl Dmastat {
    #[doc = "DMA Transfer In Progress."]
    #[must_use]
    #[inline(always)]
    pub const fn dmatip(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Transfer In Progress."]
    #[inline(always)]
    pub const fn set_dmatip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA Transfer Complete."]
    #[must_use]
    #[inline(always)]
    pub const fn dmacpl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Transfer Complete."]
    #[inline(always)]
    pub const fn set_dmacpl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA Error."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaerr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error."]
    #[inline(always)]
    pub const fn set_dmaerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Dmastat {
    #[inline(always)]
    fn default() -> Dmastat {
        Dmastat(0)
    }
}
impl core::fmt::Debug for Dmastat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmastat")
            .field("dmatip", &self.dmatip())
            .field("dmacpl", &self.dmacpl())
            .field("dmaerr", &self.dmaerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmastat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmastat {{ dmatip: {=bool:?}, dmacpl: {=bool:?}, dmaerr: {=bool:?} }}",
            self.dmatip(),
            self.dmacpl(),
            self.dmaerr()
        )
    }
}
#[doc = "DMA Target Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmatargaddr(pub u32);
impl Dmatargaddr {
    #[doc = "DMA Target Address. This register is not updated with the current address of the DMA, but will remain static with the original address during the DMA transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn ltargaddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "DMA Target Address. This register is not updated with the current address of the DMA, but will remain static with the original address during the DMA transfer."]
    #[inline(always)]
    pub const fn set_ltargaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
    #[doc = "SRAM Target."]
    #[must_use]
    #[inline(always)]
    pub const fn utargaddr(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "SRAM Target."]
    #[inline(always)]
    pub const fn set_utargaddr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for Dmatargaddr {
    #[inline(always)]
    fn default() -> Dmatargaddr {
        Dmatargaddr(0)
    }
}
impl core::fmt::Debug for Dmatargaddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmatargaddr")
            .field("ltargaddr", &self.ltargaddr())
            .field("utargaddr", &self.utargaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmatargaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmatargaddr {{ ltargaddr: {=u32:?}, utargaddr: {=u16:?} }}",
            self.ltargaddr(),
            self.utargaddr()
        )
    }
}
#[doc = "DMA Total Transfer Count."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmatotcount(pub u32);
impl Dmatotcount {
    #[doc = "Total Transfer Count. The transfer count must be a multiple of the THR setting to avoid DMA overruns."]
    #[must_use]
    #[inline(always)]
    pub const fn totcount(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Total Transfer Count. The transfer count must be a multiple of the THR setting to avoid DMA overruns."]
    #[inline(always)]
    pub const fn set_totcount(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Dmatotcount {
    #[inline(always)]
    fn default() -> Dmatotcount {
        Dmatotcount(0)
    }
}
impl core::fmt::Debug for Dmatotcount {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmatotcount")
            .field("totcount", &self.totcount())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmatotcount {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dmatotcount {{ totcount: {=u32:?} }}", self.totcount())
    }
}
#[doc = "DMA Trigger Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmatrigen(pub u32);
impl Dmatrigen {
    #[doc = "Trigger DMA upon when FIFO iss filled to level indicated by the FIFO THRESHOLD,at granularity of 16 bytes only."]
    #[must_use]
    #[inline(always)]
    pub const fn dthr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger DMA upon when FIFO iss filled to level indicated by the FIFO THRESHOLD,at granularity of 16 bytes only."]
    #[inline(always)]
    pub const fn set_dthr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger DMA at FIFO 90 percent full. This signal is also used internally for AUTOHIP function."]
    #[must_use]
    #[inline(always)]
    pub const fn dthr90(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger DMA at FIFO 90 percent full. This signal is also used internally for AUTOHIP function."]
    #[inline(always)]
    pub const fn set_dthr90(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Dmatrigen {
    #[inline(always)]
    fn default() -> Dmatrigen {
        Dmatrigen(0)
    }
}
impl core::fmt::Debug for Dmatrigen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmatrigen")
            .field("dthr", &self.dthr())
            .field("dthr90", &self.dthr90())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmatrigen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmatrigen {{ dthr: {=bool:?}, dthr90: {=bool:?} }}",
            self.dthr(),
            self.dthr90()
        )
    }
}
#[doc = "DMA Trigger Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmatrigstat(pub u32);
impl Dmatrigstat {
    #[doc = "Triggered DMA from FIFO reaching threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn dthrstat(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Triggered DMA from FIFO reaching threshold."]
    #[inline(always)]
    pub const fn set_dthrstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Triggered DMA from FIFO reaching 90 percent full."]
    #[must_use]
    #[inline(always)]
    pub const fn dthr90stat(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Triggered DMA from FIFO reaching 90 percent full."]
    #[inline(always)]
    pub const fn set_dthr90stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Dmatrigstat {
    #[inline(always)]
    fn default() -> Dmatrigstat {
        Dmatrigstat(0)
    }
}
impl core::fmt::Debug for Dmatrigstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmatrigstat")
            .field("dthrstat", &self.dthrstat())
            .field("dthr90stat", &self.dthr90stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmatrigstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmatrigstat {{ dthrstat: {=bool:?}, dthr90stat: {=bool:?} }}",
            self.dthrstat(),
            self.dthr90stat()
        )
    }
}
#[doc = "FIFO Flush."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifoflush(pub u32);
impl Fifoflush {
    #[doc = "FIFO FLUSH."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoflush(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO FLUSH."]
    #[inline(always)]
    pub const fn set_fifoflush(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Fifoflush {
    #[inline(always)]
    fn default() -> Fifoflush {
        Fifoflush(0)
    }
}
impl core::fmt::Debug for Fifoflush {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifoflush")
            .field("fifoflush", &self.fifoflush())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifoflush {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fifoflush {{ fifoflush: {=bool:?} }}", self.fifoflush())
    }
}
#[doc = "FIFO Threshold."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifothr(pub u32);
impl Fifothr {
    #[doc = "FIFO Threshold value. When the FIFO count is equal to, or larger than this value (in words), a THR interrupt is generated (if enabled)."]
    #[must_use]
    #[inline(always)]
    pub const fn fifothr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "FIFO Threshold value. When the FIFO count is equal to, or larger than this value (in words), a THR interrupt is generated (if enabled)."]
    #[inline(always)]
    pub const fn set_fifothr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Fifothr {
    #[inline(always)]
    fn default() -> Fifothr {
        Fifothr(0)
    }
}
impl core::fmt::Debug for Fifothr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifothr")
            .field("fifothr", &self.fifothr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifothr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fifothr {{ fifothr: {=u8:?} }}", self.fifothr())
    }
}
#[doc = "IO Master Interrupts: Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intclr(pub u32);
impl Intclr {
    #[doc = "This is the FIFO threshold interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn thr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This is the FIFO threshold interrupt."]
    #[inline(always)]
    pub const fn set_thr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This is the FIFO overflow interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ovf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This is the FIFO overflow interrupt."]
    #[inline(always)]
    pub const fn set_ovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This is the FIFO underflow interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn undfl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This is the FIFO underflow interrupt."]
    #[inline(always)]
    pub const fn set_undfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA completed a transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA completed a transfer."]
    #[inline(always)]
    pub const fn set_dcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA Error receieved."]
    #[must_use]
    #[inline(always)]
    pub const fn derr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error receieved."]
    #[inline(always)]
    pub const fn set_derr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Intclr {
    #[inline(always)]
    fn default() -> Intclr {
        Intclr(0)
    }
}
impl core::fmt::Debug for Intclr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intclr")
            .field("thr", &self.thr())
            .field("ovf", &self.ovf())
            .field("undfl", &self.undfl())
            .field("dcmp", &self.dcmp())
            .field("derr", &self.derr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intclr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Intclr {{ thr: {=bool:?}, ovf: {=bool:?}, undfl: {=bool:?}, dcmp: {=bool:?}, derr: {=bool:?} }}" , self . thr () , self . ovf () , self . undfl () , self . dcmp () , self . derr ())
    }
}
#[doc = "IO Master Interrupts: Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "This is the FIFO threshold interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn thr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This is the FIFO threshold interrupt."]
    #[inline(always)]
    pub const fn set_thr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This is the FIFO overflow interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ovf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This is the FIFO overflow interrupt."]
    #[inline(always)]
    pub const fn set_ovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This is the FIFO underflow interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn undfl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This is the FIFO underflow interrupt."]
    #[inline(always)]
    pub const fn set_undfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA completed a transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA completed a transfer."]
    #[inline(always)]
    pub const fn set_dcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA Error receieved."]
    #[must_use]
    #[inline(always)]
    pub const fn derr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error receieved."]
    #[inline(always)]
    pub const fn set_derr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
impl core::fmt::Debug for Inten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Inten")
            .field("thr", &self.thr())
            .field("ovf", &self.ovf())
            .field("undfl", &self.undfl())
            .field("dcmp", &self.dcmp())
            .field("derr", &self.derr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Inten {{ thr: {=bool:?}, ovf: {=bool:?}, undfl: {=bool:?}, dcmp: {=bool:?}, derr: {=bool:?} }}" , self . thr () , self . ovf () , self . undfl () , self . dcmp () , self . derr ())
    }
}
#[doc = "IO Master Interrupts: Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intset(pub u32);
impl Intset {
    #[doc = "This is the FIFO threshold interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn thr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This is the FIFO threshold interrupt."]
    #[inline(always)]
    pub const fn set_thr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This is the FIFO overflow interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ovf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This is the FIFO overflow interrupt."]
    #[inline(always)]
    pub const fn set_ovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This is the FIFO underflow interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn undfl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This is the FIFO underflow interrupt."]
    #[inline(always)]
    pub const fn set_undfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA completed a transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA completed a transfer."]
    #[inline(always)]
    pub const fn set_dcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA Error receieved."]
    #[must_use]
    #[inline(always)]
    pub const fn derr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error receieved."]
    #[inline(always)]
    pub const fn set_derr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Intset {
    #[inline(always)]
    fn default() -> Intset {
        Intset(0)
    }
}
impl core::fmt::Debug for Intset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intset")
            .field("thr", &self.thr())
            .field("ovf", &self.ovf())
            .field("undfl", &self.undfl())
            .field("dcmp", &self.dcmp())
            .field("derr", &self.derr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intset {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Intset {{ thr: {=bool:?}, ovf: {=bool:?}, undfl: {=bool:?}, dcmp: {=bool:?}, derr: {=bool:?} }}" , self . thr () , self . ovf () , self . undfl () , self . dcmp () , self . derr ())
    }
}
#[doc = "IO Master Interrupts: Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "This is the FIFO threshold interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn thr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This is the FIFO threshold interrupt."]
    #[inline(always)]
    pub const fn set_thr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This is the FIFO overflow interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn ovf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This is the FIFO overflow interrupt."]
    #[inline(always)]
    pub const fn set_ovf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "This is the FIFO underflow interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn undfl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "This is the FIFO underflow interrupt."]
    #[inline(always)]
    pub const fn set_undfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA completed a transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "DMA completed a transfer."]
    #[inline(always)]
    pub const fn set_dcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "DMA Error receieved."]
    #[must_use]
    #[inline(always)]
    pub const fn derr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error receieved."]
    #[inline(always)]
    pub const fn set_derr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Intstat {
    #[inline(always)]
    fn default() -> Intstat {
        Intstat(0)
    }
}
impl core::fmt::Debug for Intstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intstat")
            .field("thr", &self.thr())
            .field("ovf", &self.ovf())
            .field("undfl", &self.undfl())
            .field("dcmp", &self.dcmp())
            .field("derr", &self.derr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Intstat {{ thr: {=bool:?}, ovf: {=bool:?}, undfl: {=bool:?}, dcmp: {=bool:?}, derr: {=bool:?} }}" , self . thr () , self . ovf () , self . undfl () , self . dcmp () , self . derr ())
    }
}
#[doc = "PDM Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcfg(pub u32);
impl Pcfg {
    #[doc = "Data Streaming Control."]
    #[must_use]
    #[inline(always)]
    pub const fn pdmcoreen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Data Streaming Control."]
    #[inline(always)]
    pub const fn set_pdmcoreen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Soft mute control."]
    #[must_use]
    #[inline(always)]
    pub const fn softmute(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Soft mute control."]
    #[inline(always)]
    pub const fn set_softmute(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Number of clocks during gain-setting changes."]
    #[must_use]
    #[inline(always)]
    pub const fn cycles(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "Number of clocks during gain-setting changes."]
    #[inline(always)]
    pub const fn set_cycles(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "High pass filter coefficients."]
    #[must_use]
    #[inline(always)]
    pub const fn hpcutoff(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x0f;
        val as u8
    }
    #[doc = "High pass filter coefficients."]
    #[inline(always)]
    pub const fn set_hpcutoff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
    }
    #[doc = "High pass filter control."]
    #[must_use]
    #[inline(always)]
    pub const fn adchpd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "High pass filter control."]
    #[inline(always)]
    pub const fn set_adchpd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "SINC decimation rate."]
    #[must_use]
    #[inline(always)]
    pub const fn sincrate(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x7f;
        val as u8
    }
    #[doc = "SINC decimation rate."]
    #[inline(always)]
    pub const fn set_sincrate(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 10usize)) | (((val as u32) & 0x7f) << 10usize);
    }
    #[doc = "PDM_CLK frequency divisor."]
    #[must_use]
    #[inline(always)]
    pub const fn mclkdiv(&self) -> super::vals::Mclkdiv {
        let val = (self.0 >> 17usize) & 0x03;
        super::vals::Mclkdiv::from_bits(val as u8)
    }
    #[doc = "PDM_CLK frequency divisor."]
    #[inline(always)]
    pub const fn set_mclkdiv(&mut self, val: super::vals::Mclkdiv) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val.to_bits() as u32) & 0x03) << 17usize);
    }
    #[doc = "Left channel PGA gain."]
    #[must_use]
    #[inline(always)]
    pub const fn pgaleft(&self) -> super::vals::Pgaleft {
        let val = (self.0 >> 21usize) & 0x1f;
        super::vals::Pgaleft::from_bits(val as u8)
    }
    #[doc = "Left channel PGA gain."]
    #[inline(always)]
    pub const fn set_pgaleft(&mut self, val: super::vals::Pgaleft) {
        self.0 = (self.0 & !(0x1f << 21usize)) | (((val.to_bits() as u32) & 0x1f) << 21usize);
    }
    #[doc = "Right channel PGA gain."]
    #[must_use]
    #[inline(always)]
    pub const fn pgaright(&self) -> super::vals::Pgaright {
        let val = (self.0 >> 26usize) & 0x1f;
        super::vals::Pgaright::from_bits(val as u8)
    }
    #[doc = "Right channel PGA gain."]
    #[inline(always)]
    pub const fn set_pgaright(&mut self, val: super::vals::Pgaright) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val.to_bits() as u32) & 0x1f) << 26usize);
    }
    #[doc = "Left/right channel swap."]
    #[must_use]
    #[inline(always)]
    pub const fn lrswap(&self) -> super::vals::Lrswap {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Lrswap::from_bits(val as u8)
    }
    #[doc = "Left/right channel swap."]
    #[inline(always)]
    pub const fn set_lrswap(&mut self, val: super::vals::Lrswap) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Pcfg {
    #[inline(always)]
    fn default() -> Pcfg {
        Pcfg(0)
    }
}
impl core::fmt::Debug for Pcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcfg")
            .field("pdmcoreen", &self.pdmcoreen())
            .field("softmute", &self.softmute())
            .field("cycles", &self.cycles())
            .field("hpcutoff", &self.hpcutoff())
            .field("adchpd", &self.adchpd())
            .field("sincrate", &self.sincrate())
            .field("mclkdiv", &self.mclkdiv())
            .field("pgaleft", &self.pgaleft())
            .field("pgaright", &self.pgaright())
            .field("lrswap", &self.lrswap())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Pcfg {{ pdmcoreen: {=bool:?}, softmute: {=bool:?}, cycles: {=u8:?}, hpcutoff: {=u8:?}, adchpd: {=bool:?}, sincrate: {=u8:?}, mclkdiv: {:?}, pgaleft: {:?}, pgaright: {:?}, lrswap: {:?} }}" , self . pdmcoreen () , self . softmute () , self . cycles () , self . hpcutoff () , self . adchpd () , self . sincrate () , self . mclkdiv () , self . pgaleft () , self . pgaright () , self . lrswap ())
    }
}
#[doc = "Voice Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vcfg(pub u32);
impl Vcfg {
    #[doc = "Set PCM channels."]
    #[must_use]
    #[inline(always)]
    pub const fn chset(&self) -> super::vals::Chset {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::Chset::from_bits(val as u8)
    }
    #[doc = "Set PCM channels."]
    #[inline(always)]
    pub const fn set_chset(&mut self, val: super::vals::Chset) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "PCM data packing enable."]
    #[must_use]
    #[inline(always)]
    pub const fn pcmpack(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "PCM data packing enable."]
    #[inline(always)]
    pub const fn set_pcmpack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Select PDM input clock source."]
    #[must_use]
    #[inline(always)]
    pub const fn selap(&self) -> super::vals::Selap {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Selap::from_bits(val as u8)
    }
    #[doc = "Select PDM input clock source."]
    #[inline(always)]
    pub const fn set_selap(&mut self, val: super::vals::Selap) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "PDM clock sampling delay."]
    #[must_use]
    #[inline(always)]
    pub const fn dmickdel(&self) -> super::vals::Dmickdel {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Dmickdel::from_bits(val as u8)
    }
    #[doc = "PDM clock sampling delay."]
    #[inline(always)]
    pub const fn set_dmickdel(&mut self, val: super::vals::Dmickdel) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "I2S BCLK input inversion."]
    #[must_use]
    #[inline(always)]
    pub const fn bclkinv(&self) -> super::vals::Bclkinv {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Bclkinv::from_bits(val as u8)
    }
    #[doc = "I2S BCLK input inversion."]
    #[inline(always)]
    pub const fn set_bclkinv(&mut self, val: super::vals::Bclkinv) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "I2S interface enable."]
    #[must_use]
    #[inline(always)]
    pub const fn i2sen(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "I2S interface enable."]
    #[inline(always)]
    pub const fn set_i2sen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable the serial clock."]
    #[must_use]
    #[inline(always)]
    pub const fn pdmclken(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the serial clock."]
    #[inline(always)]
    pub const fn set_pdmclken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Select the PDM input clock."]
    #[must_use]
    #[inline(always)]
    pub const fn pdmclksel(&self) -> super::vals::Pdmclksel {
        let val = (self.0 >> 27usize) & 0x07;
        super::vals::Pdmclksel::from_bits(val as u8)
    }
    #[doc = "Select the PDM input clock."]
    #[inline(always)]
    pub const fn set_pdmclksel(&mut self, val: super::vals::Pdmclksel) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
    }
    #[doc = "Reset the IP core."]
    #[must_use]
    #[inline(always)]
    pub const fn rstb(&self) -> super::vals::Rstb {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Rstb::from_bits(val as u8)
    }
    #[doc = "Reset the IP core."]
    #[inline(always)]
    pub const fn set_rstb(&mut self, val: super::vals::Rstb) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable the IO clock."]
    #[must_use]
    #[inline(always)]
    pub const fn ioclken(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the IO clock."]
    #[inline(always)]
    pub const fn set_ioclken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Vcfg {
    #[inline(always)]
    fn default() -> Vcfg {
        Vcfg(0)
    }
}
impl core::fmt::Debug for Vcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Vcfg")
            .field("chset", &self.chset())
            .field("pcmpack", &self.pcmpack())
            .field("selap", &self.selap())
            .field("dmickdel", &self.dmickdel())
            .field("bclkinv", &self.bclkinv())
            .field("i2sen", &self.i2sen())
            .field("pdmclken", &self.pdmclken())
            .field("pdmclksel", &self.pdmclksel())
            .field("rstb", &self.rstb())
            .field("ioclken", &self.ioclken())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Vcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Vcfg {{ chset: {:?}, pcmpack: {=bool:?}, selap: {:?}, dmickdel: {:?}, bclkinv: {:?}, i2sen: {=bool:?}, pdmclken: {=bool:?}, pdmclksel: {:?}, rstb: {:?}, ioclken: {=bool:?} }}" , self . chset () , self . pcmpack () , self . selap () , self . dmickdel () , self . bclkinv () , self . i2sen () , self . pdmclken () , self . pdmclksel () , self . rstb () , self . ioclken ())
    }
}
#[doc = "Voice Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Voicestat(pub u32);
impl Voicestat {
    #[doc = "Valid 32-bit entries currently in the FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn fifocnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Valid 32-bit entries currently in the FIFO."]
    #[inline(always)]
    pub const fn set_fifocnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Voicestat {
    #[inline(always)]
    fn default() -> Voicestat {
        Voicestat(0)
    }
}
impl core::fmt::Debug for Voicestat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Voicestat")
            .field("fifocnt", &self.fifocnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Voicestat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Voicestat {{ fifocnt: {=u8:?} }}", self.fifocnt())
    }
}
