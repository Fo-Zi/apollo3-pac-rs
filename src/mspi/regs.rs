#[doc = "MSPI Transfer Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "Flash configuration for XIP and AUTO DMA operations. Controls value for SER (Slave Enable) for XIP operations and address generation for DMA/XIP modes. Also used to configure SPIFRF (frame format)."]
    #[must_use]
    #[inline(always)]
    pub const fn devcfg(&self) -> super::vals::Devcfg {
        let val = (self.0 >> 0usize) & 0x0f;
        super::vals::Devcfg::from_bits(val as u8)
    }
    #[doc = "Flash configuration for XIP and AUTO DMA operations. Controls value for SER (Slave Enable) for XIP operations and address generation for DMA/XIP modes. Also used to configure SPIFRF (frame format)."]
    #[inline(always)]
    pub const fn set_devcfg(&mut self, val: super::vals::Devcfg) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
    }
    #[doc = "Address Size. Address bytes to send from ADDR register name = A1 value = 0x0 desc = Send one address byte enum name = A2 value = 0x1 desc = Send two address bytes enum name = A3 value = 0x2 desc = Send three address bytes enum name = A4 value = 0x3 desc = Send four address bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn asize(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Address Size. Address bytes to send from ADDR register name = A1 value = 0x0 desc = Send one address byte enum name = A2 value = 0x1 desc = Send two address bytes enum name = A3 value = 0x2 desc = Send three address bytes enum name = A4 value = 0x3 desc = Send four address bytes."]
    #[inline(always)]
    pub const fn set_asize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Instruction Size enum name = I8 value = 0x0 desc = Instruction is 1 byte enum name = I16 value = 0x1 desc = Instruction is 2 bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn isize(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Instruction Size enum name = I8 value = 0x0 desc = Instruction is 1 byte enum name = I16 value = 0x1 desc = Instruction is 2 bytes."]
    #[inline(always)]
    pub const fn set_isize(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Separate IO configuration. This bit should be set when the target device has separate MOSI and MISO pins. Respective IN/OUT bits below should be set to map pins."]
    #[must_use]
    #[inline(always)]
    pub const fn sepio(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Separate IO configuration. This bit should be set when the target device has separate MOSI and MISO pins. Respective IN/OUT bits below should be set to map pins."]
    #[inline(always)]
    pub const fn set_sepio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Number of turnaound cycles (for TX->RX transitions). Qualified by ENTURN or XIPENTURN bit field."]
    #[must_use]
    #[inline(always)]
    pub const fn turnaround(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Number of turnaound cycles (for TX->RX transitions). Qualified by ENTURN or XIPENTURN bit field."]
    #[inline(always)]
    pub const fn set_turnaround(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Serial clock phase."]
    #[must_use]
    #[inline(always)]
    pub const fn cpha(&self) -> super::vals::Cpha {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Cpha::from_bits(val as u8)
    }
    #[doc = "Serial clock phase."]
    #[inline(always)]
    pub const fn set_cpha(&mut self, val: super::vals::Cpha) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Serial clock polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn cpol(&self) -> super::vals::Cpol {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Cpol::from_bits(val as u8)
    }
    #[doc = "Serial clock polarity."]
    #[inline(always)]
    pub const fn set_cpol(&mut self, val: super::vals::Cpol) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
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
            .field("devcfg", &self.devcfg())
            .field("asize", &self.asize())
            .field("isize", &self.isize())
            .field("sepio", &self.sepio())
            .field("turnaround", &self.turnaround())
            .field("cpha", &self.cpha())
            .field("cpol", &self.cpol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cfg {{ devcfg: {:?}, asize: {=u8:?}, isize: {=bool:?}, sepio: {=bool:?}, turnaround: {=u8:?}, cpha: {:?}, cpol: {:?} }}" , self . devcfg () , self . asize () , self . isize () , self . sepio () , self . turnaround () , self . cpha () , self . cpol ())
    }
}
#[doc = "CQ Target Read Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cqaddr(pub u32);
impl Cqaddr {
    #[doc = "Address of command queue buffer in SRAM or flash. The buffer address must be aligned to a word boundary."]
    #[must_use]
    #[inline(always)]
    pub const fn cqaddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "Address of command queue buffer in SRAM or flash. The buffer address must be aligned to a word boundary."]
    #[inline(always)]
    pub const fn set_cqaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 0usize)) | (((val as u32) & 0x1fff_ffff) << 0usize);
    }
}
impl Default for Cqaddr {
    #[inline(always)]
    fn default() -> Cqaddr {
        Cqaddr(0)
    }
}
impl core::fmt::Debug for Cqaddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cqaddr")
            .field("cqaddr", &self.cqaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cqaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cqaddr {{ cqaddr: {=u32:?} }}", self.cqaddr())
    }
}
#[doc = "Command Queue Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cqcfg(pub u32);
impl Cqcfg {
    #[doc = "Command queue enable. When set, will enable the processing of the command queue."]
    #[must_use]
    #[inline(always)]
    pub const fn cqen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue enable. When set, will enable the processing of the command queue."]
    #[inline(always)]
    pub const fn set_cqen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Sets the Priority of the command queue dma request."]
    #[must_use]
    #[inline(always)]
    pub const fn cqpri(&self) -> super::vals::Cqpri {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Cqpri::from_bits(val as u8)
    }
    #[doc = "Sets the Priority of the command queue dma request."]
    #[inline(always)]
    pub const fn set_cqpri(&mut self, val: super::vals::Cqpri) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Power off MSPI domain upon completion of DMA operation."]
    #[must_use]
    #[inline(always)]
    pub const fn cqpwroff(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Power off MSPI domain upon completion of DMA operation."]
    #[inline(always)]
    pub const fn set_cqpwroff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Eanble clear of CQMASK after each pause operation. This may be useful when using software flags to pause CQ."]
    #[must_use]
    #[inline(always)]
    pub const fn cqautoclearmask(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Eanble clear of CQMASK after each pause operation. This may be useful when using software flags to pause CQ."]
    #[inline(always)]
    pub const fn set_cqautoclearmask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Cqcfg {
    #[inline(always)]
    fn default() -> Cqcfg {
        Cqcfg(0)
    }
}
impl core::fmt::Debug for Cqcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cqcfg")
            .field("cqen", &self.cqen())
            .field("cqpri", &self.cqpri())
            .field("cqpwroff", &self.cqpwroff())
            .field("cqautoclearmask", &self.cqautoclearmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cqcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cqcfg {{ cqen: {=bool:?}, cqpri: {:?}, cqpwroff: {=bool:?}, cqautoclearmask: {=bool:?} }}" , self . cqen () , self . cqpri () , self . cqpwroff () , self . cqautoclearmask ())
    }
}
#[doc = "Command Queue Current Index."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cqcuridx(pub u32);
impl Cqcuridx {
    #[doc = "Can be used to indicate the current position of the command queue by having CQ operations write this field. A CQ hardware status flag indicates when CURIDX and ENDIDX are not equal, allowing SW to pause the CQ processing until the end index is updated."]
    #[must_use]
    #[inline(always)]
    pub const fn cqcuridx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Can be used to indicate the current position of the command queue by having CQ operations write this field. A CQ hardware status flag indicates when CURIDX and ENDIDX are not equal, allowing SW to pause the CQ processing until the end index is updated."]
    #[inline(always)]
    pub const fn set_cqcuridx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Cqcuridx {
    #[inline(always)]
    fn default() -> Cqcuridx {
        Cqcuridx(0)
    }
}
impl core::fmt::Debug for Cqcuridx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cqcuridx")
            .field("cqcuridx", &self.cqcuridx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cqcuridx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cqcuridx {{ cqcuridx: {=u8:?} }}", self.cqcuridx())
    }
}
#[doc = "Command Queue End Index."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cqendidx(pub u32);
impl Cqendidx {
    #[doc = "Can be used to indicate the end position of the command queue. A CQ hardware status bit indices when CURIDX != ENDIDX so that the CQ can be paused when it reaches the end pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn cqendidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Can be used to indicate the end position of the command queue. A CQ hardware status bit indices when CURIDX != ENDIDX so that the CQ can be paused when it reaches the end pointer."]
    #[inline(always)]
    pub const fn set_cqendidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Cqendidx {
    #[inline(always)]
    fn default() -> Cqendidx {
        Cqendidx(0)
    }
}
impl core::fmt::Debug for Cqendidx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cqendidx")
            .field("cqendidx", &self.cqendidx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cqendidx {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cqendidx {{ cqendidx: {=u8:?} }}", self.cqendidx())
    }
}
#[doc = "Command Queue Flag Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cqflags(pub u32);
impl Cqflags {
    #[doc = "Current flag status (read-only). Bits \\[7:0\\] are software controllable and bits \\[15:8\\] are hardware status."]
    #[must_use]
    #[inline(always)]
    pub const fn cqflags(&self) -> super::vals::Cqflags {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Cqflags::from_bits(val as u16)
    }
    #[doc = "Current flag status (read-only). Bits \\[7:0\\] are software controllable and bits \\[15:8\\] are hardware status."]
    #[inline(always)]
    pub const fn set_cqflags(&mut self, val: super::vals::Cqflags) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cqflags {
    #[inline(always)]
    fn default() -> Cqflags {
        Cqflags(0)
    }
}
impl core::fmt::Debug for Cqflags {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cqflags")
            .field("cqflags", &self.cqflags())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cqflags {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cqflags {{ cqflags: {:?} }}", self.cqflags())
    }
}
#[doc = "Command Queue Pause Mask Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cqpause(pub u32);
impl Cqpause {
    #[doc = "CQ will pause processing until all specified events are satisfied."]
    #[must_use]
    #[inline(always)]
    pub const fn cqmask(&self) -> super::vals::Cqmask {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Cqmask::from_bits(val as u16)
    }
    #[doc = "CQ will pause processing until all specified events are satisfied."]
    #[inline(always)]
    pub const fn set_cqmask(&mut self, val: super::vals::Cqmask) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cqpause {
    #[inline(always)]
    fn default() -> Cqpause {
        Cqpause(0)
    }
}
impl core::fmt::Debug for Cqpause {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cqpause")
            .field("cqmask", &self.cqmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cqpause {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cqpause {{ cqmask: {:?} }}", self.cqmask())
    }
}
#[doc = "Command Queue Flag Set/Clear Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cqsetclear(pub u32);
impl Cqsetclear {
    #[doc = "Set CQFlag status bits. Set has priority over clear if both are high."]
    #[must_use]
    #[inline(always)]
    pub const fn cqfset(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Set CQFlag status bits. Set has priority over clear if both are high."]
    #[inline(always)]
    pub const fn set_cqfset(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Toggle CQFlag status bits."]
    #[must_use]
    #[inline(always)]
    pub const fn cqftoggle(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Toggle CQFlag status bits."]
    #[inline(always)]
    pub const fn set_cqftoggle(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Clear CQFlag status bits."]
    #[must_use]
    #[inline(always)]
    pub const fn cqfclr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Clear CQFlag status bits."]
    #[inline(always)]
    pub const fn set_cqfclr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Cqsetclear {
    #[inline(always)]
    fn default() -> Cqsetclear {
        Cqsetclear(0)
    }
}
impl core::fmt::Debug for Cqsetclear {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cqsetclear")
            .field("cqfset", &self.cqfset())
            .field("cqftoggle", &self.cqftoggle())
            .field("cqfclr", &self.cqfclr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cqsetclear {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cqsetclear {{ cqfset: {=u8:?}, cqftoggle: {=u8:?}, cqfclr: {=u8:?} }}",
            self.cqfset(),
            self.cqftoggle(),
            self.cqfclr()
        )
    }
}
#[doc = "Command Queue Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cqstat(pub u32);
impl Cqstat {
    #[doc = "Command queue Transfer In Progress indicator. 1 will indicate that a CQ transfer is active and this will remain active even when paused waiting for external event."]
    #[must_use]
    #[inline(always)]
    pub const fn cqtip(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue Transfer In Progress indicator. 1 will indicate that a CQ transfer is active and this will remain active even when paused waiting for external event."]
    #[inline(always)]
    pub const fn set_cqtip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Command queue operation Complete. This signals the end of the command queue operation."]
    #[must_use]
    #[inline(always)]
    pub const fn cqcpl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue operation Complete. This signals the end of the command queue operation."]
    #[inline(always)]
    pub const fn set_cqcpl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Command queue processing Error. This active high bit signals that an error was encountered during the CQ operation."]
    #[must_use]
    #[inline(always)]
    pub const fn cqerr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue processing Error. This active high bit signals that an error was encountered during the CQ operation."]
    #[inline(always)]
    pub const fn set_cqerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Command queue is currently paused status."]
    #[must_use]
    #[inline(always)]
    pub const fn cqpaused(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue is currently paused status."]
    #[inline(always)]
    pub const fn set_cqpaused(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Cqstat {
    #[inline(always)]
    fn default() -> Cqstat {
        Cqstat(0)
    }
}
impl core::fmt::Debug for Cqstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cqstat")
            .field("cqtip", &self.cqtip())
            .field("cqcpl", &self.cqcpl())
            .field("cqerr", &self.cqerr())
            .field("cqpaused", &self.cqpaused())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cqstat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cqstat {{ cqtip: {=bool:?}, cqcpl: {=bool:?}, cqerr: {=bool:?}, cqpaused: {=bool:?} }}" , self . cqtip () , self . cqcpl () , self . cqerr () , self . cqpaused ())
    }
}
#[doc = "MSPI PIO Transfer Control/Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Write to 1 to initiate a PIO transaction on the bus (typically the entire register should be written at once with this bit set)."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write to 1 to initiate a PIO transaction on the bus (typically the entire register should be written at once with this bit set)."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Command status: 1 indicates command has completed. Cleared by writing 1 to this bit or starting a new transfer."]
    #[must_use]
    #[inline(always)]
    pub const fn status(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Command status: 1 indicates command has completed. Cleared by writing 1 to this bit or starting a new transfer."]
    #[inline(always)]
    pub const fn set_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Command status: 1 indicates controller is busy (command in progress)."]
    #[must_use]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Command status: 1 indicates controller is busy (command in progress)."]
    #[inline(always)]
    pub const fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Flag indicating that the operation is a command that should be replicated to both devices in paired QUAD mode. This is typically only used when reading/writing configuration registers in paired flash devices (do not set for memory transfers)."]
    #[must_use]
    #[inline(always)]
    pub const fn quadcmd(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Flag indicating that the operation is a command that should be replicated to both devices in paired QUAD mode. This is typically only used when reading/writing configuration registers in paired flash devices (do not set for memory transfers)."]
    #[inline(always)]
    pub const fn set_quadcmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "1 indicates data in FIFO is in big endian format (MSB first); 0 indicates little endian data (default, LSB first)."]
    #[must_use]
    #[inline(always)]
    pub const fn bigendian(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "1 indicates data in FIFO is in big endian format (MSB first); 0 indicates little endian data (default, LSB first)."]
    #[inline(always)]
    pub const fn set_bigendian(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates whether TX->RX turnaround cycles should be enabled for this operation (see TURNAROUND field in CFG register)."]
    #[must_use]
    #[inline(always)]
    pub const fn enturn(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether TX->RX turnaround cycles should be enabled for this operation (see TURNAROUND field in CFG register)."]
    #[inline(always)]
    pub const fn set_enturn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Indicates whether an address phase should be sent (see ADDR register and ASIZE field in CFG register)."]
    #[must_use]
    #[inline(always)]
    pub const fn senda(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether an address phase should be sent (see ADDR register and ASIZE field in CFG register)."]
    #[inline(always)]
    pub const fn set_senda(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Indicates whether an instruction phase should be sent (see INSTR field and ISIZE field in CFG register)."]
    #[must_use]
    #[inline(always)]
    pub const fn sendi(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether an instruction phase should be sent (see INSTR field and ISIZE field in CFG register)."]
    #[inline(always)]
    pub const fn set_sendi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "1 Indicates a TX operation, 0 indicates an RX operation of XFERBYTES."]
    #[must_use]
    #[inline(always)]
    pub const fn txrx(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "1 Indicates a TX operation, 0 indicates an RX operation of XFERBYTES."]
    #[inline(always)]
    pub const fn set_txrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enables data scrambling for PIO opertions. This should only be used for data operations and never for commands to a device."]
    #[must_use]
    #[inline(always)]
    pub const fn pioscramble(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enables data scrambling for PIO opertions. This should only be used for data operations and never for commands to a device."]
    #[inline(always)]
    pub const fn set_pioscramble(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Number of bytes to transmit or receive (based on TXRX bit)."]
    #[must_use]
    #[inline(always)]
    pub const fn xferbytes(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of bytes to transmit or receive (based on TXRX bit)."]
    #[inline(always)]
    pub const fn set_xferbytes(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("start", &self.start())
            .field("status", &self.status())
            .field("busy", &self.busy())
            .field("quadcmd", &self.quadcmd())
            .field("bigendian", &self.bigendian())
            .field("enturn", &self.enturn())
            .field("senda", &self.senda())
            .field("sendi", &self.sendi())
            .field("txrx", &self.txrx())
            .field("pioscramble", &self.pioscramble())
            .field("xferbytes", &self.xferbytes())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ start: {=bool:?}, status: {=bool:?}, busy: {=bool:?}, quadcmd: {=bool:?}, bigendian: {=bool:?}, enturn: {=bool:?}, senda: {=bool:?}, sendi: {=bool:?}, txrx: {=bool:?}, pioscramble: {=bool:?}, xferbytes: {=u16:?} }}" , self . start () , self . status () , self . busy () , self . quadcmd () , self . bigendian () , self . enturn () , self . senda () , self . sendi () , self . txrx () , self . pioscramble () , self . xferbytes ())
    }
}
#[doc = "DMA BYTE Transfer Count."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmabcount(pub u32);
impl Dmabcount {
    #[doc = "Burst transfer size in bytes. This is the number of bytes transferred when a FIFO trigger event occurs. Recommended values are 16 or 32."]
    #[must_use]
    #[inline(always)]
    pub const fn bcount(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Burst transfer size in bytes. This is the number of bytes transferred when a FIFO trigger event occurs. Recommended values are 16 or 32."]
    #[inline(always)]
    pub const fn set_bcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Dmabcount {
    #[inline(always)]
    fn default() -> Dmabcount {
        Dmabcount(0)
    }
}
impl core::fmt::Debug for Dmabcount {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmabcount")
            .field("bcount", &self.bcount())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmabcount {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dmabcount {{ bcount: {=u8:?} }}", self.bcount())
    }
}
#[doc = "DMA Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmacfg(pub u32);
impl Dmacfg {
    #[doc = "DMA Enable. Setting this bit to EN will start the DMA operation."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen(&self) -> super::vals::Dmaen {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Dmaen::from_bits(val as u8)
    }
    #[doc = "DMA Enable. Setting this bit to EN will start the DMA operation."]
    #[inline(always)]
    pub const fn set_dmaen(&mut self, val: super::vals::Dmaen) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn dmadir(&self) -> super::super::shared::Dmadir {
        let val = (self.0 >> 2usize) & 0x01;
        super::super::shared::Dmadir::from_bits(val as u8)
    }
    #[doc = "Direction."]
    #[inline(always)]
    pub const fn set_dmadir(&mut self, val: super::super::shared::Dmadir) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Sets the Priority of the DMA request."]
    #[must_use]
    #[inline(always)]
    pub const fn dmapri(&self) -> super::vals::Dmapri {
        let val = (self.0 >> 3usize) & 0x03;
        super::vals::Dmapri::from_bits(val as u8)
    }
    #[doc = "Sets the Priority of the DMA request."]
    #[inline(always)]
    pub const fn set_dmapri(&mut self, val: super::vals::Dmapri) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
    }
    #[doc = "Power off MSPI domain upon completion of DMA operation."]
    #[must_use]
    #[inline(always)]
    pub const fn dmapwroff(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Power off MSPI domain upon completion of DMA operation."]
    #[inline(always)]
    pub const fn set_dmapwroff(&mut self, val: bool) {
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
            .field("dmapwroff", &self.dmapwroff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmacfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmacfg {{ dmaen: {:?}, dmadir: {:?}, dmapri: {:?}, dmapwroff: {=bool:?} }}",
            self.dmaen(),
            self.dmadir(),
            self.dmapri(),
            self.dmapwroff()
        )
    }
}
#[doc = "DMA Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmastat(pub u32);
impl Dmastat {
    #[doc = "DMA Transfer In Progress indicator. 1 will indicate that a DMA transfer is active. The DMA transfer may be waiting on data, transferring data, or waiting for priority. All of these will be indicated with a 1. A 0 will indicate that the DMA is fully complete and no further transactions will be done."]
    #[must_use]
    #[inline(always)]
    pub const fn dmatip(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Transfer In Progress indicator. 1 will indicate that a DMA transfer is active. The DMA transfer may be waiting on data, transferring data, or waiting for priority. All of these will be indicated with a 1. A 0 will indicate that the DMA is fully complete and no further transactions will be done."]
    #[inline(always)]
    pub const fn set_dmatip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA Transfer Complete. This signals the end of the DMA operation."]
    #[must_use]
    #[inline(always)]
    pub const fn dmacpl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Transfer Complete. This signals the end of the DMA operation."]
    #[inline(always)]
    pub const fn set_dmacpl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA Error. This active high bit signals that an error was encountered during the DMA operation."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaerr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error. This active high bit signals that an error was encountered during the DMA operation."]
    #[inline(always)]
    pub const fn set_dmaerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Scrambling Access Alignment Error. This active high bit signals that a scrambling operation was specified for a non-word aligned DEVADDR."]
    #[must_use]
    #[inline(always)]
    pub const fn screrr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Scrambling Access Alignment Error. This active high bit signals that a scrambling operation was specified for a non-word aligned DEVADDR."]
    #[inline(always)]
    pub const fn set_screrr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
            .field("screrr", &self.screrr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmastat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dmastat {{ dmatip: {=bool:?}, dmacpl: {=bool:?}, dmaerr: {=bool:?}, screrr: {=bool:?} }}" , self . dmatip () , self . dmacpl () , self . dmaerr () , self . screrr ())
    }
}
#[doc = "DMA Transmit Trigger Threshhold."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmathresh(pub u32);
impl Dmathresh {
    #[doc = "DMA transfer FIFO level trigger. For read operations, DMA is triggered when the FIFO level is greater than this value. For write operations, DMA is triggered when the FIFO level is less than this level. Each DMA operation will consist of BCOUNT bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn dmathresh(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "DMA transfer FIFO level trigger. For read operations, DMA is triggered when the FIFO level is greater than this value. For write operations, DMA is triggered when the FIFO level is less than this level. Each DMA operation will consist of BCOUNT bytes."]
    #[inline(always)]
    pub const fn set_dmathresh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Dmathresh {
    #[inline(always)]
    fn default() -> Dmathresh {
        Dmathresh(0)
    }
}
impl core::fmt::Debug for Dmathresh {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmathresh")
            .field("dmathresh", &self.dmathresh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmathresh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dmathresh {{ dmathresh: {=u8:?} }}", self.dmathresh())
    }
}
#[doc = "DMA Total Transfer Count."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmatotcount(pub u32);
impl Dmatotcount {
    #[doc = "Total Transfer Count in bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn totcount(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Total Transfer Count in bytes."]
    #[inline(always)]
    pub const fn set_totcount(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
#[doc = "Configuration for XIP/DMA support of SPI flash modules."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash(pub u32);
impl Flash {
    #[doc = "Enable the XIP (eXecute In Place) function which effectively enables the address decoding of the MSPI device in the flash/cache address space at address 0x04000000-0x07FFFFFF."]
    #[must_use]
    #[inline(always)]
    pub const fn xipen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the XIP (eXecute In Place) function which effectively enables the address decoding of the MSPI device in the flash/cache address space at address 0x04000000-0x07FFFFFF."]
    #[inline(always)]
    pub const fn set_xipen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Controls transmission of Micron XIP acknowledge cycles (Micron Flash devices only)."]
    #[must_use]
    #[inline(always)]
    pub const fn xipack(&self) -> super::vals::Xipack {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Xipack::from_bits(val as u8)
    }
    #[doc = "Controls transmission of Micron XIP acknowledge cycles (Micron Flash devices only)."]
    #[inline(always)]
    pub const fn set_xipack(&mut self, val: super::vals::Xipack) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Indicates whether XIP/AUTO DMA data transfers are in big or little endian format."]
    #[must_use]
    #[inline(always)]
    pub const fn xipbigendian(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether XIP/AUTO DMA data transfers are in big or little endian format."]
    #[inline(always)]
    pub const fn set_xipbigendian(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Indicates whether XIP/AUTO DMA operations should enable TX->RX turnaround cycles."]
    #[must_use]
    #[inline(always)]
    pub const fn xipenturn(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether XIP/AUTO DMA operations should enable TX->RX turnaround cycles."]
    #[inline(always)]
    pub const fn set_xipenturn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Indicates whether XIP/AUTO DMA operations should send an an address phase (see DMADEVADDR register and ASIZE field in CFG)."]
    #[must_use]
    #[inline(always)]
    pub const fn xipsenda(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether XIP/AUTO DMA operations should send an an address phase (see DMADEVADDR register and ASIZE field in CFG)."]
    #[inline(always)]
    pub const fn set_xipsenda(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates whether XIP/AUTO DMA operations should send an instruction (see READINSTR field and ISIZE field in CFG)."]
    #[must_use]
    #[inline(always)]
    pub const fn xipsendi(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates whether XIP/AUTO DMA operations should send an instruction (see READINSTR field and ISIZE field in CFG)."]
    #[inline(always)]
    pub const fn set_xipsendi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Reserved. Set to 0x0."]
    #[must_use]
    #[inline(always)]
    pub const fn xipmixed(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Reserved. Set to 0x0."]
    #[inline(always)]
    pub const fn set_xipmixed(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Write command sent for DMA operations."]
    #[must_use]
    #[inline(always)]
    pub const fn writeinstr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Write command sent for DMA operations."]
    #[inline(always)]
    pub const fn set_writeinstr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Read command sent to flash for DMA/XIP operations."]
    #[must_use]
    #[inline(always)]
    pub const fn readinstr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Read command sent to flash for DMA/XIP operations."]
    #[inline(always)]
    pub const fn set_readinstr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Flash {
    #[inline(always)]
    fn default() -> Flash {
        Flash(0)
    }
}
impl core::fmt::Debug for Flash {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flash")
            .field("xipen", &self.xipen())
            .field("xipack", &self.xipack())
            .field("xipbigendian", &self.xipbigendian())
            .field("xipenturn", &self.xipenturn())
            .field("xipsenda", &self.xipsenda())
            .field("xipsendi", &self.xipsendi())
            .field("xipmixed", &self.xipmixed())
            .field("writeinstr", &self.writeinstr())
            .field("readinstr", &self.readinstr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flash {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Flash {{ xipen: {=bool:?}, xipack: {:?}, xipbigendian: {=bool:?}, xipenturn: {=bool:?}, xipsenda: {=bool:?}, xipsendi: {=bool:?}, xipmixed: {=u8:?}, writeinstr: {=u8:?}, readinstr: {=u8:?} }}" , self . xipen () , self . xipack () , self . xipbigendian () , self . xipenturn () , self . xipsenda () , self . xipsendi () , self . xipmixed () , self . writeinstr () , self . readinstr ())
    }
}
#[doc = "MSPI Transfer Instruction."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Instr(pub u32);
impl Instr {
    #[doc = "Optional Instruction field to send (1st byte) - qualified by ISEND/ISIZE."]
    #[must_use]
    #[inline(always)]
    pub const fn instr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Optional Instruction field to send (1st byte) - qualified by ISEND/ISIZE."]
    #[inline(always)]
    pub const fn set_instr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Instr {
    #[inline(always)]
    fn default() -> Instr {
        Instr(0)
    }
}
impl core::fmt::Debug for Instr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Instr")
            .field("instr", &self.instr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Instr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Instr {{ instr: {=u16:?} }}", self.instr())
    }
}
#[doc = "MSPI Master Interrupts: Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intclr(pub u32);
impl Intclr {
    #[doc = "Transfer complete. Note that DMA and CQ operations are layered, so CMDCMP, DCMP, and CQ* can all be signalled simultaneously."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdcmp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete. Note that DMA and CQ operations are layered, so CMDCMP, DCMP, and CQ* can all be signalled simultaneously."]
    #[inline(always)]
    pub const fn set_cmdcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit FIFO empty."]
    #[must_use]
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO empty."]
    #[inline(always)]
    pub const fn set_txe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit FIFO Overflow (only occurs when SW writes to a full FIFO)."]
    #[must_use]
    #[inline(always)]
    pub const fn txo(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Overflow (only occurs when SW writes to a full FIFO)."]
    #[inline(always)]
    pub const fn set_txo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive FIFO underflow (only occurs when SW reads from an empty FIFO)."]
    #[must_use]
    #[inline(always)]
    pub const fn rxu(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO underflow (only occurs when SW reads from an empty FIFO)."]
    #[inline(always)]
    pub const fn set_rxu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive FIFO overflow (cannot happen in MSPI design -- MSPI bus pins will stall)."]
    #[must_use]
    #[inline(always)]
    pub const fn rxo(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO overflow (cannot happen in MSPI design -- MSPI bus pins will stall)."]
    #[inline(always)]
    pub const fn set_rxo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Receive FIFO full."]
    #[must_use]
    #[inline(always)]
    pub const fn rxf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO full."]
    #[inline(always)]
    pub const fn set_rxf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA Complete Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Complete Interrupt."]
    #[inline(always)]
    pub const fn set_dcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA Error Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn derr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error Interrupt."]
    #[inline(always)]
    pub const fn set_derr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Command Queue Complete Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn cqcmp(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Command Queue Complete Interrupt."]
    #[inline(always)]
    pub const fn set_cqcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Command Queue Update Interrupt. Issued whenever the CQ performs an operation where address bit\\[0\\] is set. Useful for triggering CURIDX interrupts."]
    #[must_use]
    #[inline(always)]
    pub const fn cqupd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Command Queue Update Interrupt. Issued whenever the CQ performs an operation where address bit\\[0\\] is set. Useful for triggering CURIDX interrupts."]
    #[inline(always)]
    pub const fn set_cqupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Command Queue is Paused."]
    #[must_use]
    #[inline(always)]
    pub const fn cqpaused(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Command Queue is Paused."]
    #[inline(always)]
    pub const fn set_cqpaused(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Command Queue Error Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn cqerr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Command Queue Error Interrupt."]
    #[inline(always)]
    pub const fn set_cqerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Scrambling Alignment Error. Scrambling operations must be aligned to word (4-byte) start address."]
    #[must_use]
    #[inline(always)]
    pub const fn screrr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Scrambling Alignment Error. Scrambling operations must be aligned to word (4-byte) start address."]
    #[inline(always)]
    pub const fn set_screrr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
            .field("cmdcmp", &self.cmdcmp())
            .field("txe", &self.txe())
            .field("txo", &self.txo())
            .field("rxu", &self.rxu())
            .field("rxo", &self.rxo())
            .field("rxf", &self.rxf())
            .field("dcmp", &self.dcmp())
            .field("derr", &self.derr())
            .field("cqcmp", &self.cqcmp())
            .field("cqupd", &self.cqupd())
            .field("cqpaused", &self.cqpaused())
            .field("cqerr", &self.cqerr())
            .field("screrr", &self.screrr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intclr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Intclr {{ cmdcmp: {=bool:?}, txe: {=bool:?}, txo: {=bool:?}, rxu: {=bool:?}, rxo: {=bool:?}, rxf: {=bool:?}, dcmp: {=bool:?}, derr: {=bool:?}, cqcmp: {=bool:?}, cqupd: {=bool:?}, cqpaused: {=bool:?}, cqerr: {=bool:?}, screrr: {=bool:?} }}" , self . cmdcmp () , self . txe () , self . txo () , self . rxu () , self . rxo () , self . rxf () , self . dcmp () , self . derr () , self . cqcmp () , self . cqupd () , self . cqpaused () , self . cqerr () , self . screrr ())
    }
}
#[doc = "MSPI Master Interrupts: Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Transfer complete. Note that DMA and CQ operations are layered, so CMDCMP, DCMP, and CQ* can all be signalled simultaneously."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdcmp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete. Note that DMA and CQ operations are layered, so CMDCMP, DCMP, and CQ* can all be signalled simultaneously."]
    #[inline(always)]
    pub const fn set_cmdcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit FIFO empty."]
    #[must_use]
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO empty."]
    #[inline(always)]
    pub const fn set_txe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit FIFO Overflow (only occurs when SW writes to a full FIFO)."]
    #[must_use]
    #[inline(always)]
    pub const fn txo(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Overflow (only occurs when SW writes to a full FIFO)."]
    #[inline(always)]
    pub const fn set_txo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive FIFO underflow (only occurs when SW reads from an empty FIFO)."]
    #[must_use]
    #[inline(always)]
    pub const fn rxu(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO underflow (only occurs when SW reads from an empty FIFO)."]
    #[inline(always)]
    pub const fn set_rxu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive FIFO overflow (cannot happen in MSPI design -- MSPI bus pins will stall)."]
    #[must_use]
    #[inline(always)]
    pub const fn rxo(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO overflow (cannot happen in MSPI design -- MSPI bus pins will stall)."]
    #[inline(always)]
    pub const fn set_rxo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Receive FIFO full."]
    #[must_use]
    #[inline(always)]
    pub const fn rxf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO full."]
    #[inline(always)]
    pub const fn set_rxf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA Complete Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Complete Interrupt."]
    #[inline(always)]
    pub const fn set_dcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA Error Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn derr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error Interrupt."]
    #[inline(always)]
    pub const fn set_derr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Command Queue Complete Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn cqcmp(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Command Queue Complete Interrupt."]
    #[inline(always)]
    pub const fn set_cqcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Command Queue Update Interrupt. Issued whenever the CQ performs an operation where address bit\\[0\\] is set. Useful for triggering CURIDX interrupts."]
    #[must_use]
    #[inline(always)]
    pub const fn cqupd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Command Queue Update Interrupt. Issued whenever the CQ performs an operation where address bit\\[0\\] is set. Useful for triggering CURIDX interrupts."]
    #[inline(always)]
    pub const fn set_cqupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Command Queue is Paused."]
    #[must_use]
    #[inline(always)]
    pub const fn cqpaused(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Command Queue is Paused."]
    #[inline(always)]
    pub const fn set_cqpaused(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Command Queue Error Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn cqerr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Command Queue Error Interrupt."]
    #[inline(always)]
    pub const fn set_cqerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Scrambling Alignment Error. Scrambling operations must be aligned to word (4-byte) start address."]
    #[must_use]
    #[inline(always)]
    pub const fn screrr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Scrambling Alignment Error. Scrambling operations must be aligned to word (4-byte) start address."]
    #[inline(always)]
    pub const fn set_screrr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
            .field("cmdcmp", &self.cmdcmp())
            .field("txe", &self.txe())
            .field("txo", &self.txo())
            .field("rxu", &self.rxu())
            .field("rxo", &self.rxo())
            .field("rxf", &self.rxf())
            .field("dcmp", &self.dcmp())
            .field("derr", &self.derr())
            .field("cqcmp", &self.cqcmp())
            .field("cqupd", &self.cqupd())
            .field("cqpaused", &self.cqpaused())
            .field("cqerr", &self.cqerr())
            .field("screrr", &self.screrr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Inten {{ cmdcmp: {=bool:?}, txe: {=bool:?}, txo: {=bool:?}, rxu: {=bool:?}, rxo: {=bool:?}, rxf: {=bool:?}, dcmp: {=bool:?}, derr: {=bool:?}, cqcmp: {=bool:?}, cqupd: {=bool:?}, cqpaused: {=bool:?}, cqerr: {=bool:?}, screrr: {=bool:?} }}" , self . cmdcmp () , self . txe () , self . txo () , self . rxu () , self . rxo () , self . rxf () , self . dcmp () , self . derr () , self . cqcmp () , self . cqupd () , self . cqpaused () , self . cqerr () , self . screrr ())
    }
}
#[doc = "MSPI Master Interrupts: Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intset(pub u32);
impl Intset {
    #[doc = "Transfer complete. Note that DMA and CQ operations are layered, so CMDCMP, DCMP, and CQ* can all be signalled simultaneously."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdcmp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete. Note that DMA and CQ operations are layered, so CMDCMP, DCMP, and CQ* can all be signalled simultaneously."]
    #[inline(always)]
    pub const fn set_cmdcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit FIFO empty."]
    #[must_use]
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO empty."]
    #[inline(always)]
    pub const fn set_txe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit FIFO Overflow (only occurs when SW writes to a full FIFO)."]
    #[must_use]
    #[inline(always)]
    pub const fn txo(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Overflow (only occurs when SW writes to a full FIFO)."]
    #[inline(always)]
    pub const fn set_txo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive FIFO underflow (only occurs when SW reads from an empty FIFO)."]
    #[must_use]
    #[inline(always)]
    pub const fn rxu(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO underflow (only occurs when SW reads from an empty FIFO)."]
    #[inline(always)]
    pub const fn set_rxu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive FIFO overflow (cannot happen in MSPI design -- MSPI bus pins will stall)."]
    #[must_use]
    #[inline(always)]
    pub const fn rxo(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO overflow (cannot happen in MSPI design -- MSPI bus pins will stall)."]
    #[inline(always)]
    pub const fn set_rxo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Receive FIFO full."]
    #[must_use]
    #[inline(always)]
    pub const fn rxf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO full."]
    #[inline(always)]
    pub const fn set_rxf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA Complete Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Complete Interrupt."]
    #[inline(always)]
    pub const fn set_dcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA Error Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn derr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error Interrupt."]
    #[inline(always)]
    pub const fn set_derr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Command Queue Complete Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn cqcmp(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Command Queue Complete Interrupt."]
    #[inline(always)]
    pub const fn set_cqcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Command Queue Update Interrupt. Issued whenever the CQ performs an operation where address bit\\[0\\] is set. Useful for triggering CURIDX interrupts."]
    #[must_use]
    #[inline(always)]
    pub const fn cqupd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Command Queue Update Interrupt. Issued whenever the CQ performs an operation where address bit\\[0\\] is set. Useful for triggering CURIDX interrupts."]
    #[inline(always)]
    pub const fn set_cqupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Command Queue is Paused."]
    #[must_use]
    #[inline(always)]
    pub const fn cqpaused(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Command Queue is Paused."]
    #[inline(always)]
    pub const fn set_cqpaused(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Command Queue Error Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn cqerr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Command Queue Error Interrupt."]
    #[inline(always)]
    pub const fn set_cqerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Scrambling Alignment Error. Scrambling operations must be aligned to word (4-byte) start address."]
    #[must_use]
    #[inline(always)]
    pub const fn screrr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Scrambling Alignment Error. Scrambling operations must be aligned to word (4-byte) start address."]
    #[inline(always)]
    pub const fn set_screrr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
            .field("cmdcmp", &self.cmdcmp())
            .field("txe", &self.txe())
            .field("txo", &self.txo())
            .field("rxu", &self.rxu())
            .field("rxo", &self.rxo())
            .field("rxf", &self.rxf())
            .field("dcmp", &self.dcmp())
            .field("derr", &self.derr())
            .field("cqcmp", &self.cqcmp())
            .field("cqupd", &self.cqupd())
            .field("cqpaused", &self.cqpaused())
            .field("cqerr", &self.cqerr())
            .field("screrr", &self.screrr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intset {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Intset {{ cmdcmp: {=bool:?}, txe: {=bool:?}, txo: {=bool:?}, rxu: {=bool:?}, rxo: {=bool:?}, rxf: {=bool:?}, dcmp: {=bool:?}, derr: {=bool:?}, cqcmp: {=bool:?}, cqupd: {=bool:?}, cqpaused: {=bool:?}, cqerr: {=bool:?}, screrr: {=bool:?} }}" , self . cmdcmp () , self . txe () , self . txo () , self . rxu () , self . rxo () , self . rxf () , self . dcmp () , self . derr () , self . cqcmp () , self . cqupd () , self . cqpaused () , self . cqerr () , self . screrr ())
    }
}
#[doc = "MSPI Master Interrupts: Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "Transfer complete. Note that DMA and CQ operations are layered, so CMDCMP, DCMP, and CQ* can all be signalled simultaneously."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdcmp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete. Note that DMA and CQ operations are layered, so CMDCMP, DCMP, and CQ* can all be signalled simultaneously."]
    #[inline(always)]
    pub const fn set_cmdcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit FIFO empty."]
    #[must_use]
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO empty."]
    #[inline(always)]
    pub const fn set_txe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit FIFO Overflow (only occurs when SW writes to a full FIFO)."]
    #[must_use]
    #[inline(always)]
    pub const fn txo(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO Overflow (only occurs when SW writes to a full FIFO)."]
    #[inline(always)]
    pub const fn set_txo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive FIFO underflow (only occurs when SW reads from an empty FIFO)."]
    #[must_use]
    #[inline(always)]
    pub const fn rxu(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO underflow (only occurs when SW reads from an empty FIFO)."]
    #[inline(always)]
    pub const fn set_rxu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive FIFO overflow (cannot happen in MSPI design -- MSPI bus pins will stall)."]
    #[must_use]
    #[inline(always)]
    pub const fn rxo(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO overflow (cannot happen in MSPI design -- MSPI bus pins will stall)."]
    #[inline(always)]
    pub const fn set_rxo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Receive FIFO full."]
    #[must_use]
    #[inline(always)]
    pub const fn rxf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO full."]
    #[inline(always)]
    pub const fn set_rxf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DMA Complete Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Complete Interrupt."]
    #[inline(always)]
    pub const fn set_dcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "DMA Error Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn derr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error Interrupt."]
    #[inline(always)]
    pub const fn set_derr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Command Queue Complete Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn cqcmp(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Command Queue Complete Interrupt."]
    #[inline(always)]
    pub const fn set_cqcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Command Queue Update Interrupt. Issued whenever the CQ performs an operation where address bit\\[0\\] is set. Useful for triggering CURIDX interrupts."]
    #[must_use]
    #[inline(always)]
    pub const fn cqupd(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Command Queue Update Interrupt. Issued whenever the CQ performs an operation where address bit\\[0\\] is set. Useful for triggering CURIDX interrupts."]
    #[inline(always)]
    pub const fn set_cqupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Command Queue is Paused."]
    #[must_use]
    #[inline(always)]
    pub const fn cqpaused(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Command Queue is Paused."]
    #[inline(always)]
    pub const fn set_cqpaused(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Command Queue Error Interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn cqerr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Command Queue Error Interrupt."]
    #[inline(always)]
    pub const fn set_cqerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Scrambling Alignment Error. Scrambling operations must be aligned to word (4-byte) start address."]
    #[must_use]
    #[inline(always)]
    pub const fn screrr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Scrambling Alignment Error. Scrambling operations must be aligned to word (4-byte) start address."]
    #[inline(always)]
    pub const fn set_screrr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
            .field("cmdcmp", &self.cmdcmp())
            .field("txe", &self.txe())
            .field("txo", &self.txo())
            .field("rxu", &self.rxu())
            .field("rxo", &self.rxo())
            .field("rxf", &self.rxf())
            .field("dcmp", &self.dcmp())
            .field("derr", &self.derr())
            .field("cqcmp", &self.cqcmp())
            .field("cqupd", &self.cqupd())
            .field("cqpaused", &self.cqpaused())
            .field("cqerr", &self.cqerr())
            .field("screrr", &self.screrr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Intstat {{ cmdcmp: {=bool:?}, txe: {=bool:?}, txo: {=bool:?}, rxu: {=bool:?}, rxo: {=bool:?}, rxf: {=bool:?}, dcmp: {=bool:?}, derr: {=bool:?}, cqcmp: {=bool:?}, cqupd: {=bool:?}, cqpaused: {=bool:?}, cqerr: {=bool:?}, screrr: {=bool:?} }}" , self . cmdcmp () , self . txe () , self . txo () , self . rxu () , self . rxo () , self . rxf () , self . dcmp () , self . derr () , self . cqcmp () , self . cqupd () , self . cqpaused () , self . cqerr () , self . screrr ())
    }
}
#[doc = "MSPI Module Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mspicfg(pub u32);
impl Mspicfg {
    #[doc = "Enable continuous APB clock. For power-efficient operation, APBCLK should be set to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn apbclk(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable continuous APB clock. For power-efficient operation, APBCLK should be set to 0."]
    #[inline(always)]
    pub const fn set_apbclk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Controls RX data capture phase. A setting of 0 (NORMAL) captures read data at the normal capture point relative to the internal clock launch point. However, to accomodate chip/pad/board delays, a setting of RXCAP of 1 is expected to be used to align the capture point with the return data window. This bit is used in conjunction with RXNEG to provide 4 unique capture points, all about 10ns apart."]
    #[must_use]
    #[inline(always)]
    pub const fn rxcap(&self) -> super::vals::Rxcap {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Rxcap::from_bits(val as u8)
    }
    #[doc = "Controls RX data capture phase. A setting of 0 (NORMAL) captures read data at the normal capture point relative to the internal clock launch point. However, to accomodate chip/pad/board delays, a setting of RXCAP of 1 is expected to be used to align the capture point with the return data window. This bit is used in conjunction with RXNEG to provide 4 unique capture points, all about 10ns apart."]
    #[inline(always)]
    pub const fn set_rxcap(&mut self, val: super::vals::Rxcap) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Adjusts the RX capture phase to the negedge of the 48MHz internal clock (~10ns early). For normal operation, it is expected that RXNEG will be set to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn rxneg(&self) -> super::vals::Negedge {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Negedge::from_bits(val as u8)
    }
    #[doc = "Adjusts the RX capture phase to the negedge of the 48MHz internal clock (~10ns early). For normal operation, it is expected that RXNEG will be set to 0."]
    #[inline(always)]
    pub const fn set_rxneg(&mut self, val: super::vals::Negedge) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Launches TX data a half clock cycle (~10ns) early. This should normally be programmed to zero (NORMAL)."]
    #[must_use]
    #[inline(always)]
    pub const fn txneg(&self) -> super::vals::Negedge {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Negedge::from_bits(val as u8)
    }
    #[doc = "Launches TX data a half clock cycle (~10ns) early. This should normally be programmed to zero (NORMAL)."]
    #[inline(always)]
    pub const fn set_txneg(&mut self, val: super::vals::Negedge) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Selects which IOM is selected for CQ handshake status."]
    #[must_use]
    #[inline(always)]
    pub const fn iomsel(&self) -> super::vals::Iomsel {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Iomsel::from_bits(val as u8)
    }
    #[doc = "Selects which IOM is selected for CQ handshake status."]
    #[inline(always)]
    pub const fn set_iomsel(&mut self, val: super::vals::Iomsel) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
    #[doc = "Clock Divider. Allows dividing 48 MHz base clock by integer multiples. Enumerations are provided for common frequency, but any integer divide from 48 MHz is allowed. Odd divide ratios will result in a 33/66 percent duty cycle with a long low clock pulse (to allow longer round-trip for read data)."]
    #[must_use]
    #[inline(always)]
    pub const fn clkdiv(&self) -> super::vals::Clkdiv {
        let val = (self.0 >> 8usize) & 0x3f;
        super::vals::Clkdiv::from_bits(val as u8)
    }
    #[doc = "Clock Divider. Allows dividing 48 MHz base clock by integer multiples. Enumerations are provided for common frequency, but any integer divide from 48 MHz is allowed. Odd divide ratios will result in a 33/66 percent duty cycle with a long low clock pulse (to allow longer round-trip for read data)."]
    #[inline(always)]
    pub const fn set_clkdiv(&mut self, val: super::vals::Clkdiv) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
    }
    #[doc = "Reset MSPI FIFO (active high). 1=reset FIFO, 0=normal operation. May be used to manually flush the FIFO in error handling."]
    #[must_use]
    #[inline(always)]
    pub const fn fiforeset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Reset MSPI FIFO (active high). 1=reset FIFO, 0=normal operation. May be used to manually flush the FIFO in error handling."]
    #[inline(always)]
    pub const fn set_fiforeset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "IP block reset. Write to 0 to put the transfer module in reset or 1 for normal operation. This may be required after error conditions to clear the transfer on the bus."]
    #[must_use]
    #[inline(always)]
    pub const fn iprstn(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "IP block reset. Write to 0 to put the transfer module in reset or 1 for normal operation. This may be required after error conditions to clear the transfer on the bus."]
    #[inline(always)]
    pub const fn set_iprstn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Peripheral reset. Master reset to the entire MSPI module (DMA, XIP, and transfer state machines). 1=normal operation, 0=in reset."]
    #[must_use]
    #[inline(always)]
    pub const fn prstn(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral reset. Master reset to the entire MSPI module (DMA, XIP, and transfer state machines). 1=normal operation, 0=in reset."]
    #[inline(always)]
    pub const fn set_prstn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mspicfg {
    #[inline(always)]
    fn default() -> Mspicfg {
        Mspicfg(0)
    }
}
impl core::fmt::Debug for Mspicfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mspicfg")
            .field("apbclk", &self.apbclk())
            .field("rxcap", &self.rxcap())
            .field("rxneg", &self.rxneg())
            .field("txneg", &self.txneg())
            .field("iomsel", &self.iomsel())
            .field("clkdiv", &self.clkdiv())
            .field("fiforeset", &self.fiforeset())
            .field("iprstn", &self.iprstn())
            .field("prstn", &self.prstn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mspicfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Mspicfg {{ apbclk: {=bool:?}, rxcap: {:?}, rxneg: {:?}, txneg: {:?}, iomsel: {:?}, clkdiv: {:?}, fiforeset: {=bool:?}, iprstn: {=bool:?}, prstn: {=bool:?} }}" , self . apbclk () , self . rxcap () , self . rxneg () , self . txneg () , self . iomsel () , self . clkdiv () , self . fiforeset () , self . iprstn () , self . prstn ())
    }
}
#[doc = "MSPI Output Pad Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Padcfg(pub u32);
impl Padcfg {
    #[doc = "Output pad 3 configuration. 0=data\\[3\\] 1=CLK."]
    #[must_use]
    #[inline(always)]
    pub const fn out3(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Output pad 3 configuration. 0=data\\[3\\] 1=CLK."]
    #[inline(always)]
    pub const fn set_out3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Output pad 4 configuration. 0=data\\[4\\] 1=data\\[0\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn out4(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Output pad 4 configuration. 0=data\\[4\\] 1=data\\[0\\]."]
    #[inline(always)]
    pub const fn set_out4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Output pad 5 configuration. 0=data\\[5\\] 1=data\\[1\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn out5(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Output pad 5 configuration. 0=data\\[5\\] 1=data\\[1\\]."]
    #[inline(always)]
    pub const fn set_out5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Output pad 6 configuration. 0=data\\[6\\] 1=data\\[2\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn out6(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Output pad 6 configuration. 0=data\\[6\\] 1=data\\[2\\]."]
    #[inline(always)]
    pub const fn set_out6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Output pad 7 configuration. 0=data\\[7\\] 1=data\\[3\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn out7(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Output pad 7 configuration. 0=data\\[7\\] 1=data\\[3\\]."]
    #[inline(always)]
    pub const fn set_out7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Data Input pad 0 pin muxing: 0=pad\\[0\\] 1=pad\\[4\\] 2=pad\\[1\\] 3=pad\\[5\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn in0(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Data Input pad 0 pin muxing: 0=pad\\[0\\] 1=pad\\[4\\] 2=pad\\[1\\] 3=pad\\[5\\]."]
    #[inline(always)]
    pub const fn set_in0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "Data Input pad 1 pin muxing: 0=pad\\[1\\] 1=pad\\[5\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn in1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Data Input pad 1 pin muxing: 0=pad\\[1\\] 1=pad\\[5\\]."]
    #[inline(always)]
    pub const fn set_in1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Data Input pad 2 pin muxing: 0=pad\\[2\\] 1=pad\\[6\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn in2(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Data Input pad 2 pin muxing: 0=pad\\[2\\] 1=pad\\[6\\]."]
    #[inline(always)]
    pub const fn set_in2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Data Input pad 3 pin muxing: 0=pad\\[3\\] 1=pad\\[7\\]."]
    #[must_use]
    #[inline(always)]
    pub const fn in3(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Data Input pad 3 pin muxing: 0=pad\\[3\\] 1=pad\\[7\\]."]
    #[inline(always)]
    pub const fn set_in3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Reverse CS connections. Allows CS1 to be associated with lower data lanes and CS0 to be associated with upper data lines."]
    #[must_use]
    #[inline(always)]
    pub const fn revcs(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Reverse CS connections. Allows CS1 to be associated with lower data lanes and CS0 to be associated with upper data lines."]
    #[inline(always)]
    pub const fn set_revcs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Padcfg {
    #[inline(always)]
    fn default() -> Padcfg {
        Padcfg(0)
    }
}
impl core::fmt::Debug for Padcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Padcfg")
            .field("out3", &self.out3())
            .field("out4", &self.out4())
            .field("out5", &self.out5())
            .field("out6", &self.out6())
            .field("out7", &self.out7())
            .field("in0", &self.in0())
            .field("in1", &self.in1())
            .field("in2", &self.in2())
            .field("in3", &self.in3())
            .field("revcs", &self.revcs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Padcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Padcfg {{ out3: {=bool:?}, out4: {=bool:?}, out5: {=bool:?}, out6: {=bool:?}, out7: {=bool:?}, in0: {=u8:?}, in1: {=bool:?}, in2: {=bool:?}, in3: {=bool:?}, revcs: {=bool:?} }}" , self . out3 () , self . out4 () , self . out5 () , self . out6 () , self . out7 () , self . in0 () , self . in1 () , self . in2 () , self . in3 () , self . revcs ())
    }
}
#[doc = "MSPI Output Enable Pad Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Padouten(pub u32);
impl Padouten {
    #[doc = "Output pad enable configuration. Indicates which pads should be driven. Bits \\[3:0\\] are Quad0 data, \\[7:4\\] are Quad1 data, and \\[8\\] is clock."]
    #[must_use]
    #[inline(always)]
    pub const fn outen(&self) -> super::vals::Outen {
        let val = (self.0 >> 0usize) & 0x01ff;
        super::vals::Outen::from_bits(val as u16)
    }
    #[doc = "Output pad enable configuration. Indicates which pads should be driven. Bits \\[3:0\\] are Quad0 data, \\[7:4\\] are Quad1 data, and \\[8\\] is clock."]
    #[inline(always)]
    pub const fn set_outen(&mut self, val: super::vals::Outen) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val.to_bits() as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Padouten {
    #[inline(always)]
    fn default() -> Padouten {
        Padouten(0)
    }
}
impl core::fmt::Debug for Padouten {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Padouten")
            .field("outen", &self.outen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Padouten {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Padouten {{ outen: {:?} }}", self.outen())
    }
}
#[doc = "RX FIFO Entries."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxentries(pub u32);
impl Rxentries {
    #[doc = "Number of 32-bit words/entries in RX FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn rxentries(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of 32-bit words/entries in RX FIFO."]
    #[inline(always)]
    pub const fn set_rxentries(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Rxentries {
    #[inline(always)]
    fn default() -> Rxentries {
        Rxentries(0)
    }
}
impl core::fmt::Debug for Rxentries {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxentries")
            .field("rxentries", &self.rxentries())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxentries {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rxentries {{ rxentries: {=u8:?} }}", self.rxentries())
    }
}
#[doc = "External Flash Scrambling Controls."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scrambling(pub u32);
impl Scrambling {
    #[doc = "Scrambling region start address \\[25:16\\] (64K block granularity). The START block is the FIRST block included in the scrambled address range."]
    #[must_use]
    #[inline(always)]
    pub const fn scrstart(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Scrambling region start address \\[25:16\\] (64K block granularity). The START block is the FIRST block included in the scrambled address range."]
    #[inline(always)]
    pub const fn set_scrstart(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Scrambling region end address \\[25:16\\] (64K block granularity). The END block is the LAST block included in the scrambled address range."]
    #[must_use]
    #[inline(always)]
    pub const fn scrend(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Scrambling region end address \\[25:16\\] (64K block granularity). The END block is the LAST block included in the scrambled address range."]
    #[inline(always)]
    pub const fn set_scrend(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "Enables Data Scrambling Region. When 1 reads and writes to the range will be scrambled. When 0, data will be read/written unmodified. Address range is specified in 64K granularity and the START/END ranges are included within the range."]
    #[must_use]
    #[inline(always)]
    pub const fn screnable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enables Data Scrambling Region. When 1 reads and writes to the range will be scrambled. When 0, data will be read/written unmodified. Address range is specified in 64K granularity and the START/END ranges are included within the range."]
    #[inline(always)]
    pub const fn set_screnable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Scrambling {
    #[inline(always)]
    fn default() -> Scrambling {
        Scrambling(0)
    }
}
impl core::fmt::Debug for Scrambling {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scrambling")
            .field("scrstart", &self.scrstart())
            .field("scrend", &self.scrend())
            .field("screnable", &self.screnable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Scrambling {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Scrambling {{ scrstart: {=u16:?}, scrend: {=u16:?}, screnable: {=bool:?} }}",
            self.scrstart(),
            self.scrend(),
            self.screnable()
        )
    }
}
#[doc = "TX/RX FIFO Threshhold Levels."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Threshold(pub u32);
impl Threshold {
    #[doc = "Number of entries in TX FIFO that cause TXF interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn txthresh(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of entries in TX FIFO that cause TXF interrupt."]
    #[inline(always)]
    pub const fn set_txthresh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Number of entries in TX FIFO that cause RXE interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn rxthresh(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of entries in TX FIFO that cause RXE interrupt."]
    #[inline(always)]
    pub const fn set_rxthresh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
}
impl Default for Threshold {
    #[inline(always)]
    fn default() -> Threshold {
        Threshold(0)
    }
}
impl core::fmt::Debug for Threshold {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Threshold")
            .field("txthresh", &self.txthresh())
            .field("rxthresh", &self.rxthresh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Threshold {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Threshold {{ txthresh: {=u8:?}, rxthresh: {=u8:?} }}",
            self.txthresh(),
            self.rxthresh()
        )
    }
}
#[doc = "TX FIFO Entries."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txentries(pub u32);
impl Txentries {
    #[doc = "Number of 32-bit words/entries in TX FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn txentries(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of 32-bit words/entries in TX FIFO."]
    #[inline(always)]
    pub const fn set_txentries(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Txentries {
    #[inline(always)]
    fn default() -> Txentries {
        Txentries(0)
    }
}
impl core::fmt::Debug for Txentries {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txentries")
            .field("txentries", &self.txentries())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txentries {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Txentries {{ txentries: {=u8:?} }}", self.txentries())
    }
}
