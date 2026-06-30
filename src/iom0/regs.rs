#[doc = "I/O Clock Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkcfg(pub u32);
impl Clkcfg {
    #[doc = "Enable for the interface clock. Must be enabled prior to executing any IO operations."]
    #[must_use]
    #[inline(always)]
    pub const fn ioclken(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable for the interface clock. Must be enabled prior to executing any IO operations."]
    #[inline(always)]
    pub const fn set_ioclken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Select the input clock frequency."]
    #[must_use]
    #[inline(always)]
    pub const fn fsel(&self) -> super::super::shared::Fsel {
        let val = (self.0 >> 8usize) & 0x07;
        super::super::shared::Fsel::from_bits(val as u8)
    }
    #[doc = "Select the input clock frequency."]
    #[inline(always)]
    pub const fn set_fsel(&mut self, val: super::super::shared::Fsel) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Enable divide by 3 of the source IOCLK. Division by 3 is done before the DIVEN programmable divider, and if enabled will provide the divided by 3 clock as the source to the programmable divider."]
    #[must_use]
    #[inline(always)]
    pub const fn div3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable divide by 3 of the source IOCLK. Division by 3 is done before the DIVEN programmable divider, and if enabled will provide the divided by 3 clock as the source to the programmable divider."]
    #[inline(always)]
    pub const fn set_div3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable clock division by TOTPER and LOWPER."]
    #[must_use]
    #[inline(always)]
    pub const fn diven(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enable clock division by TOTPER and LOWPER."]
    #[inline(always)]
    pub const fn set_diven(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Clock low clock count minus 1. This provides the number of clocks the divided clock will be low when the DIVEN = 1. Only applicable when DIVEN = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn lowper(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Clock low clock count minus 1. This provides the number of clocks the divided clock will be low when the DIVEN = 1. Only applicable when DIVEN = 1."]
    #[inline(always)]
    pub const fn set_lowper(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Clock total clock count minus 1. This provides the total period of the divided clock -1 when the DIVEN is active. The source clock is selected by FSEL. Only applicable when DIVEN = 1."]
    #[must_use]
    #[inline(always)]
    pub const fn totper(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Clock total clock count minus 1. This provides the total period of the divided clock -1 when the DIVEN is active. The source clock is selected by FSEL. Only applicable when DIVEN = 1."]
    #[inline(always)]
    pub const fn set_totper(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Clkcfg {
    #[inline(always)]
    fn default() -> Clkcfg {
        Clkcfg(0)
    }
}
impl core::fmt::Debug for Clkcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkcfg")
            .field("ioclken", &self.ioclken())
            .field("fsel", &self.fsel())
            .field("div3", &self.div3())
            .field("diven", &self.diven())
            .field("lowper", &self.lowper())
            .field("totper", &self.totper())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Clkcfg {{ ioclken: {=bool:?}, fsel: {:?}, div3: {=bool:?}, diven: {=bool:?}, lowper: {=u8:?}, totper: {=u8:?} }}" , self . ioclken () , self . fsel () , self . div3 () , self . diven () , self . lowper () , self . totper ())
    }
}
#[doc = "Command and offset Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd(pub u32);
impl Cmd {
    #[doc = "Command for submodule."]
    #[must_use]
    #[inline(always)]
    pub const fn cmd(&self) -> super::vals::Cmd {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Cmd::from_bits(val as u8)
    }
    #[doc = "Command for submodule."]
    #[inline(always)]
    pub const fn set_cmd(&mut self, val: super::vals::Cmd) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Number of offset bytes to use for the command - 0, 1, 2, 3 are valid selections. The second (byte 1) and third byte (byte 2) are read from the OFFSETHI register, and the low order byte is pulled from this register in the OFFSETLO field. Offset bytes are transmitted highest byte first. EG if offsetcnt == 3, OFFSETHI\\[15:8\\] will be transmitted first, then OFFSETHI\\[7:0\\] then OFFSETLO. If offsetcnt == 2, OFFSETHI\\[7:0\\] will be transmitted, then OFFSETLO. If offsetcnt == 1, only OFFSETLO will be transmitted. Offset bytes are always transmitted MSB first, regardless of the value of the LSB control bit within the module configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn offsetcnt(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "Number of offset bytes to use for the command - 0, 1, 2, 3 are valid selections. The second (byte 1) and third byte (byte 2) are read from the OFFSETHI register, and the low order byte is pulled from this register in the OFFSETLO field. Offset bytes are transmitted highest byte first. EG if offsetcnt == 3, OFFSETHI\\[15:8\\] will be transmitted first, then OFFSETHI\\[7:0\\] then OFFSETLO. If offsetcnt == 2, OFFSETHI\\[7:0\\] will be transmitted, then OFFSETLO. If offsetcnt == 1, only OFFSETLO will be transmitted. Offset bytes are always transmitted MSB first, regardless of the value of the LSB control bit within the module configuration."]
    #[inline(always)]
    pub const fn set_offsetcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "Contine to hold the bus after the current transaction if set to a 1 with a new command issued."]
    #[must_use]
    #[inline(always)]
    pub const fn cont(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Contine to hold the bus after the current transaction if set to a 1 with a new command issued."]
    #[inline(always)]
    pub const fn set_cont(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Defines the transaction size in bytes. The offset transfer is not included in this size."]
    #[must_use]
    #[inline(always)]
    pub const fn tsize(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "Defines the transaction size in bytes. The offset transfer is not included in this size."]
    #[inline(always)]
    pub const fn set_tsize(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
    #[doc = "Command Specific selection information. Not used in Master I2C. Used as CEn select for Master SPI transactions."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdsel(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Command Specific selection information. Not used in Master I2C. Used as CEn select for Master SPI transactions."]
    #[inline(always)]
    pub const fn set_cmdsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "This register holds the low order byte of offset to be used in the transaction. The number of offset bytes to use is set with bits 1:0 of the command."]
    #[must_use]
    #[inline(always)]
    pub const fn offsetlo(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "This register holds the low order byte of offset to be used in the transaction. The number of offset bytes to use is set with bits 1:0 of the command."]
    #[inline(always)]
    pub const fn set_offsetlo(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Cmd {
    #[inline(always)]
    fn default() -> Cmd {
        Cmd(0)
    }
}
impl core::fmt::Debug for Cmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmd")
            .field("cmd", &self.cmd())
            .field("offsetcnt", &self.offsetcnt())
            .field("cont", &self.cont())
            .field("tsize", &self.tsize())
            .field("cmdsel", &self.cmdsel())
            .field("offsetlo", &self.offsetlo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmd {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cmd {{ cmd: {:?}, offsetcnt: {=u8:?}, cont: {=bool:?}, tsize: {=u16:?}, cmdsel: {=u8:?}, offsetlo: {=u8:?} }}" , self . cmd () , self . offsetcnt () , self . cont () , self . tsize () , self . cmdsel () , self . offsetlo ())
    }
}
#[doc = "Command status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdstat(pub u32);
impl Cmdstat {
    #[doc = "current command that is being executed."]
    #[must_use]
    #[inline(always)]
    pub const fn ccmd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "current command that is being executed."]
    #[inline(always)]
    pub const fn set_ccmd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "The current status of the command execution."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdstat(&self) -> super::super::shared::Cmdstat {
        let val = (self.0 >> 5usize) & 0x07;
        super::super::shared::Cmdstat::from_bits(val as u8)
    }
    #[doc = "The current status of the command execution."]
    #[inline(always)]
    pub const fn set_cmdstat(&mut self, val: super::super::shared::Cmdstat) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
    }
    #[doc = "The current number of bytes still to be transferred with this command. This field will count down to zero."]
    #[must_use]
    #[inline(always)]
    pub const fn ctsize(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "The current number of bytes still to be transferred with this command. This field will count down to zero."]
    #[inline(always)]
    pub const fn set_ctsize(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
}
impl Default for Cmdstat {
    #[inline(always)]
    fn default() -> Cmdstat {
        Cmdstat(0)
    }
}
impl core::fmt::Debug for Cmdstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdstat")
            .field("ccmd", &self.ccmd())
            .field("cmdstat", &self.cmdstat())
            .field("ctsize", &self.ctsize())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cmdstat {{ ccmd: {=u8:?}, cmdstat: {:?}, ctsize: {=u16:?} }}",
            self.ccmd(),
            self.cmdstat(),
            self.ctsize()
        )
    }
}
#[doc = "CQ Target Read Address Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cqaddr(pub u32);
impl Cqaddr {
    #[doc = "Bits 19:2 of target byte address for source of CQ (read only). The buffer must be aligned on a word boundary."]
    #[must_use]
    #[inline(always)]
    pub const fn cqaddr(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "Bits 19:2 of target byte address for source of CQ (read only). The buffer must be aligned on a word boundary."]
    #[inline(always)]
    pub const fn set_cqaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 2usize)) | (((val as u32) & 0x0003_ffff) << 2usize);
    }
    #[doc = "Bit 28 of target byte address for source of CQ (read only). Used to denote Flash (0) or SRAM (1) access."]
    #[must_use]
    #[inline(always)]
    pub const fn cqaddr28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Bit 28 of target byte address for source of CQ (read only). Used to denote Flash (0) or SRAM (1) access."]
    #[inline(always)]
    pub const fn set_cqaddr28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
            .field("cqaddr28", &self.cqaddr28())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cqaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cqaddr {{ cqaddr: {=u32:?}, cqaddr28: {=bool:?} }}",
            self.cqaddr(),
            self.cqaddr28()
        )
    }
}
#[doc = "Command Queue Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cqcfg(pub u32);
impl Cqcfg {
    #[doc = "Command queue enable. When set, will enable the processing of the command queue and fetches of address/data pairs will proceed from the word address within the CQADDR register. Can be disabled using a CQ executed write to this bit as well."]
    #[must_use]
    #[inline(always)]
    pub const fn cqen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue enable. When set, will enable the processing of the command queue and fetches of address/data pairs will proceed from the word address within the CQADDR register. Can be disabled using a CQ executed write to this bit as well."]
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
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cqcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cqcfg {{ cqen: {=bool:?}, cqpri: {:?} }}",
            self.cqen(),
            self.cqpri()
        )
    }
}
#[doc = "IOM Command Queue current index value . Compared to the CQENDIDX reg contents to generate the IDXEQ Pause event for command queue."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cqcuridx(pub u32);
impl Cqcuridx {
    #[doc = "Holds 8 bits of data that will be compared with the CQENDIX register field. If the values match, the IDXEQ pause event will be activated, which will cause the pausing of command quue operation if the IDXEQ bit is enabled in CQPAUSEEN."]
    #[must_use]
    #[inline(always)]
    pub const fn cqcuridx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Holds 8 bits of data that will be compared with the CQENDIX register field. If the values match, the IDXEQ pause event will be activated, which will cause the pausing of command quue operation if the IDXEQ bit is enabled in CQPAUSEEN."]
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
#[doc = "IOM Command Queue current index value . Compared to the CQCURIDX reg contents to generate the IDXEQ Pause event for command queue."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cqendidx(pub u32);
impl Cqendidx {
    #[doc = "Holds 8 bits of data that will be compared with the CQCURIX register field. If the values match, the IDXEQ pause event will be activated, which will cause the pausing of command quue operation if the IDXEQ bit is enabled in CQPAUSEEN."]
    #[must_use]
    #[inline(always)]
    pub const fn cqendidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Holds 8 bits of data that will be compared with the CQCURIX register field. If the values match, the IDXEQ pause event will be activated, which will cause the pausing of command quue operation if the IDXEQ bit is enabled in CQPAUSEEN."]
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
    pub const fn cqflags(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Current flag status (read-only). Bits \\[7:0\\] are software controllable and bits \\[15:8\\] are hardware status."]
    #[inline(always)]
    pub const fn set_cqflags(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Mask the bits used to generate the command queue interrupt. A '1' in the bit position will enable the pause event to trigger the interrupt, if the CQWT_int interrupt is enabled. Bits definitions are the same as CQPAUSE."]
    #[must_use]
    #[inline(always)]
    pub const fn cqirqmask(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Mask the bits used to generate the command queue interrupt. A '1' in the bit position will enable the pause event to trigger the interrupt, if the CQWT_int interrupt is enabled. Bits definitions are the same as CQPAUSE."]
    #[inline(always)]
    pub const fn set_cqirqmask(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
            .field("cqirqmask", &self.cqirqmask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cqflags {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cqflags {{ cqflags: {=u16:?}, cqirqmask: {=u16:?} }}",
            self.cqflags(),
            self.cqirqmask()
        )
    }
}
#[doc = "Command Queue Pause Enable Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cqpauseen(pub u32);
impl Cqpauseen {
    #[doc = "Enables the specified event to pause command processing when active."]
    #[must_use]
    #[inline(always)]
    pub const fn cqpen(&self) -> super::vals::Cqpen {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Cqpen::from_bits(val as u16)
    }
    #[doc = "Enables the specified event to pause command processing when active."]
    #[inline(always)]
    pub const fn set_cqpen(&mut self, val: super::vals::Cqpen) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
}
impl Default for Cqpauseen {
    #[inline(always)]
    fn default() -> Cqpauseen {
        Cqpauseen(0)
    }
}
impl core::fmt::Debug for Cqpauseen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cqpauseen")
            .field("cqpen", &self.cqpen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cqpauseen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cqpauseen {{ cqpen: {:?} }}", self.cqpen())
    }
}
#[doc = "Command Queue Flag Set/Clear Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cqsetclear(pub u32);
impl Cqsetclear {
    #[doc = "Set CQFlag status bits. Will set to 1 the value of any SWFLAG with a '1' in the corresponding bit position of this field."]
    #[must_use]
    #[inline(always)]
    pub const fn cqfset(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Set CQFlag status bits. Will set to 1 the value of any SWFLAG with a '1' in the corresponding bit position of this field."]
    #[inline(always)]
    pub const fn set_cqfset(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Toggle the indicated bit. Will toggle the value of any SWFLAG with a '1' in the corresponding bit position of this field."]
    #[must_use]
    #[inline(always)]
    pub const fn cqftgl(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Toggle the indicated bit. Will toggle the value of any SWFLAG with a '1' in the corresponding bit position of this field."]
    #[inline(always)]
    pub const fn set_cqftgl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Clear CQFlag status bits. Will clear to 0 any SWFLAG with a '1' in the corresponding bit position of this field."]
    #[must_use]
    #[inline(always)]
    pub const fn cqfclr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Clear CQFlag status bits. Will clear to 0 any SWFLAG with a '1' in the corresponding bit position of this field."]
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
            .field("cqftgl", &self.cqftgl())
            .field("cqfclr", &self.cqfclr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cqsetclear {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cqsetclear {{ cqfset: {=u8:?}, cqftgl: {=u8:?}, cqfclr: {=u8:?} }}",
            self.cqfset(),
            self.cqftgl(),
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
    #[doc = "Command queue operation is currently paused."]
    #[must_use]
    #[inline(always)]
    pub const fn cqpaused(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue operation is currently paused."]
    #[inline(always)]
    pub const fn set_cqpaused(&mut self, val: bool) {
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
            .field("cqpaused", &self.cqpaused())
            .field("cqerr", &self.cqerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cqstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cqstat {{ cqtip: {=bool:?}, cqpaused: {=bool:?}, cqerr: {=bool:?} }}",
            self.cqtip(),
            self.cqpaused(),
            self.cqerr()
        )
    }
}
#[doc = "DCX Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcx(pub u32);
impl Dcx {
    #[doc = "Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE0 output."]
    #[must_use]
    #[inline(always)]
    pub const fn ce0out(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE0 output."]
    #[inline(always)]
    pub const fn set_ce0out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE1 output."]
    #[must_use]
    #[inline(always)]
    pub const fn ce1out(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE1 output."]
    #[inline(always)]
    pub const fn set_ce1out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE2 output."]
    #[must_use]
    #[inline(always)]
    pub const fn ce2out(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE2 output."]
    #[inline(always)]
    pub const fn set_ce2out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE3 output."]
    #[must_use]
    #[inline(always)]
    pub const fn ce3out(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Revision A: MUST NOT be programmed! Revision B: Enable DCX output for CE3 output."]
    #[inline(always)]
    pub const fn set_ce3out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Revision A: MUST NOT be programmed! Revision B: Bit 4: DCX Signaling Enable via other CE signals. The selected DCX signal (unused CE pin) will be driven low during write of offset byte, and high during transmission of data bytes."]
    #[must_use]
    #[inline(always)]
    pub const fn dcxen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Revision A: MUST NOT be programmed! Revision B: Bit 4: DCX Signaling Enable via other CE signals. The selected DCX signal (unused CE pin) will be driven low during write of offset byte, and high during transmission of data bytes."]
    #[inline(always)]
    pub const fn set_dcxen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Dcx {
    #[inline(always)]
    fn default() -> Dcx {
        Dcx(0)
    }
}
impl core::fmt::Debug for Dcx {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dcx")
            .field("ce0out", &self.ce0out())
            .field("ce1out", &self.ce1out())
            .field("ce2out", &self.ce2out())
            .field("ce3out", &self.ce3out())
            .field("dcxen", &self.dcxen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcx {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dcx {{ ce0out: {=bool:?}, ce1out: {=bool:?}, ce2out: {=bool:?}, ce3out: {=bool:?}, dcxen: {=bool:?} }}" , self . ce0out () , self . ce1out () , self . ce2out () , self . ce3out () , self . dcxen ())
    }
}
#[doc = "I2C Device Configuration register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Devcfg(pub u32);
impl Devcfg {
    #[doc = "I2C address of the device that the Master will use to target for read/write operations. This can be either a 7b or 10b address."]
    #[must_use]
    #[inline(always)]
    pub const fn devaddr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "I2C address of the device that the Master will use to target for read/write operations. This can be either a 7b or 10b address."]
    #[inline(always)]
    pub const fn set_devaddr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Devcfg {
    #[inline(always)]
    fn default() -> Devcfg {
        Devcfg(0)
    }
}
impl core::fmt::Debug for Devcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Devcfg")
            .field("devaddr", &self.devaddr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Devcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Devcfg {{ devaddr: {=u16:?} }}", self.devaddr())
    }
}
#[doc = "DMA Configuration Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmacfg(pub u32);
impl Dmacfg {
    #[doc = "DMA Enable. Setting this bit to EN will start the DMA operation. This should be the last DMA related register set prior to issuing the command."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Enable. Setting this bit to EN will start the DMA operation. This should be the last DMA related register set prior to issuing the command."]
    #[inline(always)]
    pub const fn set_dmaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Direction."]
    #[must_use]
    #[inline(always)]
    pub const fn dmadir(&self) -> super::super::shared::Dmadir {
        let val = (self.0 >> 1usize) & 0x01;
        super::super::shared::Dmadir::from_bits(val as u8)
    }
    #[doc = "Direction."]
    #[inline(always)]
    pub const fn set_dmadir(&mut self, val: super::super::shared::Dmadir) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
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
    #[doc = "Power off module after DMA is complete. If this bit is active, the module will request to power off the supply it is attached to. If there are other units still requiring power from the same domain, power down will not be performed."]
    #[must_use]
    #[inline(always)]
    pub const fn dpwroff(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Power off module after DMA is complete. If this bit is active, the module will request to power off the supply it is attached to. If there are other units still requiring power from the same domain, power down will not be performed."]
    #[inline(always)]
    pub const fn set_dpwroff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
            .field("dpwroff", &self.dpwroff())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmacfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmacfg {{ dmaen: {=bool:?}, dmadir: {:?}, dmapri: {:?}, dpwroff: {=bool:?} }}",
            self.dmaen(),
            self.dmadir(),
            self.dmapri(),
            self.dpwroff()
        )
    }
}
#[doc = "DMA Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmastat(pub u32);
impl Dmastat {
    #[doc = "DMA Transfer In Progress indicator. 1 will indicate that a DMA transfer is active. The DMA transfer may be waiting on data, transferring data, or waiting for priority. All of these will be indicated with a 1. A 0 will indicate that the DMA is fully complete and no further transactions will be done. This bit is read only."]
    #[must_use]
    #[inline(always)]
    pub const fn dmatip(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Transfer In Progress indicator. 1 will indicate that a DMA transfer is active. The DMA transfer may be waiting on data, transferring data, or waiting for priority. All of these will be indicated with a 1. A 0 will indicate that the DMA is fully complete and no further transactions will be done. This bit is read only."]
    #[inline(always)]
    pub const fn set_dmatip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA Transfer Complete. This signals the end of the DMA operation. This bit can be cleared by writing to 0, and will also be cleared when a new DMA is started."]
    #[must_use]
    #[inline(always)]
    pub const fn dmacpl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Transfer Complete. This signals the end of the DMA operation. This bit can be cleared by writing to 0, and will also be cleared when a new DMA is started."]
    #[inline(always)]
    pub const fn set_dmacpl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA Error. This active high bit signals an error was encountered during the DMA operation. The bit can be cleared by writing to 0. Once set, this bit will remain set until cleared by software."]
    #[must_use]
    #[inline(always)]
    pub const fn dmaerr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error. This active high bit signals an error was encountered during the DMA operation. The bit can be cleared by writing to 0. Once set, this bit will remain set until cleared by software."]
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
    #[doc = "Bits \\[19:0\\] of the target byte address for source of DMA (either read or write). The address can be any byte alignment, and does not have to be word aligned. In cases of non-word aligned addresses, the DMA logic will take care for ensuring only the target bytes are read/written."]
    #[must_use]
    #[inline(always)]
    pub const fn targaddr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "Bits \\[19:0\\] of the target byte address for source of DMA (either read or write). The address can be any byte alignment, and does not have to be word aligned. In cases of non-word aligned addresses, the DMA logic will take care for ensuring only the target bytes are read/written."]
    #[inline(always)]
    pub const fn set_targaddr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
    #[doc = "Bit 28 of the target byte address for source of DMA (either read or write). In cases of non-word aligned addresses, the DMA logic will take care for ensuring only the target bytes are read/written. Setting to '1' will select the SRAM. Setting to '0' will select the flash."]
    #[must_use]
    #[inline(always)]
    pub const fn targaddr28(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Bit 28 of the target byte address for source of DMA (either read or write). In cases of non-word aligned addresses, the DMA logic will take care for ensuring only the target bytes are read/written. Setting to '1' will select the SRAM. Setting to '0' will select the flash."]
    #[inline(always)]
    pub const fn set_targaddr28(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
            .field("targaddr", &self.targaddr())
            .field("targaddr28", &self.targaddr28())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmatargaddr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmatargaddr {{ targaddr: {=u32:?}, targaddr28: {=bool:?} }}",
            self.targaddr(),
            self.targaddr28()
        )
    }
}
#[doc = "DMA Total Transfer Count."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmatotcount(pub u32);
impl Dmatotcount {
    #[doc = "Triggered DMA from Command complete event occured. Bit is read only and can be cleared by disabling the DTHR trigger enable or by disabling DMA."]
    #[must_use]
    #[inline(always)]
    pub const fn totcount(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Triggered DMA from Command complete event occured. Bit is read only and can be cleared by disabling the DTHR trigger enable or by disabling DMA."]
    #[inline(always)]
    pub const fn set_totcount(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
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
    #[doc = "Trigger DMA upon command complete. Enables the trigger of the DMA when a command is completed. When this event is triggered, the number of words transferred will be the lesser of the remaining TOTCOUNT bytes, or."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmdcmpen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger DMA upon command complete. Enables the trigger of the DMA when a command is completed. When this event is triggered, the number of words transferred will be the lesser of the remaining TOTCOUNT bytes, or."]
    #[inline(always)]
    pub const fn set_dcmdcmpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger DMA upon THR level reached. For M2P DMA operations (IOM writes), the trigger will assert when the write FIFO has (WTHR/4) number of words free in the write FIFO, and will transfer (WTHR/4) number of words or, if the number of words left to transfer is less than the WTHR value, will transfer the remaining byte count. For P2M DMA operations, the trigger will assert when the read FIFO has (RTHR/4) words available in the read FIFO, and will transfer (RTHR/4) words to SRAM. This trigger will NOT assert when the transaction completes and there are less than RTHR bytes left in the fifo, since the RTHR has not been reached. In this case, the CMDCMP trigger must also be enabled to transfer the remaining read FIFO data to SRAM."]
    #[must_use]
    #[inline(always)]
    pub const fn dthren(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger DMA upon THR level reached. For M2P DMA operations (IOM writes), the trigger will assert when the write FIFO has (WTHR/4) number of words free in the write FIFO, and will transfer (WTHR/4) number of words or, if the number of words left to transfer is less than the WTHR value, will transfer the remaining byte count. For P2M DMA operations, the trigger will assert when the read FIFO has (RTHR/4) words available in the read FIFO, and will transfer (RTHR/4) words to SRAM. This trigger will NOT assert when the transaction completes and there are less than RTHR bytes left in the fifo, since the RTHR has not been reached. In this case, the CMDCMP trigger must also be enabled to transfer the remaining read FIFO data to SRAM."]
    #[inline(always)]
    pub const fn set_dthren(&mut self, val: bool) {
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
            .field("dcmdcmpen", &self.dcmdcmpen())
            .field("dthren", &self.dthren())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmatrigen {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmatrigen {{ dcmdcmpen: {=bool:?}, dthren: {=bool:?} }}",
            self.dcmdcmpen(),
            self.dthren()
        )
    }
}
#[doc = "DMA Trigger Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmatrigstat(pub u32);
impl Dmatrigstat {
    #[doc = "Triggered DMA from Command complete event. Bit is read only and can be cleared by disabling the DCMDCMP trigger enable or by disabling DMA."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmdcmp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Triggered DMA from Command complete event. Bit is read only and can be cleared by disabling the DCMDCMP trigger enable or by disabling DMA."]
    #[inline(always)]
    pub const fn set_dcmdcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Triggered DMA from THR event. Bit is read only and can be cleared by disabling the DTHR trigger enable or by disabling DMA."]
    #[must_use]
    #[inline(always)]
    pub const fn dthr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Triggered DMA from THR event. Bit is read only and can be cleared by disabling the DTHR trigger enable or by disabling DMA."]
    #[inline(always)]
    pub const fn set_dthr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA triggered when DCMDCMP = 0, and the amount of data in the FIFO was enough to complete the DMA operation (greater than or equal to current TOTCOUNT) when the command completed. This trigger is default active when the DCMDCMP trigger is disabled and there is enough data in the FIFO to complete the DMA operation."]
    #[must_use]
    #[inline(always)]
    pub const fn dtotcmp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA triggered when DCMDCMP = 0, and the amount of data in the FIFO was enough to complete the DMA operation (greater than or equal to current TOTCOUNT) when the command completed. This trigger is default active when the DCMDCMP trigger is disabled and there is enough data in the FIFO to complete the DMA operation."]
    #[inline(always)]
    pub const fn set_dtotcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
            .field("dcmdcmp", &self.dcmdcmp())
            .field("dthr", &self.dthr())
            .field("dtotcmp", &self.dtotcmp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmatrigstat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dmatrigstat {{ dcmdcmp: {=bool:?}, dthr: {=bool:?}, dtotcmp: {=bool:?} }}",
            self.dcmdcmp(),
            self.dthr(),
            self.dtotcmp()
        )
    }
}
#[doc = "FIFO Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifoctrl(pub u32);
impl Fifoctrl {
    #[doc = "Selects the mode in which 'pop' events are done for the fifo read operations. A value of '1' will prevent a pop event on a read operation, and will require a write to the FIFOPOP register to create a pop event. A value of '0' in this register will allow a pop event to occur on the read of the FIFOPOP register, and may cause inadvertant fifo pops when used in a debugging mode."]
    #[must_use]
    #[inline(always)]
    pub const fn popwr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Selects the mode in which 'pop' events are done for the fifo read operations. A value of '1' will prevent a pop event on a read operation, and will require a write to the FIFOPOP register to create a pop event. A value of '0' in this register will allow a pop event to occur on the read of the FIFOPOP register, and may cause inadvertant fifo pops when used in a debugging mode."]
    #[inline(always)]
    pub const fn set_popwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Active low manual reset of the fifo. Write to 0 to reset fifo, and then write to 1 to remove the reset."]
    #[must_use]
    #[inline(always)]
    pub const fn fiforstn(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Active low manual reset of the fifo. Write to 0 to reset fifo, and then write to 1 to remove the reset."]
    #[inline(always)]
    pub const fn set_fiforstn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Fifoctrl {
    #[inline(always)]
    fn default() -> Fifoctrl {
        Fifoctrl(0)
    }
}
impl core::fmt::Debug for Fifoctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifoctrl")
            .field("popwr", &self.popwr())
            .field("fiforstn", &self.fiforstn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifoctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fifoctrl {{ popwr: {=bool:?}, fiforstn: {=bool:?} }}",
            self.popwr(),
            self.fiforstn()
        )
    }
}
#[doc = "FIFO Pointers."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifoloc(pub u32);
impl Fifoloc {
    #[doc = "Current FIFO write pointer. Value is the index into the outgoing FIFO (FIFO0), which is used during write operations to external devices."]
    #[must_use]
    #[inline(always)]
    pub const fn fifowptr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Current FIFO write pointer. Value is the index into the outgoing FIFO (FIFO0), which is used during write operations to external devices."]
    #[inline(always)]
    pub const fn set_fifowptr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Current FIFO read pointer. Used to index into the incoming FIFO (FIFO1), which is used to store read data returned from external devices during a read operation."]
    #[must_use]
    #[inline(always)]
    pub const fn fiforptr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Current FIFO read pointer. Used to index into the incoming FIFO (FIFO1), which is used to store read data returned from external devices during a read operation."]
    #[inline(always)]
    pub const fn set_fiforptr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for Fifoloc {
    #[inline(always)]
    fn default() -> Fifoloc {
        Fifoloc(0)
    }
}
impl core::fmt::Debug for Fifoloc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifoloc")
            .field("fifowptr", &self.fifowptr())
            .field("fiforptr", &self.fiforptr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifoloc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fifoloc {{ fifowptr: {=u8:?}, fiforptr: {=u8:?} }}",
            self.fifowptr(),
            self.fiforptr()
        )
    }
}
#[doc = "FIFO size and remaining slots open values."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifoptr(pub u32);
impl Fifoptr {
    #[doc = "The number of valid data bytes currently in the FIFO 0 (written by MCU, read by interface)."]
    #[must_use]
    #[inline(always)]
    pub const fn fifo0siz(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The number of valid data bytes currently in the FIFO 0 (written by MCU, read by interface)."]
    #[inline(always)]
    pub const fn set_fifo0siz(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "The number of remaining data bytes slots currently in FIFO 0 (written by MCU, read by interface)."]
    #[must_use]
    #[inline(always)]
    pub const fn fifo0rem(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "The number of remaining data bytes slots currently in FIFO 0 (written by MCU, read by interface)."]
    #[inline(always)]
    pub const fn set_fifo0rem(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "The number of valid data bytes currently in FIFO 1 (written by interface, read by MCU)."]
    #[must_use]
    #[inline(always)]
    pub const fn fifo1siz(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "The number of valid data bytes currently in FIFO 1 (written by interface, read by MCU)."]
    #[inline(always)]
    pub const fn set_fifo1siz(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "The number of remaining data bytes slots currently in FIFO 1 (written by interface, read by MCU)."]
    #[must_use]
    #[inline(always)]
    pub const fn fifo1rem(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "The number of remaining data bytes slots currently in FIFO 1 (written by interface, read by MCU)."]
    #[inline(always)]
    pub const fn set_fifo1rem(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Fifoptr {
    #[inline(always)]
    fn default() -> Fifoptr {
        Fifoptr(0)
    }
}
impl core::fmt::Debug for Fifoptr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifoptr")
            .field("fifo0siz", &self.fifo0siz())
            .field("fifo0rem", &self.fifo0rem())
            .field("fifo1siz", &self.fifo1siz())
            .field("fifo1rem", &self.fifo1rem())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifoptr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Fifoptr {{ fifo0siz: {=u8:?}, fifo0rem: {=u8:?}, fifo1siz: {=u8:?}, fifo1rem: {=u8:?} }}" , self . fifo0siz () , self . fifo0rem () , self . fifo1siz () , self . fifo1rem ())
    }
}
#[doc = "FIFO Threshold Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifothr(pub u32);
impl Fifothr {
    #[doc = "FIFO read threshold in bytes. A value of 0 will disable the read FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the read fifo contains FIFORTHR valid bytes of data, as indicated by the FIFO1SIZ field. This is intended to signal when a data transfer of FIFORTHR bytes can be done from the IOM module to the host via the read fifo to support large IOM read operations."]
    #[must_use]
    #[inline(always)]
    pub const fn fiforthr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "FIFO read threshold in bytes. A value of 0 will disable the read FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the read fifo contains FIFORTHR valid bytes of data, as indicated by the FIFO1SIZ field. This is intended to signal when a data transfer of FIFORTHR bytes can be done from the IOM module to the host via the read fifo to support large IOM read operations."]
    #[inline(always)]
    pub const fn set_fiforthr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "FIFO write threshold in bytes. A value of 0 will disable the write FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the write fifo contains FIFOWTHR free bytes, as indicated by the FIFO0REM field. This is intended to signal when a transfer of FIFOWTHR bytes can be done from the host to the IOM write fifo to support large IOM write operations."]
    #[must_use]
    #[inline(always)]
    pub const fn fifowthr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "FIFO write threshold in bytes. A value of 0 will disable the write FIFO level from activating the threshold interrupt. If this field is non-zero, it will trigger a threshold interrupt when the write fifo contains FIFOWTHR free bytes, as indicated by the FIFO0REM field. This is intended to signal when a transfer of FIFOWTHR bytes can be done from the host to the IOM write fifo to support large IOM write operations."]
    #[inline(always)]
    pub const fn set_fifowthr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
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
            .field("fiforthr", &self.fiforthr())
            .field("fifowthr", &self.fifowthr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifothr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fifothr {{ fiforthr: {=u8:?}, fifowthr: {=u8:?} }}",
            self.fiforthr(),
            self.fifowthr()
        )
    }
}
#[doc = "IO Master Interrupts: Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intclr(pub u32);
impl Intclr {
    #[doc = "Command Complete interrupt. Asserted when the current operation has completed. For repeated commands, this will only be asserted when the final repeated command is completed."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdcmp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Command Complete interrupt. Asserted when the current operation has completed. For repeated commands, this will only be asserted when the final repeated command is completed."]
    #[inline(always)]
    pub const fn set_cmdcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Threshold interrupt. For write operations, asserted when the number of free bytes in the write FIFO equals or exceeds the WTHR field. For read operations, asserted when the number of valid bytes in the read FIFO equals of exceeds the value set in the RTHR field."]
    #[must_use]
    #[inline(always)]
    pub const fn thr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Threshold interrupt. For write operations, asserted when the number of free bytes in the write FIFO equals or exceeds the WTHR field. For read operations, asserted when the number of valid bytes in the read FIFO equals of exceeds the value set in the RTHR field."]
    #[inline(always)]
    pub const fn set_thr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Read FIFO Underflow interrupt. This occurs when software tries to pop from an empty fifo."]
    #[must_use]
    #[inline(always)]
    pub const fn fundfl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Read FIFO Underflow interrupt. This occurs when software tries to pop from an empty fifo."]
    #[inline(always)]
    pub const fn set_fundfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write FIFO Overflow interrupt. This occurs when software tries to write to a full fifo. The current operation does not stop."]
    #[must_use]
    #[inline(always)]
    pub const fn fovfl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write FIFO Overflow interrupt. This occurs when software tries to write to a full fifo. The current operation does not stop."]
    #[inline(always)]
    pub const fn set_fovfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "I2C NAK interrupt. Asserted when an unexpected NAK has been received on the I2C bus."]
    #[must_use]
    #[inline(always)]
    pub const fn nak(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "I2C NAK interrupt. Asserted when an unexpected NAK has been received on the I2C bus."]
    #[inline(always)]
    pub const fn set_nak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "illegal FIFO access interrupt. Asserted when there is a overflow or underflow event."]
    #[must_use]
    #[inline(always)]
    pub const fn iacc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "illegal FIFO access interrupt. Asserted when there is a overflow or underflow event."]
    #[inline(always)]
    pub const fn set_iacc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "illegal command interrupt. Asserted when a command is written when an active command is in progress."]
    #[must_use]
    #[inline(always)]
    pub const fn icmd(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "illegal command interrupt. Asserted when a command is written when an active command is in progress."]
    #[inline(always)]
    pub const fn set_icmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "START command interrupt. Asserted when another master on the bus has signaled a START command."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "START command interrupt. Asserted when another master on the bus has signaled a START command."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "STOP command interrupt. Asserted when another master on the bus has signaled a STOP command."]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "STOP command interrupt. Asserted when another master on the bus has signaled a STOP command."]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Arbitration loss interrupt. Asserted when arbitration is enabled and has been lost to another master on the bus."]
    #[must_use]
    #[inline(always)]
    pub const fn arb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration loss interrupt. Asserted when arbitration is enabled and has been lost to another master on the bus."]
    #[inline(always)]
    pub const fn set_arb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmp(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state."]
    #[inline(always)]
    pub const fn set_dcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
    #[must_use]
    #[inline(always)]
    pub const fn derr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
    #[inline(always)]
    pub const fn set_derr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
    #[must_use]
    #[inline(always)]
    pub const fn cqpaused(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
    #[inline(always)]
    pub const fn set_cqpaused(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CQ write operation performed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
    #[must_use]
    #[inline(always)]
    pub const fn cqupd(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CQ write operation performed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
    #[inline(always)]
    pub const fn set_cqupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Error during command queue operations."]
    #[must_use]
    #[inline(always)]
    pub const fn cqerr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Error during command queue operations."]
    #[inline(always)]
    pub const fn set_cqerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
            .field("thr", &self.thr())
            .field("fundfl", &self.fundfl())
            .field("fovfl", &self.fovfl())
            .field("nak", &self.nak())
            .field("iacc", &self.iacc())
            .field("icmd", &self.icmd())
            .field("start", &self.start())
            .field("stop", &self.stop())
            .field("arb", &self.arb())
            .field("dcmp", &self.dcmp())
            .field("derr", &self.derr())
            .field("cqpaused", &self.cqpaused())
            .field("cqupd", &self.cqupd())
            .field("cqerr", &self.cqerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intclr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Intclr {{ cmdcmp: {=bool:?}, thr: {=bool:?}, fundfl: {=bool:?}, fovfl: {=bool:?}, nak: {=bool:?}, iacc: {=bool:?}, icmd: {=bool:?}, start: {=bool:?}, stop: {=bool:?}, arb: {=bool:?}, dcmp: {=bool:?}, derr: {=bool:?}, cqpaused: {=bool:?}, cqupd: {=bool:?}, cqerr: {=bool:?} }}" , self . cmdcmp () , self . thr () , self . fundfl () , self . fovfl () , self . nak () , self . iacc () , self . icmd () , self . start () , self . stop () , self . arb () , self . dcmp () , self . derr () , self . cqpaused () , self . cqupd () , self . cqerr ())
    }
}
#[doc = "IO Master Interrupts: Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Command Complete interrupt. Asserted when the current operation has completed. For repeated commands, this will only be asserted when the final repeated command is completed."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdcmp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Command Complete interrupt. Asserted when the current operation has completed. For repeated commands, this will only be asserted when the final repeated command is completed."]
    #[inline(always)]
    pub const fn set_cmdcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Threshold interrupt. For write operations, asserted when the number of free bytes in the write FIFO equals or exceeds the WTHR field. For read operations, asserted when the number of valid bytes in the read FIFO equals of exceeds the value set in the RTHR field."]
    #[must_use]
    #[inline(always)]
    pub const fn thr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Threshold interrupt. For write operations, asserted when the number of free bytes in the write FIFO equals or exceeds the WTHR field. For read operations, asserted when the number of valid bytes in the read FIFO equals of exceeds the value set in the RTHR field."]
    #[inline(always)]
    pub const fn set_thr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Read FIFO Underflow interrupt. This occurs when software tries to pop from an empty fifo."]
    #[must_use]
    #[inline(always)]
    pub const fn fundfl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Read FIFO Underflow interrupt. This occurs when software tries to pop from an empty fifo."]
    #[inline(always)]
    pub const fn set_fundfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write FIFO Overflow interrupt. This occurs when software tries to write to a full fifo. The current operation does not stop."]
    #[must_use]
    #[inline(always)]
    pub const fn fovfl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write FIFO Overflow interrupt. This occurs when software tries to write to a full fifo. The current operation does not stop."]
    #[inline(always)]
    pub const fn set_fovfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "I2C NAK interrupt. Asserted when an unexpected NAK has been received on the I2C bus."]
    #[must_use]
    #[inline(always)]
    pub const fn nak(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "I2C NAK interrupt. Asserted when an unexpected NAK has been received on the I2C bus."]
    #[inline(always)]
    pub const fn set_nak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "illegal FIFO access interrupt. Asserted when there is a overflow or underflow event."]
    #[must_use]
    #[inline(always)]
    pub const fn iacc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "illegal FIFO access interrupt. Asserted when there is a overflow or underflow event."]
    #[inline(always)]
    pub const fn set_iacc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "illegal command interrupt. Asserted when a command is written when an active command is in progress."]
    #[must_use]
    #[inline(always)]
    pub const fn icmd(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "illegal command interrupt. Asserted when a command is written when an active command is in progress."]
    #[inline(always)]
    pub const fn set_icmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "START command interrupt. Asserted when another master on the bus has signaled a START command."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "START command interrupt. Asserted when another master on the bus has signaled a START command."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "STOP command interrupt. Asserted when another master on the bus has signaled a STOP command."]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "STOP command interrupt. Asserted when another master on the bus has signaled a STOP command."]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Arbitration loss interrupt. Asserted when arbitration is enabled and has been lost to another master on the bus."]
    #[must_use]
    #[inline(always)]
    pub const fn arb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration loss interrupt. Asserted when arbitration is enabled and has been lost to another master on the bus."]
    #[inline(always)]
    pub const fn set_arb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmp(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state."]
    #[inline(always)]
    pub const fn set_dcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
    #[must_use]
    #[inline(always)]
    pub const fn derr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
    #[inline(always)]
    pub const fn set_derr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
    #[must_use]
    #[inline(always)]
    pub const fn cqpaused(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
    #[inline(always)]
    pub const fn set_cqpaused(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CQ write operation performed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
    #[must_use]
    #[inline(always)]
    pub const fn cqupd(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CQ write operation performed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
    #[inline(always)]
    pub const fn set_cqupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Error during command queue operations."]
    #[must_use]
    #[inline(always)]
    pub const fn cqerr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Error during command queue operations."]
    #[inline(always)]
    pub const fn set_cqerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
            .field("thr", &self.thr())
            .field("fundfl", &self.fundfl())
            .field("fovfl", &self.fovfl())
            .field("nak", &self.nak())
            .field("iacc", &self.iacc())
            .field("icmd", &self.icmd())
            .field("start", &self.start())
            .field("stop", &self.stop())
            .field("arb", &self.arb())
            .field("dcmp", &self.dcmp())
            .field("derr", &self.derr())
            .field("cqpaused", &self.cqpaused())
            .field("cqupd", &self.cqupd())
            .field("cqerr", &self.cqerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Inten {{ cmdcmp: {=bool:?}, thr: {=bool:?}, fundfl: {=bool:?}, fovfl: {=bool:?}, nak: {=bool:?}, iacc: {=bool:?}, icmd: {=bool:?}, start: {=bool:?}, stop: {=bool:?}, arb: {=bool:?}, dcmp: {=bool:?}, derr: {=bool:?}, cqpaused: {=bool:?}, cqupd: {=bool:?}, cqerr: {=bool:?} }}" , self . cmdcmp () , self . thr () , self . fundfl () , self . fovfl () , self . nak () , self . iacc () , self . icmd () , self . start () , self . stop () , self . arb () , self . dcmp () , self . derr () , self . cqpaused () , self . cqupd () , self . cqerr ())
    }
}
#[doc = "IO Master Interrupts: Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intset(pub u32);
impl Intset {
    #[doc = "Command Complete interrupt. Asserted when the current operation has completed. For repeated commands, this will only be asserted when the final repeated command is completed."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdcmp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Command Complete interrupt. Asserted when the current operation has completed. For repeated commands, this will only be asserted when the final repeated command is completed."]
    #[inline(always)]
    pub const fn set_cmdcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Threshold interrupt. For write operations, asserted when the number of free bytes in the write FIFO equals or exceeds the WTHR field. For read operations, asserted when the number of valid bytes in the read FIFO equals of exceeds the value set in the RTHR field."]
    #[must_use]
    #[inline(always)]
    pub const fn thr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Threshold interrupt. For write operations, asserted when the number of free bytes in the write FIFO equals or exceeds the WTHR field. For read operations, asserted when the number of valid bytes in the read FIFO equals of exceeds the value set in the RTHR field."]
    #[inline(always)]
    pub const fn set_thr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Read FIFO Underflow interrupt. This occurs when software tries to pop from an empty fifo."]
    #[must_use]
    #[inline(always)]
    pub const fn fundfl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Read FIFO Underflow interrupt. This occurs when software tries to pop from an empty fifo."]
    #[inline(always)]
    pub const fn set_fundfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write FIFO Overflow interrupt. This occurs when software tries to write to a full fifo. The current operation does not stop."]
    #[must_use]
    #[inline(always)]
    pub const fn fovfl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write FIFO Overflow interrupt. This occurs when software tries to write to a full fifo. The current operation does not stop."]
    #[inline(always)]
    pub const fn set_fovfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "I2C NAK interrupt. Asserted when an unexpected NAK has been received on the I2C bus."]
    #[must_use]
    #[inline(always)]
    pub const fn nak(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "I2C NAK interrupt. Asserted when an unexpected NAK has been received on the I2C bus."]
    #[inline(always)]
    pub const fn set_nak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "illegal FIFO access interrupt. Asserted when there is a overflow or underflow event."]
    #[must_use]
    #[inline(always)]
    pub const fn iacc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "illegal FIFO access interrupt. Asserted when there is a overflow or underflow event."]
    #[inline(always)]
    pub const fn set_iacc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "illegal command interrupt. Asserted when a command is written when an active command is in progress."]
    #[must_use]
    #[inline(always)]
    pub const fn icmd(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "illegal command interrupt. Asserted when a command is written when an active command is in progress."]
    #[inline(always)]
    pub const fn set_icmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "START command interrupt. Asserted when another master on the bus has signaled a START command."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "START command interrupt. Asserted when another master on the bus has signaled a START command."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "STOP command interrupt. Asserted when another master on the bus has signaled a STOP command."]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "STOP command interrupt. Asserted when another master on the bus has signaled a STOP command."]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Arbitration loss interrupt. Asserted when arbitration is enabled and has been lost to another master on the bus."]
    #[must_use]
    #[inline(always)]
    pub const fn arb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration loss interrupt. Asserted when arbitration is enabled and has been lost to another master on the bus."]
    #[inline(always)]
    pub const fn set_arb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmp(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state."]
    #[inline(always)]
    pub const fn set_dcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
    #[must_use]
    #[inline(always)]
    pub const fn derr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
    #[inline(always)]
    pub const fn set_derr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
    #[must_use]
    #[inline(always)]
    pub const fn cqpaused(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
    #[inline(always)]
    pub const fn set_cqpaused(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CQ write operation performed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
    #[must_use]
    #[inline(always)]
    pub const fn cqupd(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CQ write operation performed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
    #[inline(always)]
    pub const fn set_cqupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Error during command queue operations."]
    #[must_use]
    #[inline(always)]
    pub const fn cqerr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Error during command queue operations."]
    #[inline(always)]
    pub const fn set_cqerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
            .field("thr", &self.thr())
            .field("fundfl", &self.fundfl())
            .field("fovfl", &self.fovfl())
            .field("nak", &self.nak())
            .field("iacc", &self.iacc())
            .field("icmd", &self.icmd())
            .field("start", &self.start())
            .field("stop", &self.stop())
            .field("arb", &self.arb())
            .field("dcmp", &self.dcmp())
            .field("derr", &self.derr())
            .field("cqpaused", &self.cqpaused())
            .field("cqupd", &self.cqupd())
            .field("cqerr", &self.cqerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intset {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Intset {{ cmdcmp: {=bool:?}, thr: {=bool:?}, fundfl: {=bool:?}, fovfl: {=bool:?}, nak: {=bool:?}, iacc: {=bool:?}, icmd: {=bool:?}, start: {=bool:?}, stop: {=bool:?}, arb: {=bool:?}, dcmp: {=bool:?}, derr: {=bool:?}, cqpaused: {=bool:?}, cqupd: {=bool:?}, cqerr: {=bool:?} }}" , self . cmdcmp () , self . thr () , self . fundfl () , self . fovfl () , self . nak () , self . iacc () , self . icmd () , self . start () , self . stop () , self . arb () , self . dcmp () , self . derr () , self . cqpaused () , self . cqupd () , self . cqerr ())
    }
}
#[doc = "IO Master Interrupts: Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "Command Complete interrupt. Asserted when the current operation has completed. For repeated commands, this will only be asserted when the final repeated command is completed."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdcmp(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Command Complete interrupt. Asserted when the current operation has completed. For repeated commands, this will only be asserted when the final repeated command is completed."]
    #[inline(always)]
    pub const fn set_cmdcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Threshold interrupt. For write operations, asserted when the number of free bytes in the write FIFO equals or exceeds the WTHR field. For read operations, asserted when the number of valid bytes in the read FIFO equals of exceeds the value set in the RTHR field."]
    #[must_use]
    #[inline(always)]
    pub const fn thr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Threshold interrupt. For write operations, asserted when the number of free bytes in the write FIFO equals or exceeds the WTHR field. For read operations, asserted when the number of valid bytes in the read FIFO equals of exceeds the value set in the RTHR field."]
    #[inline(always)]
    pub const fn set_thr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Read FIFO Underflow interrupt. This occurs when software tries to pop from an empty fifo."]
    #[must_use]
    #[inline(always)]
    pub const fn fundfl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Read FIFO Underflow interrupt. This occurs when software tries to pop from an empty fifo."]
    #[inline(always)]
    pub const fn set_fundfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write FIFO Overflow interrupt. This occurs when software tries to write to a full fifo. The current operation does not stop."]
    #[must_use]
    #[inline(always)]
    pub const fn fovfl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write FIFO Overflow interrupt. This occurs when software tries to write to a full fifo. The current operation does not stop."]
    #[inline(always)]
    pub const fn set_fovfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "I2C NAK interrupt. Asserted when an unexpected NAK has been received on the I2C bus."]
    #[must_use]
    #[inline(always)]
    pub const fn nak(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "I2C NAK interrupt. Asserted when an unexpected NAK has been received on the I2C bus."]
    #[inline(always)]
    pub const fn set_nak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "illegal FIFO access interrupt. Asserted when there is a overflow or underflow event."]
    #[must_use]
    #[inline(always)]
    pub const fn iacc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "illegal FIFO access interrupt. Asserted when there is a overflow or underflow event."]
    #[inline(always)]
    pub const fn set_iacc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "illegal command interrupt. Asserted when a command is written when an active command is in progress."]
    #[must_use]
    #[inline(always)]
    pub const fn icmd(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "illegal command interrupt. Asserted when a command is written when an active command is in progress."]
    #[inline(always)]
    pub const fn set_icmd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "START command interrupt. Asserted when another master on the bus has signaled a START command."]
    #[must_use]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "START command interrupt. Asserted when another master on the bus has signaled a START command."]
    #[inline(always)]
    pub const fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "STOP command interrupt. Asserted when another master on the bus has signaled a STOP command."]
    #[must_use]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "STOP command interrupt. Asserted when another master on the bus has signaled a STOP command."]
    #[inline(always)]
    pub const fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Arbitration loss interrupt. Asserted when arbitration is enabled and has been lost to another master on the bus."]
    #[must_use]
    #[inline(always)]
    pub const fn arb(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Arbitration loss interrupt. Asserted when arbitration is enabled and has been lost to another master on the bus."]
    #[inline(always)]
    pub const fn set_arb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmp(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state."]
    #[inline(always)]
    pub const fn set_dcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
    #[must_use]
    #[inline(always)]
    pub const fn derr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
    #[inline(always)]
    pub const fn set_derr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
    #[must_use]
    #[inline(always)]
    pub const fn cqpaused(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
    #[inline(always)]
    pub const fn set_cqpaused(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "CQ write operation performed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
    #[must_use]
    #[inline(always)]
    pub const fn cqupd(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "CQ write operation performed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
    #[inline(always)]
    pub const fn set_cqupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Error during command queue operations."]
    #[must_use]
    #[inline(always)]
    pub const fn cqerr(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Error during command queue operations."]
    #[inline(always)]
    pub const fn set_cqerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
            .field("thr", &self.thr())
            .field("fundfl", &self.fundfl())
            .field("fovfl", &self.fovfl())
            .field("nak", &self.nak())
            .field("iacc", &self.iacc())
            .field("icmd", &self.icmd())
            .field("start", &self.start())
            .field("stop", &self.stop())
            .field("arb", &self.arb())
            .field("dcmp", &self.dcmp())
            .field("derr", &self.derr())
            .field("cqpaused", &self.cqpaused())
            .field("cqupd", &self.cqupd())
            .field("cqerr", &self.cqerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Intstat {{ cmdcmp: {=bool:?}, thr: {=bool:?}, fundfl: {=bool:?}, fovfl: {=bool:?}, nak: {=bool:?}, iacc: {=bool:?}, icmd: {=bool:?}, start: {=bool:?}, stop: {=bool:?}, arb: {=bool:?}, dcmp: {=bool:?}, derr: {=bool:?}, cqpaused: {=bool:?}, cqupd: {=bool:?}, cqerr: {=bool:?} }}" , self . cmdcmp () , self . thr () , self . fundfl () , self . fovfl () , self . nak () , self . iacc () , self . icmd () , self . start () , self . stop () , self . arb () , self . dcmp () , self . derr () , self . cqpaused () , self . cqupd () , self . cqerr ())
    }
}
#[doc = "IOM Debug Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iomdbg(pub u32);
impl Iomdbg {
    #[doc = "Debug Enable. Setting bit will enable the update of data within this register, otherwise it is clock gated for power savings."]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable. Setting bit will enable the update of data within this register, otherwise it is clock gated for power savings."]
    #[inline(always)]
    pub const fn set_dbgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "IOCLK debug clock control. Enable IO_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
    #[must_use]
    #[inline(always)]
    pub const fn ioclkon(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "IOCLK debug clock control. Enable IO_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
    #[inline(always)]
    pub const fn set_ioclkon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "APBCLK debug clock control. Enable APB_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
    #[must_use]
    #[inline(always)]
    pub const fn apbclkon(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "APBCLK debug clock control. Enable APB_CLK to be active when this bit is '1'. Otherwise, the clock is controlled with gating from the logic as needed."]
    #[inline(always)]
    pub const fn set_apbclkon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Debug control for various options. DBGDATA\\[1:0\\] is used to select between different debug data available in the DBG0 and DBG1 registers."]
    #[must_use]
    #[inline(always)]
    pub const fn dbgdata(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "Debug control for various options. DBGDATA\\[1:0\\] is used to select between different debug data available in the DBG0 and DBG1 registers."]
    #[inline(always)]
    pub const fn set_dbgdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for Iomdbg {
    #[inline(always)]
    fn default() -> Iomdbg {
        Iomdbg(0)
    }
}
impl core::fmt::Debug for Iomdbg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iomdbg")
            .field("dbgen", &self.dbgen())
            .field("ioclkon", &self.ioclkon())
            .field("apbclkon", &self.apbclkon())
            .field("dbgdata", &self.dbgdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iomdbg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Iomdbg {{ dbgen: {=bool:?}, ioclkon: {=bool:?}, apbclkon: {=bool:?}, dbgdata: {=u32:?} }}" , self . dbgen () , self . ioclkon () , self . apbclkon () , self . dbgdata ())
    }
}
#[doc = "I2C Master configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mi2ccfg(pub u32);
impl Mi2ccfg {
    #[doc = "Sets the I2C master device address size to either 7b (0) or 10b (1)."]
    #[must_use]
    #[inline(always)]
    pub const fn addrsz(&self) -> super::vals::Addrsz {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Addrsz::from_bits(val as u8)
    }
    #[doc = "Sets the I2C master device address size to either 7b (0) or 10b (1)."]
    #[inline(always)]
    pub const fn set_addrsz(&mut self, val: super::vals::Addrsz) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Direction of data transmit and receive, MSB(0) or LSB(1) first. Default per I2C specification is MSB first. This applies to both read and write data, and read data will be bit."]
    #[must_use]
    #[inline(always)]
    pub const fn i2clsb(&self) -> super::vals::I2clsb {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::I2clsb::from_bits(val as u8)
    }
    #[doc = "Direction of data transmit and receive, MSB(0) or LSB(1) first. Default per I2C specification is MSB first. This applies to both read and write data, and read data will be bit."]
    #[inline(always)]
    pub const fn set_i2clsb(&mut self, val: super::vals::I2clsb) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables multi-master arbitration for the I2C master. If the bus is known to have only a single master, this function can be disabled to save clock cycles on I2C transactions."]
    #[must_use]
    #[inline(always)]
    pub const fn arben(&self) -> super::vals::Arben {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Arben::from_bits(val as u8)
    }
    #[doc = "Enables multi-master arbitration for the I2C master. If the bus is known to have only a single master, this function can be disabled to save clock cycles on I2C transactions."]
    #[inline(always)]
    pub const fn set_arben(&mut self, val: super::vals::Arben) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Delay to enable on the SDA output. Values are 0x0-0x3."]
    #[must_use]
    #[inline(always)]
    pub const fn sdadly(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Delay to enable on the SDA output. Values are 0x0-0x3."]
    #[inline(always)]
    pub const fn set_sdadly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Not used. To reset the module, toggle the SMOD_EN for the module."]
    #[must_use]
    #[inline(always)]
    pub const fn mi2crst(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Not used. To reset the module, toggle the SMOD_EN for the module."]
    #[inline(always)]
    pub const fn set_mi2crst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Number of IOCLK cycles to delay the rising edge of the SCL output en (clock will go low on this edge). Used to allow clock shaping."]
    #[must_use]
    #[inline(always)]
    pub const fn sclendly(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of IOCLK cycles to delay the rising edge of the SCL output en (clock will go low on this edge). Used to allow clock shaping."]
    #[inline(always)]
    pub const fn set_sclendly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Number of IOCLK cycles to delay the SDA output en (all transitions affected). Used to delay data relative to clock."]
    #[must_use]
    #[inline(always)]
    pub const fn sdaendly(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Number of IOCLK cycles to delay the SDA output en (all transitions affected). Used to delay data relative to clock."]
    #[inline(always)]
    pub const fn set_sdaendly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Number of Base clk cycles to wait before sampling the SCL clock to determine if a clock stretch event has occured."]
    #[must_use]
    #[inline(always)]
    pub const fn smpcnt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Number of Base clk cycles to wait before sampling the SCL clock to determine if a clock stretch event has occured."]
    #[inline(always)]
    pub const fn set_smpcnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Disable detection of clock stretch events smaller than 1 cycle."]
    #[must_use]
    #[inline(always)]
    pub const fn strdis(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Disable detection of clock stretch events smaller than 1 cycle."]
    #[inline(always)]
    pub const fn set_strdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Mi2ccfg {
    #[inline(always)]
    fn default() -> Mi2ccfg {
        Mi2ccfg(0)
    }
}
impl core::fmt::Debug for Mi2ccfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mi2ccfg")
            .field("addrsz", &self.addrsz())
            .field("i2clsb", &self.i2clsb())
            .field("arben", &self.arben())
            .field("sdadly", &self.sdadly())
            .field("mi2crst", &self.mi2crst())
            .field("sclendly", &self.sclendly())
            .field("sdaendly", &self.sdaendly())
            .field("smpcnt", &self.smpcnt())
            .field("strdis", &self.strdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mi2ccfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Mi2ccfg {{ addrsz: {:?}, i2clsb: {:?}, arben: {:?}, sdadly: {=u8:?}, mi2crst: {=bool:?}, sclendly: {=u8:?}, sdaendly: {=u8:?}, smpcnt: {=u8:?}, strdis: {=bool:?} }}" , self . addrsz () , self . i2clsb () , self . arben () , self . sdadly () , self . mi2crst () , self . sclendly () , self . sdaendly () , self . smpcnt () , self . strdis ())
    }
}
#[doc = "SPI module master configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mspicfg(pub u32);
impl Mspicfg {
    #[doc = "selects SPI polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn spol(&self) -> super::super::shared::Spol {
        let val = (self.0 >> 0usize) & 0x01;
        super::super::shared::Spol::from_bits(val as u8)
    }
    #[doc = "selects SPI polarity."]
    #[inline(always)]
    pub const fn set_spol(&mut self, val: super::super::shared::Spol) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "selects SPI phase."]
    #[must_use]
    #[inline(always)]
    pub const fn spha(&self) -> super::super::shared::Spha {
        let val = (self.0 >> 1usize) & 0x01;
        super::super::shared::Spha::from_bits(val as u8)
    }
    #[doc = "selects SPI phase."]
    #[inline(always)]
    pub const fn set_spha(&mut self, val: super::super::shared::Spha) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Enables full duplex mode for Master SPI write operations. Data will be captured simultaneously into the read fifo."]
    #[must_use]
    #[inline(always)]
    pub const fn fulldup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enables full duplex mode for Master SPI write operations. Data will be captured simultaneously into the read fifo."]
    #[inline(always)]
    pub const fn set_fulldup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "enables write mode flow control."]
    #[must_use]
    #[inline(always)]
    pub const fn wtfc(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "enables write mode flow control."]
    #[inline(always)]
    pub const fn set_wtfc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "enables read mode flow control."]
    #[must_use]
    #[inline(always)]
    pub const fn rdfc(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "enables read mode flow control."]
    #[inline(always)]
    pub const fn set_rdfc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "inverts MOSI when flow control is enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn mosiinv(&self) -> super::vals::Mosiinv {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Mosiinv::from_bits(val as u8)
    }
    #[doc = "inverts MOSI when flow control is enabled."]
    #[inline(always)]
    pub const fn set_mosiinv(&mut self, val: super::vals::Mosiinv) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "selects the write mode flow control signal."]
    #[must_use]
    #[inline(always)]
    pub const fn wtfcirq(&self) -> super::vals::Wtfcirq {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Wtfcirq::from_bits(val as u8)
    }
    #[doc = "selects the write mode flow control signal."]
    #[inline(always)]
    pub const fn set_wtfcirq(&mut self, val: super::vals::Wtfcirq) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "selects the write flow control signal polarity. The transfers are halted when the selected flow control signal is OPPOSITE polarity of bit. (For example: WTFCPOL = 0 will allow a IRQ=1 to pause transfers)."]
    #[must_use]
    #[inline(always)]
    pub const fn wtfcpol(&self) -> super::vals::Fcpol {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Fcpol::from_bits(val as u8)
    }
    #[doc = "selects the write flow control signal polarity. The transfers are halted when the selected flow control signal is OPPOSITE polarity of bit. (For example: WTFCPOL = 0 will allow a IRQ=1 to pause transfers)."]
    #[inline(always)]
    pub const fn set_wtfcpol(&mut self, val: super::vals::Fcpol) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "selects the read flow control signal polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn rdfcpol(&self) -> super::vals::Fcpol {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Fcpol::from_bits(val as u8)
    }
    #[doc = "selects the read flow control signal polarity."]
    #[inline(always)]
    pub const fn set_rdfcpol(&mut self, val: super::vals::Fcpol) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Selects data transfer as MSB first (0) or LSB first (1) for the data portion of the SPI transaction. The offset bytes are always transmitted MSB first."]
    #[must_use]
    #[inline(always)]
    pub const fn spilsb(&self) -> super::super::shared::Spilsb {
        let val = (self.0 >> 23usize) & 0x01;
        super::super::shared::Spilsb::from_bits(val as u8)
    }
    #[doc = "Selects data transfer as MSB first (0) or LSB first (1) for the data portion of the SPI transaction. The offset bytes are always transmitted MSB first."]
    #[inline(always)]
    pub const fn set_spilsb(&mut self, val: super::super::shared::Spilsb) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Delay tap to use for the input signal (MISO). This gives more hold time on the input data."]
    #[must_use]
    #[inline(always)]
    pub const fn dindly(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Delay tap to use for the input signal (MISO). This gives more hold time on the input data."]
    #[inline(always)]
    pub const fn set_dindly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "Delay tap to use for the output signal (MOSI). This give more hold time on the output data."]
    #[must_use]
    #[inline(always)]
    pub const fn doutdly(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x07;
        val as u8
    }
    #[doc = "Delay tap to use for the output signal (MOSI). This give more hold time on the output data."]
    #[inline(always)]
    pub const fn set_doutdly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 27usize)) | (((val as u32) & 0x07) << 27usize);
    }
    #[doc = "Not used. To reset the module, toggle the SMOD_EN for the module."]
    #[must_use]
    #[inline(always)]
    pub const fn mspirst(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Not used. To reset the module, toggle the SMOD_EN for the module."]
    #[inline(always)]
    pub const fn set_mspirst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
            .field("spol", &self.spol())
            .field("spha", &self.spha())
            .field("fulldup", &self.fulldup())
            .field("wtfc", &self.wtfc())
            .field("rdfc", &self.rdfc())
            .field("mosiinv", &self.mosiinv())
            .field("wtfcirq", &self.wtfcirq())
            .field("wtfcpol", &self.wtfcpol())
            .field("rdfcpol", &self.rdfcpol())
            .field("spilsb", &self.spilsb())
            .field("dindly", &self.dindly())
            .field("doutdly", &self.doutdly())
            .field("mspirst", &self.mspirst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mspicfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Mspicfg {{ spol: {:?}, spha: {:?}, fulldup: {=bool:?}, wtfc: {=bool:?}, rdfc: {=bool:?}, mosiinv: {:?}, wtfcirq: {:?}, wtfcpol: {:?}, rdfcpol: {:?}, spilsb: {:?}, dindly: {=u8:?}, doutdly: {=u8:?}, mspirst: {=bool:?} }}" , self . spol () , self . spha () , self . fulldup () , self . wtfc () , self . rdfc () , self . mosiinv () , self . wtfcirq () , self . wtfcpol () , self . rdfcpol () , self . spilsb () , self . dindly () , self . doutdly () , self . mspirst ())
    }
}
#[doc = "High order 2 bytes of 3 byte offset for IO transaction."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Offsethi(pub u32);
impl Offsethi {
    #[doc = "Holds the high order 2 bytes of the 3 byte addressing/offset field to use with IO commands. The number of offset bytes to use is specified in the command register."]
    #[must_use]
    #[inline(always)]
    pub const fn offsethi(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Holds the high order 2 bytes of the 3 byte addressing/offset field to use with IO commands. The number of offset bytes to use is specified in the command register."]
    #[inline(always)]
    pub const fn set_offsethi(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Offsethi {
    #[inline(always)]
    fn default() -> Offsethi {
        Offsethi(0)
    }
}
impl core::fmt::Debug for Offsethi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Offsethi")
            .field("offsethi", &self.offsethi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Offsethi {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Offsethi {{ offsethi: {=u16:?} }}", self.offsethi())
    }
}
#[doc = "IOM Module Status Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Bit has been deprecated. Please refer to the other error indicators. This will always return 0."]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Bit has been deprecated. Please refer to the other error indicators. This will always return 0."]
    #[inline(always)]
    pub const fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates if the active I/O Command is currently processing a transaction, or command is complete, but the FIFO pointers are still syncronizing internally. This bit will go high at the start of the transaction, and will go low when the command is complete, and the data and pointers within the FIFO have been syncronized."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdact(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if the active I/O Command is currently processing a transaction, or command is complete, but the FIFO pointers are still syncronizing internally. This bit will go high at the start of the transaction, and will go low when the command is complete, and the data and pointers within the FIFO have been syncronized."]
    #[inline(always)]
    pub const fn set_cmdact(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "indicates if the active I/O state machine is IDLE. Note - The state machine could be in idle state due to holdoffs from data availability, or as the command gets propagated into the logic from the registers."]
    #[must_use]
    #[inline(always)]
    pub const fn idlest(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "indicates if the active I/O state machine is IDLE. Note - The state machine could be in idle state due to holdoffs from data availability, or as the command gets propagated into the logic from the registers."]
    #[inline(always)]
    pub const fn set_idlest(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("err", &self.err())
            .field("cmdact", &self.cmdact())
            .field("idlest", &self.idlest())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Status {{ err: {=bool:?}, cmdact: {=bool:?}, idlest: {=bool:?} }}",
            self.err(),
            self.cmdact(),
            self.idlest()
        )
    }
}
#[doc = "Submodule control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Submodctrl(pub u32);
impl Submodctrl {
    #[doc = "Submodule 0 enable (1) or disable (0)."]
    #[must_use]
    #[inline(always)]
    pub const fn smod0en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Submodule 0 enable (1) or disable (0)."]
    #[inline(always)]
    pub const fn set_smod0en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Submodule 0 module type. This is the SPI Master interface."]
    #[must_use]
    #[inline(always)]
    pub const fn smod0type(&self) -> super::vals::Smod0type {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::Smod0type::from_bits(val as u8)
    }
    #[doc = "Submodule 0 module type. This is the SPI Master interface."]
    #[inline(always)]
    pub const fn set_smod0type(&mut self, val: super::vals::Smod0type) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
    #[doc = "Submodule 1 enable (1) or disable (0)."]
    #[must_use]
    #[inline(always)]
    pub const fn smod1en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Submodule 1 enable (1) or disable (0)."]
    #[inline(always)]
    pub const fn set_smod1en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Submodule 0 module type. This is the I2C Master interface."]
    #[must_use]
    #[inline(always)]
    pub const fn smod1type(&self) -> super::vals::Smod1type {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::Smod1type::from_bits(val as u8)
    }
    #[doc = "Submodule 0 module type. This is the I2C Master interface."]
    #[inline(always)]
    pub const fn set_smod1type(&mut self, val: super::vals::Smod1type) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
    }
}
impl Default for Submodctrl {
    #[inline(always)]
    fn default() -> Submodctrl {
        Submodctrl(0)
    }
}
impl core::fmt::Debug for Submodctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Submodctrl")
            .field("smod0en", &self.smod0en())
            .field("smod0type", &self.smod0type())
            .field("smod1en", &self.smod1en())
            .field("smod1type", &self.smod1type())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Submodctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Submodctrl {{ smod0en: {=bool:?}, smod0type: {:?}, smod1en: {=bool:?}, smod1type: {:?} }}" , self . smod0en () , self . smod0type () , self . smod1en () , self . smod1type ())
    }
}
