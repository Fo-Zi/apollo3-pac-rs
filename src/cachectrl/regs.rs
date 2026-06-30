#[doc = "Flash Cache Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cachecfg(pub u32);
impl Cachecfg {
    #[doc = "Enables the flash cache controller and enables power to the cache SRAMs. The ICACHE_ENABLE and DCACHE_ENABLE should be set to enable caching for each type of access."]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the flash cache controller and enables power to the cache SRAMs. The ICACHE_ENABLE and DCACHE_ENABLE should be set to enable caching for each type of access."]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Sets the cache repleacment policy. 0=LRR (least recently replaced), 1=LRU (least recently used). LRR minimizes writes to the TAG SRAM."]
    #[must_use]
    #[inline(always)]
    pub const fn lru(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Sets the cache repleacment policy. 0=LRR (least recently replaced), 1=LRU (least recently used). LRR minimizes writes to the TAG SRAM."]
    #[inline(always)]
    pub const fn set_lru(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable Non-cacheable region 0. See NCR0 registers to define the region."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_nc0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Non-cacheable region 0. See NCR0 registers to define the region."]
    #[inline(always)]
    pub const fn set_enable_nc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable Non-cacheable region 1. See NCR1 registers to define the region."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_nc1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Non-cacheable region 1. See NCR1 registers to define the region."]
    #[inline(always)]
    pub const fn set_enable_nc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Sets the cache configuration."]
    #[must_use]
    #[inline(always)]
    pub const fn config(&self) -> super::vals::Config {
        let val = (self.0 >> 4usize) & 0x0f;
        super::vals::Config::from_bits(val as u8)
    }
    #[doc = "Sets the cache configuration."]
    #[inline(always)]
    pub const fn set_config(&mut self, val: super::vals::Config) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
    }
    #[doc = "Enable Flash Instruction Caching."]
    #[must_use]
    #[inline(always)]
    pub const fn icache_enable(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Flash Instruction Caching."]
    #[inline(always)]
    pub const fn set_icache_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Enable Flash Data Caching."]
    #[must_use]
    #[inline(always)]
    pub const fn dcache_enable(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Flash Data Caching."]
    #[inline(always)]
    pub const fn set_dcache_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable clock gating of cache TAG RAM. Software should enable this bit for optimal power efficiency."]
    #[must_use]
    #[inline(always)]
    pub const fn cache_clkgate(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enable clock gating of cache TAG RAM. Software should enable this bit for optimal power efficiency."]
    #[inline(always)]
    pub const fn set_cache_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable LS (light sleep) of cache RAMs. Software should DISABLE this bit since cache activity is too high to benefit from LS usage."]
    #[must_use]
    #[inline(always)]
    pub const fn cache_ls(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable LS (light sleep) of cache RAMs. Software should DISABLE this bit since cache activity is too high to benefit from LS usage."]
    #[inline(always)]
    pub const fn set_cache_ls(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable aggressive clock gating of entire data array. This bit should be set to 1 for optimal power efficiency."]
    #[must_use]
    #[inline(always)]
    pub const fn data_clkgate(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Enable aggressive clock gating of entire data array. This bit should be set to 1 for optimal power efficiency."]
    #[inline(always)]
    pub const fn set_data_clkgate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Enable Cache Monitoring Stats. Cache monitoring consumes additional power and should only be enabled when profiling code and counters will increment when this bit is set. Counter values will be retained when this is set to 0, allowing software to enable/disable counting for multiple code segments."]
    #[must_use]
    #[inline(always)]
    pub const fn enable_monitor(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Cache Monitoring Stats. Cache monitoring consumes additional power and should only be enabled when profiling code and counters will increment when this bit is set. Counter values will be retained when this is set to 0, allowing software to enable/disable counting for multiple code segments."]
    #[inline(always)]
    pub const fn set_enable_monitor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Cachecfg {
    #[inline(always)]
    fn default() -> Cachecfg {
        Cachecfg(0)
    }
}
impl core::fmt::Debug for Cachecfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cachecfg")
            .field("enable", &self.enable())
            .field("lru", &self.lru())
            .field("enable_nc0", &self.enable_nc0())
            .field("enable_nc1", &self.enable_nc1())
            .field("config", &self.config())
            .field("icache_enable", &self.icache_enable())
            .field("dcache_enable", &self.dcache_enable())
            .field("cache_clkgate", &self.cache_clkgate())
            .field("cache_ls", &self.cache_ls())
            .field("data_clkgate", &self.data_clkgate())
            .field("enable_monitor", &self.enable_monitor())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cachecfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cachecfg {{ enable: {=bool:?}, lru: {=bool:?}, enable_nc0: {=bool:?}, enable_nc1: {=bool:?}, config: {:?}, icache_enable: {=bool:?}, dcache_enable: {=bool:?}, cache_clkgate: {=bool:?}, cache_ls: {=bool:?}, data_clkgate: {=bool:?}, enable_monitor: {=bool:?} }}" , self . enable () , self . lru () , self . enable_nc0 () , self . enable_nc1 () , self . config () , self . icache_enable () , self . dcache_enable () , self . cache_clkgate () , self . cache_ls () , self . data_clkgate () , self . enable_monitor ())
    }
}
#[doc = "Cache Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Writing a 1 to this bitfield invalidates the flash cache contents."]
    #[must_use]
    #[inline(always)]
    pub const fn invalidate(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing a 1 to this bitfield invalidates the flash cache contents."]
    #[inline(always)]
    pub const fn set_invalidate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Reset Cache Statistics. When written to a 1, the cache monitor counters will be cleared. The monitor counters can be reset only when the CACHECFG.ENABLE_MONITOR bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn reset_stat(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Reset Cache Statistics. When written to a 1, the cache monitor counters will be cleared. The monitor counters can be reset only when the CACHECFG.ENABLE_MONITOR bit is set."]
    #[inline(always)]
    pub const fn set_reset_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Cache Ready Status (enabled and not processing an invalidate operation)."]
    #[must_use]
    #[inline(always)]
    pub const fn cache_ready(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Cache Ready Status (enabled and not processing an invalidate operation)."]
    #[inline(always)]
    pub const fn set_cache_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Flash Sleep Mode Status. 1 indicates that flash0 is in sleep mode, 0 indicates flash0 is in normal mode."]
    #[must_use]
    #[inline(always)]
    pub const fn flash0_slm_status(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Flash Sleep Mode Status. 1 indicates that flash0 is in sleep mode, 0 indicates flash0 is in normal mode."]
    #[inline(always)]
    pub const fn set_flash0_slm_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Disable Flash Sleep Mode. Write 1 to wake flash0 from sleep mode (reading the array will also automatically wake it)."]
    #[must_use]
    #[inline(always)]
    pub const fn flash0_slm_disable(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Flash Sleep Mode. Write 1 to wake flash0 from sleep mode (reading the array will also automatically wake it)."]
    #[inline(always)]
    pub const fn set_flash0_slm_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable Flash Sleep Mode. Write to 1 to put flash 0 into sleep mode. NOTE: there is a 5us latency after waking flash until the first access will be returned."]
    #[must_use]
    #[inline(always)]
    pub const fn flash0_slm_enable(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Flash Sleep Mode. Write to 1 to put flash 0 into sleep mode. NOTE: there is a 5us latency after waking flash until the first access will be returned."]
    #[inline(always)]
    pub const fn set_flash0_slm_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Flash Sleep Mode Status. 1 indicates that flash1 is in sleep mode, 0 indicates flash1 is in normal mode."]
    #[must_use]
    #[inline(always)]
    pub const fn flash1_slm_status(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Flash Sleep Mode Status. 1 indicates that flash1 is in sleep mode, 0 indicates flash1 is in normal mode."]
    #[inline(always)]
    pub const fn set_flash1_slm_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Disable Flash Sleep Mode. Write 1 to wake flash1 from sleep mode (reading the array will also automatically wake it)."]
    #[must_use]
    #[inline(always)]
    pub const fn flash1_slm_disable(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Flash Sleep Mode. Write 1 to wake flash1 from sleep mode (reading the array will also automatically wake it)."]
    #[inline(always)]
    pub const fn set_flash1_slm_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable Flash Sleep Mode. Write to 1 to put flash 1 into sleep mode. NOTE: there is a 5us latency after waking flash until the first access will be returned."]
    #[must_use]
    #[inline(always)]
    pub const fn flash1_slm_enable(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enable Flash Sleep Mode. Write to 1 to put flash 1 into sleep mode. NOTE: there is a 5us latency after waking flash until the first access will be returned."]
    #[inline(always)]
    pub const fn set_flash1_slm_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
            .field("invalidate", &self.invalidate())
            .field("reset_stat", &self.reset_stat())
            .field("cache_ready", &self.cache_ready())
            .field("flash0_slm_status", &self.flash0_slm_status())
            .field("flash0_slm_disable", &self.flash0_slm_disable())
            .field("flash0_slm_enable", &self.flash0_slm_enable())
            .field("flash1_slm_status", &self.flash1_slm_status())
            .field("flash1_slm_disable", &self.flash1_slm_disable())
            .field("flash1_slm_enable", &self.flash1_slm_enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ invalidate: {=bool:?}, reset_stat: {=bool:?}, cache_ready: {=bool:?}, flash0_slm_status: {=bool:?}, flash0_slm_disable: {=bool:?}, flash0_slm_enable: {=bool:?}, flash1_slm_status: {=bool:?}, flash1_slm_disable: {=bool:?}, flash1_slm_enable: {=bool:?} }}" , self . invalidate () , self . reset_stat () , self . cache_ready () , self . flash0_slm_status () , self . flash0_slm_disable () , self . flash0_slm_enable () , self . flash1_slm_status () , self . flash1_slm_disable () , self . flash1_slm_enable ())
    }
}
#[doc = "Flash Control Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flashcfg(pub u32);
impl Flashcfg {
    #[doc = "Sets read waitstates for normal (fast) operation. A value of 1 is recommended."]
    #[must_use]
    #[inline(always)]
    pub const fn rd_wait(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Sets read waitstates for normal (fast) operation. A value of 1 is recommended."]
    #[inline(always)]
    pub const fn set_rd_wait(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Sets SE delay (flash address setup). A value of 5 is recommended."]
    #[must_use]
    #[inline(always)]
    pub const fn sedelay(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Sets SE delay (flash address setup). A value of 5 is recommended."]
    #[inline(always)]
    pub const fn set_sedelay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Sets flash waitstates when in LPM Mode 2 (RD_WAIT in LPM mode 2 only)."]
    #[must_use]
    #[inline(always)]
    pub const fn lpm_rd_wait(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Sets flash waitstates when in LPM Mode 2 (RD_WAIT in LPM mode 2 only)."]
    #[inline(always)]
    pub const fn set_lpm_rd_wait(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Controls flash low power modes (control of LPM pin)."]
    #[must_use]
    #[inline(always)]
    pub const fn lpmmode(&self) -> super::vals::Lpmmode {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Lpmmode::from_bits(val as u8)
    }
    #[doc = "Controls flash low power modes (control of LPM pin)."]
    #[inline(always)]
    pub const fn set_lpmmode(&mut self, val: super::vals::Lpmmode) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
}
impl Default for Flashcfg {
    #[inline(always)]
    fn default() -> Flashcfg {
        Flashcfg(0)
    }
}
impl core::fmt::Debug for Flashcfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flashcfg")
            .field("rd_wait", &self.rd_wait())
            .field("sedelay", &self.sedelay())
            .field("lpm_rd_wait", &self.lpm_rd_wait())
            .field("lpmmode", &self.lpmmode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flashcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Flashcfg {{ rd_wait: {=u8:?}, sedelay: {=u8:?}, lpm_rd_wait: {=u8:?}, lpmmode: {:?} }}" , self . rd_wait () , self . sedelay () , self . lpm_rd_wait () , self . lpmmode ())
    }
}
#[doc = "Flash Cache Noncachable Region 0 End."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ncr0end(pub u32);
impl Ncr0end {
    #[doc = "End address for non-cacheable region 0."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "End address for non-cacheable region 0."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 4usize)) | (((val as u32) & 0x007f_ffff) << 4usize);
    }
}
impl Default for Ncr0end {
    #[inline(always)]
    fn default() -> Ncr0end {
        Ncr0end(0)
    }
}
impl core::fmt::Debug for Ncr0end {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ncr0end")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ncr0end {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ncr0end {{ addr: {=u32:?} }}", self.addr())
    }
}
#[doc = "Flash Cache Noncachable Region 0 Start."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ncr0start(pub u32);
impl Ncr0start {
    #[doc = "Start address for non-cacheable region 0."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Start address for non-cacheable region 0."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 4usize)) | (((val as u32) & 0x007f_ffff) << 4usize);
    }
}
impl Default for Ncr0start {
    #[inline(always)]
    fn default() -> Ncr0start {
        Ncr0start(0)
    }
}
impl core::fmt::Debug for Ncr0start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ncr0start")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ncr0start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ncr0start {{ addr: {=u32:?} }}", self.addr())
    }
}
#[doc = "Flash Cache Noncachable Region 1 End."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ncr1end(pub u32);
impl Ncr1end {
    #[doc = "End address for non-cacheable region 1."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "End address for non-cacheable region 1."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 4usize)) | (((val as u32) & 0x007f_ffff) << 4usize);
    }
}
impl Default for Ncr1end {
    #[inline(always)]
    fn default() -> Ncr1end {
        Ncr1end(0)
    }
}
impl core::fmt::Debug for Ncr1end {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ncr1end")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ncr1end {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ncr1end {{ addr: {=u32:?} }}", self.addr())
    }
}
#[doc = "Flash Cache Noncachable Region 1 Start."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ncr1start(pub u32);
impl Ncr1start {
    #[doc = "Start address for non-cacheable region 1."]
    #[must_use]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x007f_ffff;
        val as u32
    }
    #[doc = "Start address for non-cacheable region 1."]
    #[inline(always)]
    pub const fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x007f_ffff << 4usize)) | (((val as u32) & 0x007f_ffff) << 4usize);
    }
}
impl Default for Ncr1start {
    #[inline(always)]
    fn default() -> Ncr1start {
        Ncr1start(0)
    }
}
impl core::fmt::Debug for Ncr1start {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ncr1start")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ncr1start {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Ncr1start {{ addr: {=u32:?} }}", self.addr())
    }
}
