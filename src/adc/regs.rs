#[doc = "Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "This bit enables the ADC module. While the ADC is enabled, the ADCCFG and SLOT Configuration regsiter settings must remain stable and unchanged. All configuration register settings, slot configuration settings and window comparison settings should be written prior to setting the ADCEN bit to '1'."]
    #[must_use]
    #[inline(always)]
    pub const fn adcen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the ADC module. While the ADC is enabled, the ADCCFG and SLOT Configuration regsiter settings must remain stable and unchanged. All configuration register settings, slot configuration settings and window comparison settings should be written prior to setting the ADCEN bit to '1'."]
    #[inline(always)]
    pub const fn set_adcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit enables Repeating Scan Mode."]
    #[must_use]
    #[inline(always)]
    pub const fn rpten(&self) -> super::vals::Rpten {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Rpten::from_bits(val as u8)
    }
    #[doc = "This bit enables Repeating Scan Mode."]
    #[inline(always)]
    pub const fn set_rpten(&mut self, val: super::vals::Rpten) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Select power mode to enter between active scans."]
    #[must_use]
    #[inline(always)]
    pub const fn lpmode(&self) -> super::vals::Lpmode {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Lpmode::from_bits(val as u8)
    }
    #[doc = "Select power mode to enter between active scans."]
    #[inline(always)]
    pub const fn set_lpmode(&mut self, val: super::vals::Lpmode) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Clock mode register."]
    #[must_use]
    #[inline(always)]
    pub const fn ckmode(&self) -> super::vals::Ckmode {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ckmode::from_bits(val as u8)
    }
    #[doc = "Clock mode register."]
    #[inline(always)]
    pub const fn set_ckmode(&mut self, val: super::vals::Ckmode) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Select the ADC reference voltage."]
    #[must_use]
    #[inline(always)]
    pub const fn refsel(&self) -> super::vals::Refsel {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Refsel::from_bits(val as u8)
    }
    #[doc = "Select the ADC reference voltage."]
    #[inline(always)]
    pub const fn set_refsel(&mut self, val: super::vals::Refsel) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Destructive FIFO Read Enable. Setting this will enable FIFO pop upon reading the FIFOPR register."]
    #[must_use]
    #[inline(always)]
    pub const fn dfiforden(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Destructive FIFO Read Enable. Setting this will enable FIFO pop upon reading the FIFOPR register."]
    #[inline(always)]
    pub const fn set_dfiforden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Select the ADC trigger source."]
    #[must_use]
    #[inline(always)]
    pub const fn trigsel(&self) -> super::vals::Trigsel {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::Trigsel::from_bits(val as u8)
    }
    #[doc = "Select the ADC trigger source."]
    #[inline(always)]
    pub const fn set_trigsel(&mut self, val: super::vals::Trigsel) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "This bit selects the ADC trigger polarity for external off chip triggers."]
    #[must_use]
    #[inline(always)]
    pub const fn trigpol(&self) -> super::vals::Trigpol {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Trigpol::from_bits(val as u8)
    }
    #[doc = "This bit selects the ADC trigger polarity for external off chip triggers."]
    #[inline(always)]
    pub const fn set_trigpol(&mut self, val: super::vals::Trigpol) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Select the source and frequency for the ADC clock. All values not enumerated below are undefined."]
    #[must_use]
    #[inline(always)]
    pub const fn clksel(&self) -> super::vals::Clksel {
        let val = (self.0 >> 24usize) & 0x03;
        super::vals::Clksel::from_bits(val as u8)
    }
    #[doc = "Select the source and frequency for the ADC clock. All values not enumerated below are undefined."]
    #[inline(always)]
    pub const fn set_clksel(&mut self, val: super::vals::Clksel) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
    }
}
impl Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        Cfg(0)
    }
}
impl core::fmt::Debug for Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg")
            .field("adcen", &self.adcen())
            .field("rpten", &self.rpten())
            .field("lpmode", &self.lpmode())
            .field("ckmode", &self.ckmode())
            .field("refsel", &self.refsel())
            .field("dfiforden", &self.dfiforden())
            .field("trigsel", &self.trigsel())
            .field("trigpol", &self.trigpol())
            .field("clksel", &self.clksel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cfg {{ adcen: {=bool:?}, rpten: {:?}, lpmode: {:?}, ckmode: {:?}, refsel: {:?}, dfiforden: {=bool:?}, trigsel: {:?}, trigpol: {:?}, clksel: {:?} }}" , self . adcen () , self . rpten () , self . lpmode () , self . ckmode () , self . refsel () , self . dfiforden () , self . trigsel () , self . trigpol () , self . clksel ())
    }
}
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
    #[doc = "Enables dynamic priority based on FIFO fullness. When FIFO is full, priority is automatically set to HIGH. Otherwise, DMAPRI is used."]
    #[must_use]
    #[inline(always)]
    pub const fn dmadynpri(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enables dynamic priority based on FIFO fullness. When FIFO is full, priority is automatically set to HIGH. Otherwise, DMAPRI is used."]
    #[inline(always)]
    pub const fn set_dmadynpri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Halt New ADC conversions until DMA Status DMAERR and DMACPL Cleared."]
    #[must_use]
    #[inline(always)]
    pub const fn dmahonstat(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Halt New ADC conversions until DMA Status DMAERR and DMACPL Cleared."]
    #[inline(always)]
    pub const fn set_dmahonstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Mask the FIFOCNT and SLOTNUM when transferring FIFO contents to memory."]
    #[must_use]
    #[inline(always)]
    pub const fn dmamsk(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Mask the FIFOCNT and SLOTNUM when transferring FIFO contents to memory."]
    #[inline(always)]
    pub const fn set_dmamsk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Power Off the ADC System upon DMACPL."]
    #[must_use]
    #[inline(always)]
    pub const fn dpwroff(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Power Off the ADC System upon DMACPL."]
    #[inline(always)]
    pub const fn set_dpwroff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
            .field("dmadynpri", &self.dmadynpri())
            .field("dmahonstat", &self.dmahonstat())
            .field("dmamsk", &self.dmamsk())
            .field("dpwroff", &self.dpwroff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmacfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dmacfg {{ dmaen: {=bool:?}, dmadir: {:?}, dmapri: {:?}, dmadynpri: {=bool:?}, dmahonstat: {=bool:?}, dmamsk: {=bool:?}, dpwroff: {=bool:?} }}" , self . dmaen () , self . dmadir () , self . dmapri () , self . dmadynpri () , self . dmahonstat () , self . dmamsk () , self . dpwroff ())
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
    #[doc = "DMA Target Address."]
    #[must_use]
    #[inline(always)]
    pub const fn ltargaddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0007_ffff;
        val as u32
    }
    #[doc = "DMA Target Address."]
    #[inline(always)]
    pub const fn set_ltargaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
    }
    #[doc = "SRAM Target."]
    #[must_use]
    #[inline(always)]
    pub const fn utargaddr(&self) -> u16 {
        let val = (self.0 >> 19usize) & 0x1fff;
        val as u16
    }
    #[doc = "SRAM Target."]
    #[inline(always)]
    pub const fn set_utargaddr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 19usize)) | (((val as u32) & 0x1fff) << 19usize);
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
    #[doc = "Total Transfer Count."]
    #[must_use]
    #[inline(always)]
    pub const fn totcount(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0xffff;
        val as u16
    }
    #[doc = "Total Transfer Count."]
    #[inline(always)]
    pub const fn set_totcount(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 2usize)) | (((val as u32) & 0xffff) << 2usize);
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
        defmt::write!(f, "Dmatotcount {{ totcount: {=u16:?} }}", self.totcount())
    }
}
#[doc = "DMA Trigger Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmatrigen(pub u32);
impl Dmatrigen {
    #[doc = "Trigger DMA upon FIFO 75 percent Full."]
    #[must_use]
    #[inline(always)]
    pub const fn dfifo75(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger DMA upon FIFO 75 percent Full."]
    #[inline(always)]
    pub const fn set_dfifo75(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger DMA upon FIFO 100 percent Full."]
    #[must_use]
    #[inline(always)]
    pub const fn dfifofull(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger DMA upon FIFO 100 percent Full."]
    #[inline(always)]
    pub const fn set_dfifofull(&mut self, val: bool) {
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
            .field("dfifo75", &self.dfifo75())
            .field("dfifofull", &self.dfifofull())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmatrigen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmatrigen {{ dfifo75: {=bool:?}, dfifofull: {=bool:?} }}",
            self.dfifo75(),
            self.dfifofull()
        )
    }
}
#[doc = "DMA Trigger Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmatrigstat(pub u32);
impl Dmatrigstat {
    #[doc = "Triggered DMA from FIFO 75 percent Full."]
    #[must_use]
    #[inline(always)]
    pub const fn d75stat(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Triggered DMA from FIFO 75 percent Full."]
    #[inline(always)]
    pub const fn set_d75stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Triggered DMA from FIFO 100 percent Full."]
    #[must_use]
    #[inline(always)]
    pub const fn dfullstat(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Triggered DMA from FIFO 100 percent Full."]
    #[inline(always)]
    pub const fn set_dfullstat(&mut self, val: bool) {
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
            .field("d75stat", &self.d75stat())
            .field("dfullstat", &self.dfullstat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmatrigstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmatrigstat {{ d75stat: {=bool:?}, dfullstat: {=bool:?} }}",
            self.d75stat(),
            self.dfullstat()
        )
    }
}
#[doc = "FIFO Data and Valid Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifo(pub u32);
impl Fifo {
    #[doc = "Oldest data in the FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Oldest data in the FIFO."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
    #[doc = "Number of valid entries in the ADC FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0xff;
        val as u8
    }
    #[doc = "Number of valid entries in the ADC FIFO."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
    }
    #[doc = "Slot number associated with this FIFO data."]
    #[must_use]
    #[inline(always)]
    pub const fn slotnum(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "Slot number associated with this FIFO data."]
    #[inline(always)]
    pub const fn set_slotnum(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
    #[doc = "RESERVED."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvd(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "RESERVED."]
    #[inline(always)]
    pub const fn set_rsvd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Fifo {
    #[inline(always)]
    fn default() -> Fifo {
        Fifo(0)
    }
}
impl core::fmt::Debug for Fifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifo")
            .field("data", &self.data())
            .field("count", &self.count())
            .field("slotnum", &self.slotnum())
            .field("rsvd", &self.rsvd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fifo {{ data: {=u32:?}, count: {=u8:?}, slotnum: {=u8:?}, rsvd: {=bool:?} }}",
            self.data(),
            self.count(),
            self.slotnum(),
            self.rsvd()
        )
    }
}
#[doc = "FIFO Data and Valid Count Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifopr(pub u32);
impl Fifopr {
    #[doc = "Oldest data in the FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Oldest data in the FIFO."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
    #[doc = "Number of valid entries in the ADC FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0xff;
        val as u8
    }
    #[doc = "Number of valid entries in the ADC FIFO."]
    #[inline(always)]
    pub const fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 20usize)) | (((val as u32) & 0xff) << 20usize);
    }
    #[doc = "Slot number associated with this FIFO data."]
    #[must_use]
    #[inline(always)]
    pub const fn slotnumpr(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "Slot number associated with this FIFO data."]
    #[inline(always)]
    pub const fn set_slotnumpr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
    #[doc = "RESERVED."]
    #[must_use]
    #[inline(always)]
    pub const fn rsvdpr(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "RESERVED."]
    #[inline(always)]
    pub const fn set_rsvdpr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Fifopr {
    #[inline(always)]
    fn default() -> Fifopr {
        Fifopr(0)
    }
}
impl core::fmt::Debug for Fifopr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifopr")
            .field("data", &self.data())
            .field("count", &self.count())
            .field("slotnumpr", &self.slotnumpr())
            .field("rsvdpr", &self.rsvdpr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifopr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fifopr {{ data: {=u32:?}, count: {=u8:?}, slotnumpr: {=u8:?}, rsvdpr: {=bool:?} }}",
            self.data(),
            self.count(),
            self.slotnumpr(),
            self.rsvdpr()
        )
    }
}
#[doc = "ADC Interrupt registers: Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intclr(pub u32);
impl Intclr {
    #[doc = "ADC conversion complete interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn cnvcmp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ADC conversion complete interrupt."]
    #[inline(always)]
    pub const fn set_cnvcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC scan complete interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn scncmp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ADC scan complete interrupt."]
    #[inline(always)]
    pub const fn set_scncmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO 75 percent full interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoovr1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 75 percent full interrupt."]
    #[inline(always)]
    pub const fn set_fifoovr1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FIFO 100 percent full interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoovr2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 100 percent full interrupt."]
    #[inline(always)]
    pub const fn set_fifoovr2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Window comparator voltage excursion interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wcexc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Window comparator voltage excursion interrupt."]
    #[inline(always)]
    pub const fn set_wcexc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Window comparator voltage incursion interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wcinc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Window comparator voltage incursion interrupt."]
    #[inline(always)]
    pub const fn set_wcinc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA Transfer Complete."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Transfer Complete."]
    #[inline(always)]
    pub const fn set_dcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA Error Condition."]
    #[must_use]
    #[inline(always)]
    pub const fn derr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error Condition."]
    #[inline(always)]
    pub const fn set_derr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
            .field("cnvcmp", &self.cnvcmp())
            .field("scncmp", &self.scncmp())
            .field("fifoovr1", &self.fifoovr1())
            .field("fifoovr2", &self.fifoovr2())
            .field("wcexc", &self.wcexc())
            .field("wcinc", &self.wcinc())
            .field("dcmp", &self.dcmp())
            .field("derr", &self.derr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intclr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Intclr {{ cnvcmp: {=bool:?}, scncmp: {=bool:?}, fifoovr1: {=bool:?}, fifoovr2: {=bool:?}, wcexc: {=bool:?}, wcinc: {=bool:?}, dcmp: {=bool:?}, derr: {=bool:?} }}" , self . cnvcmp () , self . scncmp () , self . fifoovr1 () , self . fifoovr2 () , self . wcexc () , self . wcinc () , self . dcmp () , self . derr ())
    }
}
#[doc = "ADC Interrupt registers: Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "ADC conversion complete interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn cnvcmp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ADC conversion complete interrupt."]
    #[inline(always)]
    pub const fn set_cnvcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC scan complete interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn scncmp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ADC scan complete interrupt."]
    #[inline(always)]
    pub const fn set_scncmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO 75 percent full interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoovr1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 75 percent full interrupt."]
    #[inline(always)]
    pub const fn set_fifoovr1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FIFO 100 percent full interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoovr2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 100 percent full interrupt."]
    #[inline(always)]
    pub const fn set_fifoovr2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Window comparator voltage excursion interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wcexc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Window comparator voltage excursion interrupt."]
    #[inline(always)]
    pub const fn set_wcexc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Window comparator voltage incursion interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wcinc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Window comparator voltage incursion interrupt."]
    #[inline(always)]
    pub const fn set_wcinc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA Transfer Complete."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Transfer Complete."]
    #[inline(always)]
    pub const fn set_dcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA Error Condition."]
    #[must_use]
    #[inline(always)]
    pub const fn derr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error Condition."]
    #[inline(always)]
    pub const fn set_derr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
            .field("cnvcmp", &self.cnvcmp())
            .field("scncmp", &self.scncmp())
            .field("fifoovr1", &self.fifoovr1())
            .field("fifoovr2", &self.fifoovr2())
            .field("wcexc", &self.wcexc())
            .field("wcinc", &self.wcinc())
            .field("dcmp", &self.dcmp())
            .field("derr", &self.derr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Inten {{ cnvcmp: {=bool:?}, scncmp: {=bool:?}, fifoovr1: {=bool:?}, fifoovr2: {=bool:?}, wcexc: {=bool:?}, wcinc: {=bool:?}, dcmp: {=bool:?}, derr: {=bool:?} }}" , self . cnvcmp () , self . scncmp () , self . fifoovr1 () , self . fifoovr2 () , self . wcexc () , self . wcinc () , self . dcmp () , self . derr ())
    }
}
#[doc = "ADC Interrupt registers: Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intset(pub u32);
impl Intset {
    #[doc = "ADC conversion complete interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn cnvcmp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ADC conversion complete interrupt."]
    #[inline(always)]
    pub const fn set_cnvcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC scan complete interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn scncmp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ADC scan complete interrupt."]
    #[inline(always)]
    pub const fn set_scncmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO 75 percent full interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoovr1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 75 percent full interrupt."]
    #[inline(always)]
    pub const fn set_fifoovr1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FIFO 100 percent full interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoovr2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 100 percent full interrupt."]
    #[inline(always)]
    pub const fn set_fifoovr2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Window comparator voltage excursion interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wcexc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Window comparator voltage excursion interrupt."]
    #[inline(always)]
    pub const fn set_wcexc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Window comparator voltage incursion interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wcinc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Window comparator voltage incursion interrupt."]
    #[inline(always)]
    pub const fn set_wcinc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA Transfer Complete."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Transfer Complete."]
    #[inline(always)]
    pub const fn set_dcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA Error Condition."]
    #[must_use]
    #[inline(always)]
    pub const fn derr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error Condition."]
    #[inline(always)]
    pub const fn set_derr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
            .field("cnvcmp", &self.cnvcmp())
            .field("scncmp", &self.scncmp())
            .field("fifoovr1", &self.fifoovr1())
            .field("fifoovr2", &self.fifoovr2())
            .field("wcexc", &self.wcexc())
            .field("wcinc", &self.wcinc())
            .field("dcmp", &self.dcmp())
            .field("derr", &self.derr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intset {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Intset {{ cnvcmp: {=bool:?}, scncmp: {=bool:?}, fifoovr1: {=bool:?}, fifoovr2: {=bool:?}, wcexc: {=bool:?}, wcinc: {=bool:?}, dcmp: {=bool:?}, derr: {=bool:?} }}" , self . cnvcmp () , self . scncmp () , self . fifoovr1 () , self . fifoovr2 () , self . wcexc () , self . wcinc () , self . dcmp () , self . derr ())
    }
}
#[doc = "ADC Interrupt registers: Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "ADC conversion complete interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn cnvcmp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "ADC conversion complete interrupt."]
    #[inline(always)]
    pub const fn set_cnvcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "ADC scan complete interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn scncmp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "ADC scan complete interrupt."]
    #[inline(always)]
    pub const fn set_scncmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO 75 percent full interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoovr1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 75 percent full interrupt."]
    #[inline(always)]
    pub const fn set_fifoovr1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FIFO 100 percent full interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoovr2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO 100 percent full interrupt."]
    #[inline(always)]
    pub const fn set_fifoovr2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Window comparator voltage excursion interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wcexc(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Window comparator voltage excursion interrupt."]
    #[inline(always)]
    pub const fn set_wcexc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Window comparator voltage incursion interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn wcinc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Window comparator voltage incursion interrupt."]
    #[inline(always)]
    pub const fn set_wcinc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA Transfer Complete."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Transfer Complete."]
    #[inline(always)]
    pub const fn set_dcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA Error Condition."]
    #[must_use]
    #[inline(always)]
    pub const fn derr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error Condition."]
    #[inline(always)]
    pub const fn set_derr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
            .field("cnvcmp", &self.cnvcmp())
            .field("scncmp", &self.scncmp())
            .field("fifoovr1", &self.fifoovr1())
            .field("fifoovr2", &self.fifoovr2())
            .field("wcexc", &self.wcexc())
            .field("wcinc", &self.wcinc())
            .field("dcmp", &self.dcmp())
            .field("derr", &self.derr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Intstat {{ cnvcmp: {=bool:?}, scncmp: {=bool:?}, fifoovr1: {=bool:?}, fifoovr2: {=bool:?}, wcexc: {=bool:?}, wcinc: {=bool:?}, dcmp: {=bool:?}, derr: {=bool:?} }}" , self . cnvcmp () , self . scncmp () , self . fifoovr1 () , self . fifoovr2 () , self . wcexc () , self . wcinc () , self . dcmp () , self . derr ())
    }
}
#[doc = "Scale Window Comparator Limits."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scwlim(pub u32);
impl Scwlim {
    #[doc = "Scale the window limits compare values per precision mode. When set to 0x0 (default), the values in the 20-bit limits registers will compare directly with the FIFO values regardless of the precision mode the slot is configured to. When set to 0x1, the compare values will be divided by the difference in precision bits while performing the window limit comparisons."]
    #[must_use]
    #[inline(always)]
    pub const fn scwlimen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Scale the window limits compare values per precision mode. When set to 0x0 (default), the values in the 20-bit limits registers will compare directly with the FIFO values regardless of the precision mode the slot is configured to. When set to 0x1, the compare values will be divided by the difference in precision bits while performing the window limit comparisons."]
    #[inline(always)]
    pub const fn set_scwlimen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Scwlim {
    #[inline(always)]
    fn default() -> Scwlim {
        Scwlim(0)
    }
}
impl core::fmt::Debug for Scwlim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scwlim")
            .field("scwlimen", &self.scwlimen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scwlim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Scwlim {{ scwlimen: {=bool:?} }}", self.scwlimen())
    }
}
#[doc = "Slot 0 Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sl0cfg(pub u32);
impl Sl0cfg {
    #[doc = "This bit enables slot 0 for ADC conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn slen0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables slot 0 for ADC conversions."]
    #[inline(always)]
    pub const fn set_slen0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit enables the window compare function for slot 0."]
    #[must_use]
    #[inline(always)]
    pub const fn wcen0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the window compare function for slot 0."]
    #[inline(always)]
    pub const fn set_wcen0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Select one of the 14 channel inputs for this slot."]
    #[must_use]
    #[inline(always)]
    pub const fn chsel0(&self) -> super::vals::Chsel0 {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Chsel0::from_bits(val as u8)
    }
    #[doc = "Select one of the 14 channel inputs for this slot."]
    #[inline(always)]
    pub const fn set_chsel0(&mut self, val: super::vals::Chsel0) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Set the Precision Mode For Slot."]
    #[must_use]
    #[inline(always)]
    pub const fn prmode0(&self) -> super::vals::Prmode0 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Prmode0::from_bits(val as u8)
    }
    #[doc = "Set the Precision Mode For Slot."]
    #[inline(always)]
    pub const fn set_prmode0(&mut self, val: super::vals::Prmode0) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Select the number of measurements to average in the accumulate divide module for this slot."]
    #[must_use]
    #[inline(always)]
    pub const fn adsel0(&self) -> super::vals::Adsel0 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Adsel0::from_bits(val as u8)
    }
    #[doc = "Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub const fn set_adsel0(&mut self, val: super::vals::Adsel0) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for Sl0cfg {
    #[inline(always)]
    fn default() -> Sl0cfg {
        Sl0cfg(0)
    }
}
impl core::fmt::Debug for Sl0cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sl0cfg")
            .field("slen0", &self.slen0())
            .field("wcen0", &self.wcen0())
            .field("chsel0", &self.chsel0())
            .field("prmode0", &self.prmode0())
            .field("adsel0", &self.adsel0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sl0cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Sl0cfg {{ slen0: {=bool:?}, wcen0: {=bool:?}, chsel0: {:?}, prmode0: {:?}, adsel0: {:?} }}" , self . slen0 () , self . wcen0 () , self . chsel0 () , self . prmode0 () , self . adsel0 ())
    }
}
#[doc = "Slot 1 Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sl1cfg(pub u32);
impl Sl1cfg {
    #[doc = "This bit enables slot 1 for ADC conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn slen1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables slot 1 for ADC conversions."]
    #[inline(always)]
    pub const fn set_slen1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit enables the window compare function for slot 1."]
    #[must_use]
    #[inline(always)]
    pub const fn wcen1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the window compare function for slot 1."]
    #[inline(always)]
    pub const fn set_wcen1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Select one of the 14 channel inputs for this slot."]
    #[must_use]
    #[inline(always)]
    pub const fn chsel1(&self) -> super::vals::Chsel1 {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Chsel1::from_bits(val as u8)
    }
    #[doc = "Select one of the 14 channel inputs for this slot."]
    #[inline(always)]
    pub const fn set_chsel1(&mut self, val: super::vals::Chsel1) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Set the Precision Mode For Slot."]
    #[must_use]
    #[inline(always)]
    pub const fn prmode1(&self) -> super::vals::Prmode1 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Prmode1::from_bits(val as u8)
    }
    #[doc = "Set the Precision Mode For Slot."]
    #[inline(always)]
    pub const fn set_prmode1(&mut self, val: super::vals::Prmode1) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Select the number of measurements to average in the accumulate divide module for this slot."]
    #[must_use]
    #[inline(always)]
    pub const fn adsel1(&self) -> super::vals::Adsel1 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Adsel1::from_bits(val as u8)
    }
    #[doc = "Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub const fn set_adsel1(&mut self, val: super::vals::Adsel1) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for Sl1cfg {
    #[inline(always)]
    fn default() -> Sl1cfg {
        Sl1cfg(0)
    }
}
impl core::fmt::Debug for Sl1cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sl1cfg")
            .field("slen1", &self.slen1())
            .field("wcen1", &self.wcen1())
            .field("chsel1", &self.chsel1())
            .field("prmode1", &self.prmode1())
            .field("adsel1", &self.adsel1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sl1cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Sl1cfg {{ slen1: {=bool:?}, wcen1: {=bool:?}, chsel1: {:?}, prmode1: {:?}, adsel1: {:?} }}" , self . slen1 () , self . wcen1 () , self . chsel1 () , self . prmode1 () , self . adsel1 ())
    }
}
#[doc = "Slot 2 Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sl2cfg(pub u32);
impl Sl2cfg {
    #[doc = "This bit enables slot 2 for ADC conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn slen2(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables slot 2 for ADC conversions."]
    #[inline(always)]
    pub const fn set_slen2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit enables the window compare function for slot 2."]
    #[must_use]
    #[inline(always)]
    pub const fn wcen2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the window compare function for slot 2."]
    #[inline(always)]
    pub const fn set_wcen2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Select one of the 14 channel inputs for this slot."]
    #[must_use]
    #[inline(always)]
    pub const fn chsel2(&self) -> super::vals::Chsel2 {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Chsel2::from_bits(val as u8)
    }
    #[doc = "Select one of the 14 channel inputs for this slot."]
    #[inline(always)]
    pub const fn set_chsel2(&mut self, val: super::vals::Chsel2) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Set the Precision Mode For Slot."]
    #[must_use]
    #[inline(always)]
    pub const fn prmode2(&self) -> super::vals::Prmode2 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Prmode2::from_bits(val as u8)
    }
    #[doc = "Set the Precision Mode For Slot."]
    #[inline(always)]
    pub const fn set_prmode2(&mut self, val: super::vals::Prmode2) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Select the number of measurements to average in the accumulate divide module for this slot."]
    #[must_use]
    #[inline(always)]
    pub const fn adsel2(&self) -> super::vals::Adsel2 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Adsel2::from_bits(val as u8)
    }
    #[doc = "Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub const fn set_adsel2(&mut self, val: super::vals::Adsel2) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for Sl2cfg {
    #[inline(always)]
    fn default() -> Sl2cfg {
        Sl2cfg(0)
    }
}
impl core::fmt::Debug for Sl2cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sl2cfg")
            .field("slen2", &self.slen2())
            .field("wcen2", &self.wcen2())
            .field("chsel2", &self.chsel2())
            .field("prmode2", &self.prmode2())
            .field("adsel2", &self.adsel2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sl2cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Sl2cfg {{ slen2: {=bool:?}, wcen2: {=bool:?}, chsel2: {:?}, prmode2: {:?}, adsel2: {:?} }}" , self . slen2 () , self . wcen2 () , self . chsel2 () , self . prmode2 () , self . adsel2 ())
    }
}
#[doc = "Slot 3 Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sl3cfg(pub u32);
impl Sl3cfg {
    #[doc = "This bit enables slot 3 for ADC conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn slen3(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables slot 3 for ADC conversions."]
    #[inline(always)]
    pub const fn set_slen3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit enables the window compare function for slot 3."]
    #[must_use]
    #[inline(always)]
    pub const fn wcen3(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the window compare function for slot 3."]
    #[inline(always)]
    pub const fn set_wcen3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Select one of the 14 channel inputs for this slot."]
    #[must_use]
    #[inline(always)]
    pub const fn chsel3(&self) -> super::vals::Chsel3 {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Chsel3::from_bits(val as u8)
    }
    #[doc = "Select one of the 14 channel inputs for this slot."]
    #[inline(always)]
    pub const fn set_chsel3(&mut self, val: super::vals::Chsel3) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Set the Precision Mode For Slot."]
    #[must_use]
    #[inline(always)]
    pub const fn prmode3(&self) -> super::vals::Prmode3 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Prmode3::from_bits(val as u8)
    }
    #[doc = "Set the Precision Mode For Slot."]
    #[inline(always)]
    pub const fn set_prmode3(&mut self, val: super::vals::Prmode3) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Select the number of measurements to average in the accumulate divide module for this slot."]
    #[must_use]
    #[inline(always)]
    pub const fn adsel3(&self) -> super::vals::Adsel3 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Adsel3::from_bits(val as u8)
    }
    #[doc = "Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub const fn set_adsel3(&mut self, val: super::vals::Adsel3) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for Sl3cfg {
    #[inline(always)]
    fn default() -> Sl3cfg {
        Sl3cfg(0)
    }
}
impl core::fmt::Debug for Sl3cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sl3cfg")
            .field("slen3", &self.slen3())
            .field("wcen3", &self.wcen3())
            .field("chsel3", &self.chsel3())
            .field("prmode3", &self.prmode3())
            .field("adsel3", &self.adsel3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sl3cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Sl3cfg {{ slen3: {=bool:?}, wcen3: {=bool:?}, chsel3: {:?}, prmode3: {:?}, adsel3: {:?} }}" , self . slen3 () , self . wcen3 () , self . chsel3 () , self . prmode3 () , self . adsel3 ())
    }
}
#[doc = "Slot 4 Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sl4cfg(pub u32);
impl Sl4cfg {
    #[doc = "This bit enables slot 4 for ADC conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn slen4(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables slot 4 for ADC conversions."]
    #[inline(always)]
    pub const fn set_slen4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit enables the window compare function for slot 4."]
    #[must_use]
    #[inline(always)]
    pub const fn wcen4(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the window compare function for slot 4."]
    #[inline(always)]
    pub const fn set_wcen4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Select one of the 14 channel inputs for this slot."]
    #[must_use]
    #[inline(always)]
    pub const fn chsel4(&self) -> super::vals::Chsel4 {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Chsel4::from_bits(val as u8)
    }
    #[doc = "Select one of the 14 channel inputs for this slot."]
    #[inline(always)]
    pub const fn set_chsel4(&mut self, val: super::vals::Chsel4) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Set the Precision Mode For Slot."]
    #[must_use]
    #[inline(always)]
    pub const fn prmode4(&self) -> super::vals::Prmode4 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Prmode4::from_bits(val as u8)
    }
    #[doc = "Set the Precision Mode For Slot."]
    #[inline(always)]
    pub const fn set_prmode4(&mut self, val: super::vals::Prmode4) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Select the number of measurements to average in the accumulate divide module for this slot."]
    #[must_use]
    #[inline(always)]
    pub const fn adsel4(&self) -> super::vals::Adsel4 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Adsel4::from_bits(val as u8)
    }
    #[doc = "Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub const fn set_adsel4(&mut self, val: super::vals::Adsel4) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for Sl4cfg {
    #[inline(always)]
    fn default() -> Sl4cfg {
        Sl4cfg(0)
    }
}
impl core::fmt::Debug for Sl4cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sl4cfg")
            .field("slen4", &self.slen4())
            .field("wcen4", &self.wcen4())
            .field("chsel4", &self.chsel4())
            .field("prmode4", &self.prmode4())
            .field("adsel4", &self.adsel4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sl4cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Sl4cfg {{ slen4: {=bool:?}, wcen4: {=bool:?}, chsel4: {:?}, prmode4: {:?}, adsel4: {:?} }}" , self . slen4 () , self . wcen4 () , self . chsel4 () , self . prmode4 () , self . adsel4 ())
    }
}
#[doc = "Slot 5 Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sl5cfg(pub u32);
impl Sl5cfg {
    #[doc = "This bit enables slot 5 for ADC conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn slen5(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables slot 5 for ADC conversions."]
    #[inline(always)]
    pub const fn set_slen5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit enables the window compare function for slot 5."]
    #[must_use]
    #[inline(always)]
    pub const fn wcen5(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the window compare function for slot 5."]
    #[inline(always)]
    pub const fn set_wcen5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Select one of the 14 channel inputs for this slot."]
    #[must_use]
    #[inline(always)]
    pub const fn chsel5(&self) -> super::vals::Chsel5 {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Chsel5::from_bits(val as u8)
    }
    #[doc = "Select one of the 14 channel inputs for this slot."]
    #[inline(always)]
    pub const fn set_chsel5(&mut self, val: super::vals::Chsel5) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Set the Precision Mode For Slot."]
    #[must_use]
    #[inline(always)]
    pub const fn prmode5(&self) -> super::vals::Prmode5 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Prmode5::from_bits(val as u8)
    }
    #[doc = "Set the Precision Mode For Slot."]
    #[inline(always)]
    pub const fn set_prmode5(&mut self, val: super::vals::Prmode5) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Select number of measurements to average in the accumulate divide module for this slot."]
    #[must_use]
    #[inline(always)]
    pub const fn adsel5(&self) -> super::vals::Adsel5 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Adsel5::from_bits(val as u8)
    }
    #[doc = "Select number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub const fn set_adsel5(&mut self, val: super::vals::Adsel5) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for Sl5cfg {
    #[inline(always)]
    fn default() -> Sl5cfg {
        Sl5cfg(0)
    }
}
impl core::fmt::Debug for Sl5cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sl5cfg")
            .field("slen5", &self.slen5())
            .field("wcen5", &self.wcen5())
            .field("chsel5", &self.chsel5())
            .field("prmode5", &self.prmode5())
            .field("adsel5", &self.adsel5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sl5cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Sl5cfg {{ slen5: {=bool:?}, wcen5: {=bool:?}, chsel5: {:?}, prmode5: {:?}, adsel5: {:?} }}" , self . slen5 () , self . wcen5 () , self . chsel5 () , self . prmode5 () , self . adsel5 ())
    }
}
#[doc = "Slot 6 Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sl6cfg(pub u32);
impl Sl6cfg {
    #[doc = "This bit enables slot 6 for ADC conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn slen6(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables slot 6 for ADC conversions."]
    #[inline(always)]
    pub const fn set_slen6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit enables the window compare function for slot 6."]
    #[must_use]
    #[inline(always)]
    pub const fn wcen6(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the window compare function for slot 6."]
    #[inline(always)]
    pub const fn set_wcen6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Select one of the 14 channel inputs for this slot."]
    #[must_use]
    #[inline(always)]
    pub const fn chsel6(&self) -> super::vals::Chsel6 {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Chsel6::from_bits(val as u8)
    }
    #[doc = "Select one of the 14 channel inputs for this slot."]
    #[inline(always)]
    pub const fn set_chsel6(&mut self, val: super::vals::Chsel6) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Set the Precision Mode For Slot."]
    #[must_use]
    #[inline(always)]
    pub const fn prmode6(&self) -> super::vals::Prmode6 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Prmode6::from_bits(val as u8)
    }
    #[doc = "Set the Precision Mode For Slot."]
    #[inline(always)]
    pub const fn set_prmode6(&mut self, val: super::vals::Prmode6) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Select the number of measurements to average in the accumulate divide module for this slot."]
    #[must_use]
    #[inline(always)]
    pub const fn adsel6(&self) -> super::vals::Adsel6 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Adsel6::from_bits(val as u8)
    }
    #[doc = "Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub const fn set_adsel6(&mut self, val: super::vals::Adsel6) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for Sl6cfg {
    #[inline(always)]
    fn default() -> Sl6cfg {
        Sl6cfg(0)
    }
}
impl core::fmt::Debug for Sl6cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sl6cfg")
            .field("slen6", &self.slen6())
            .field("wcen6", &self.wcen6())
            .field("chsel6", &self.chsel6())
            .field("prmode6", &self.prmode6())
            .field("adsel6", &self.adsel6())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sl6cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Sl6cfg {{ slen6: {=bool:?}, wcen6: {=bool:?}, chsel6: {:?}, prmode6: {:?}, adsel6: {:?} }}" , self . slen6 () , self . wcen6 () , self . chsel6 () , self . prmode6 () , self . adsel6 ())
    }
}
#[doc = "Slot 7 Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sl7cfg(pub u32);
impl Sl7cfg {
    #[doc = "This bit enables slot 7 for ADC conversions."]
    #[must_use]
    #[inline(always)]
    pub const fn slen7(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables slot 7 for ADC conversions."]
    #[inline(always)]
    pub const fn set_slen7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit enables the window compare function for slot 7."]
    #[must_use]
    #[inline(always)]
    pub const fn wcen7(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bit enables the window compare function for slot 7."]
    #[inline(always)]
    pub const fn set_wcen7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Select one of the 14 channel inputs for this slot."]
    #[must_use]
    #[inline(always)]
    pub const fn chsel7(&self) -> super::vals::Chsel7 {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Chsel7::from_bits(val as u8)
    }
    #[doc = "Select one of the 14 channel inputs for this slot."]
    #[inline(always)]
    pub const fn set_chsel7(&mut self, val: super::vals::Chsel7) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "Set the Precision Mode For Slot."]
    #[must_use]
    #[inline(always)]
    pub const fn prmode7(&self) -> super::vals::Prmode7 {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Prmode7::from_bits(val as u8)
    }
    #[doc = "Set the Precision Mode For Slot."]
    #[inline(always)]
    pub const fn set_prmode7(&mut self, val: super::vals::Prmode7) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Select the number of measurements to average in the accumulate divide module for this slot."]
    #[must_use]
    #[inline(always)]
    pub const fn adsel7(&self) -> super::vals::Adsel7 {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::Adsel7::from_bits(val as u8)
    }
    #[doc = "Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub const fn set_adsel7(&mut self, val: super::vals::Adsel7) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
}
impl Default for Sl7cfg {
    #[inline(always)]
    fn default() -> Sl7cfg {
        Sl7cfg(0)
    }
}
impl core::fmt::Debug for Sl7cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sl7cfg")
            .field("slen7", &self.slen7())
            .field("wcen7", &self.wcen7())
            .field("chsel7", &self.chsel7())
            .field("prmode7", &self.prmode7())
            .field("adsel7", &self.adsel7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sl7cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Sl7cfg {{ slen7: {=bool:?}, wcen7: {=bool:?}, chsel7: {:?}, prmode7: {:?}, adsel7: {:?} }}" , self . slen7 () , self . wcen7 () , self . chsel7 () , self . prmode7 () , self . adsel7 ())
    }
}
#[doc = "ADC Power Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Indicates the power-status of the ADC."]
    #[must_use]
    #[inline(always)]
    pub const fn pwdstat(&self) -> super::vals::Pwdstat {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pwdstat::from_bits(val as u8)
    }
    #[doc = "Indicates the power-status of the ADC."]
    #[inline(always)]
    pub const fn set_pwdstat(&mut self, val: super::vals::Pwdstat) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
impl core::fmt::Debug for Stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat")
            .field("pwdstat", &self.pwdstat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Stat {{ pwdstat: {:?} }}", self.pwdstat())
    }
}
#[doc = "Software trigger."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swt(pub u32);
impl Swt {
    #[doc = "Writing 0x37 to this register generates a software trigger."]
    #[must_use]
    #[inline(always)]
    pub const fn swt(&self) -> super::vals::Swt {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Swt::from_bits(val as u8)
    }
    #[doc = "Writing 0x37 to this register generates a software trigger."]
    #[inline(always)]
    pub const fn set_swt(&mut self, val: super::vals::Swt) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Swt {
    #[inline(always)]
    fn default() -> Swt {
        Swt(0)
    }
}
impl core::fmt::Debug for Swt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Swt").field("swt", &self.swt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Swt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Swt {{ swt: {:?} }}", self.swt())
    }
}
#[doc = "Window Comparator Lower Limits Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wllim(pub u32);
impl Wllim {
    #[doc = "Sets the lower limit for the window comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn llim(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Sets the lower limit for the window comparator."]
    #[inline(always)]
    pub const fn set_llim(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Wllim {
    #[inline(always)]
    fn default() -> Wllim {
        Wllim(0)
    }
}
impl core::fmt::Debug for Wllim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wllim").field("llim", &self.llim()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wllim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wllim {{ llim: {=u32:?} }}", self.llim())
    }
}
#[doc = "Window Comparator Upper Limits Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wulim(pub u32);
impl Wulim {
    #[doc = "Sets the upper limit for the window comparator."]
    #[must_use]
    #[inline(always)]
    pub const fn ulim(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Sets the upper limit for the window comparator."]
    #[inline(always)]
    pub const fn set_ulim(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Wulim {
    #[inline(always)]
    fn default() -> Wulim {
        Wulim(0)
    }
}
impl core::fmt::Debug for Wulim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wulim").field("ulim", &self.ulim()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wulim {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Wulim {{ ulim: {=u32:?} }}", self.ulim())
    }
}
