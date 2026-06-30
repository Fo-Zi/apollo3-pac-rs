#[doc = "BLE Core Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blecfg(pub u32);
impl Blecfg {
    #[doc = "Enable the power state machine for automatic sequencing and control of power states of the BLE Core module."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrsmen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the power state machine for automatic sequencing and control of power states of the BLE Core module."]
    #[inline(always)]
    pub const fn set_pwrsmen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Reset line to the BLE Core. This will reset the BLE core when asserted ('0') and must be written to '1' prior to performing any BTLE related operations to the core."]
    #[must_use]
    #[inline(always)]
    pub const fn blerstn(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Reset line to the BLE Core. This will reset the BLE core when asserted ('0') and must be written to '1' prior to performing any BTLE related operations to the core."]
    #[inline(always)]
    pub const fn set_blerstn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "WAKE signal override. Controls the source of the WAKE signal to the BLE Core."]
    #[must_use]
    #[inline(always)]
    pub const fn wakeupctl(&self) -> super::vals::Wakeupctl {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Wakeupctl::from_bits(val as u8)
    }
    #[doc = "WAKE signal override. Controls the source of the WAKE signal to the BLE Core."]
    #[inline(always)]
    pub const fn set_wakeupctl(&mut self, val: super::vals::Wakeupctl) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "DCDCFLG signal override. The value of this field will be sent to the BLE Core when the PWRSM is off. Otherwise, the value is supplied from internal logic."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdcflgctl(&self) -> super::vals::Dcdcflgctl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Dcdcflgctl::from_bits(val as u8)
    }
    #[doc = "DCDCFLG signal override. The value of this field will be sent to the BLE Core when the PWRSM is off. Otherwise, the value is supplied from internal logic."]
    #[inline(always)]
    pub const fn set_dcdcflgctl(&mut self, val: super::vals::Dcdcflgctl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "BLEH power on request override. The value of this field will be sent to the BLE Core when the PWRSM is off. Otherwise, the value is supplied from internal logic."]
    #[must_use]
    #[inline(always)]
    pub const fn blehreqctl(&self) -> super::vals::Blehreqctl {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Blehreqctl::from_bits(val as u8)
    }
    #[doc = "BLEH power on request override. The value of this field will be sent to the BLE Core when the PWRSM is off. Otherwise, the value is supplied from internal logic."]
    #[inline(always)]
    pub const fn set_blehreqctl(&mut self, val: super::vals::Blehreqctl) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Debug control of BLEIF power state machine. Allows transition into the active state in the BLEIF state without waiting for dcdc req from BLE Core."]
    #[must_use]
    #[inline(always)]
    pub const fn wt4actoff(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Debug control of BLEIF power state machine. Allows transition into the active state in the BLEIF state without waiting for dcdc req from BLE Core."]
    #[inline(always)]
    pub const fn set_wt4actoff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Force power state machine to go to the sleep state. Intended for debug only. Has no effect on the actual BLE Core state, only the state of the BLEIF interface state machine."]
    #[must_use]
    #[inline(always)]
    pub const fn mcufrcslp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Force power state machine to go to the sleep state. Intended for debug only. Has no effect on the actual BLE Core state, only the state of the BLEIF interface state machine."]
    #[inline(always)]
    pub const fn set_mcufrcslp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Force the clock in the BLEIF to be always running."]
    #[must_use]
    #[inline(always)]
    pub const fn frcclk(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Force the clock in the BLEIF to be always running."]
    #[inline(always)]
    pub const fn set_frcclk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Set to prevent the BLE power control module from waking up the BLE Core after going into power down. To be used for graceful shutdown, set by software prior to powering off and will allow assertion of reset from sleep state."]
    #[must_use]
    #[inline(always)]
    pub const fn stayasleep(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Set to prevent the BLE power control module from waking up the BLE Core after going into power down. To be used for graceful shutdown, set by software prior to powering off and will allow assertion of reset from sleep state."]
    #[inline(always)]
    pub const fn set_stayasleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Configuration of BLEH isolation control for power related signals."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrisoctl(&self) -> super::vals::Pwrisoctl {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Pwrisoctl::from_bits(val as u8)
    }
    #[doc = "Configuration of BLEH isolation control for power related signals."]
    #[inline(always)]
    pub const fn set_pwrisoctl(&mut self, val: super::vals::Pwrisoctl) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Configuration of BLEH isolation controls for SPI related signals."]
    #[must_use]
    #[inline(always)]
    pub const fn spiisoctl(&self) -> super::vals::Spiisoctl {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Spiisoctl::from_bits(val as u8)
    }
    #[doc = "Configuration of BLEH isolation controls for SPI related signals."]
    #[inline(always)]
    pub const fn set_spiisoctl(&mut self, val: super::vals::Spiisoctl) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for Blecfg {
    #[inline(always)]
    fn default() -> Blecfg {
        Blecfg(0)
    }
}
impl core::fmt::Debug for Blecfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Blecfg")
            .field("pwrsmen", &self.pwrsmen())
            .field("blerstn", &self.blerstn())
            .field("wakeupctl", &self.wakeupctl())
            .field("dcdcflgctl", &self.dcdcflgctl())
            .field("blehreqctl", &self.blehreqctl())
            .field("wt4actoff", &self.wt4actoff())
            .field("mcufrcslp", &self.mcufrcslp())
            .field("frcclk", &self.frcclk())
            .field("stayasleep", &self.stayasleep())
            .field("pwrisoctl", &self.pwrisoctl())
            .field("spiisoctl", &self.spiisoctl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Blecfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Blecfg {{ pwrsmen: {=bool:?}, blerstn: {=bool:?}, wakeupctl: {:?}, dcdcflgctl: {:?}, blehreqctl: {:?}, wt4actoff: {=bool:?}, mcufrcslp: {=bool:?}, frcclk: {=bool:?}, stayasleep: {=bool:?}, pwrisoctl: {:?}, spiisoctl: {:?} }}" , self . pwrsmen () , self . blerstn () , self . wakeupctl () , self . dcdcflgctl () , self . blehreqctl () , self . wt4actoff () , self . mcufrcslp () , self . frcclk () , self . stayasleep () , self . pwrisoctl () , self . spiisoctl ())
    }
}
#[doc = "BLEIF Master Debug Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bledbg(pub u32);
impl Bledbg {
    #[doc = "Debug Enable. Setting this bit will enable the update of data within this register, otherwise it is clock gated for power savings."]
    #[must_use]
    #[inline(always)]
    pub const fn dbgen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Debug Enable. Setting this bit will enable the update of data within this register, otherwise it is clock gated for power savings."]
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
    #[doc = "Debug data."]
    #[must_use]
    #[inline(always)]
    pub const fn dbgdata(&self) -> u32 {
        let val = (self.0 >> 3usize) & 0x1fff_ffff;
        val as u32
    }
    #[doc = "Debug data."]
    #[inline(always)]
    pub const fn set_dbgdata(&mut self, val: u32) {
        self.0 = (self.0 & !(0x1fff_ffff << 3usize)) | (((val as u32) & 0x1fff_ffff) << 3usize);
    }
}
impl Default for Bledbg {
    #[inline(always)]
    fn default() -> Bledbg {
        Bledbg(0)
    }
}
impl core::fmt::Debug for Bledbg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bledbg")
            .field("dbgen", &self.dbgen())
            .field("ioclkon", &self.ioclkon())
            .field("apbclkon", &self.apbclkon())
            .field("dbgdata", &self.dbgdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bledbg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bledbg {{ dbgen: {=bool:?}, ioclkon: {=bool:?}, apbclkon: {=bool:?}, dbgdata: {=u32:?} }}" , self . dbgen () , self . ioclkon () , self . apbclkon () , self . dbgdata ())
    }
}
#[doc = "BLE Core status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bstatus(pub u32);
impl Bstatus {
    #[doc = "State of the BLE Core logic."]
    #[must_use]
    #[inline(always)]
    pub const fn b2mstate(&self) -> super::vals::B2mstate {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::B2mstate::from_bits(val as u8)
    }
    #[doc = "State of the BLE Core logic."]
    #[inline(always)]
    pub const fn set_b2mstate(&mut self, val: super::vals::B2mstate) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Value of the SPISTATUS signal from the BLE Core. The signal is asserted when the BLE Core is able to accept write data via the SPI interface. Data should be transmitted to the BLE core only when this signal is 1. The hardware will automatically wait for this signal prior to performing a write operation if flow control is active."]
    #[must_use]
    #[inline(always)]
    pub const fn spistatus(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Value of the SPISTATUS signal from the BLE Core. The signal is asserted when the BLE Core is able to accept write data via the SPI interface. Data should be transmitted to the BLE core only when this signal is 1. The hardware will automatically wait for this signal prior to performing a write operation if flow control is active."]
    #[inline(always)]
    pub const fn set_spistatus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Value of the DCDCREQ signal from the BLE Core. The DCDCREQ signal is sent from the core to the BLEIF module when the BLE core requires BLEH power to be active. When activated, this is indicated by DCDCFLAG going to 1."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdcreq(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Value of the DCDCREQ signal from the BLE Core. The DCDCREQ signal is sent from the core to the BLEIF module when the BLE core requires BLEH power to be active. When activated, this is indicated by DCDCFLAG going to 1."]
    #[inline(always)]
    pub const fn set_dcdcreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Value of the DCDCFLAG signal to the BLE Core. The DCDCFLAG is a signal to the BLE Core indicating that the BLEH ppower is active."]
    #[must_use]
    #[inline(always)]
    pub const fn dcdcflag(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Value of the DCDCFLAG signal to the BLE Core. The DCDCFLAG is a signal to the BLE Core indicating that the BLEH ppower is active."]
    #[inline(always)]
    pub const fn set_dcdcflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Value of the WAKEUP signal to the BLE Core . The WAKEUP signals is sent from the BLEIF to the BLECORE to request the BLE Core transition from sleep state to active state."]
    #[must_use]
    #[inline(always)]
    pub const fn wakeup(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Value of the WAKEUP signal to the BLE Core . The WAKEUP signals is sent from the BLEIF to the BLECORE to request the BLE Core transition from sleep state to active state."]
    #[inline(always)]
    pub const fn set_wakeup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Status of the BLEIRQ signal from the BLE Core. A value of 1 idicates that read data is available in the core and a read operation needs to be performed."]
    #[must_use]
    #[inline(always)]
    pub const fn bleirq(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Status of the BLEIRQ signal from the BLE Core. A value of 1 idicates that read data is available in the core and a read operation needs to be performed."]
    #[inline(always)]
    pub const fn set_bleirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Current status of the power state machine."]
    #[must_use]
    #[inline(always)]
    pub const fn pwrst(&self) -> super::vals::Pwrst {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Pwrst::from_bits(val as u8)
    }
    #[doc = "Current status of the power state machine."]
    #[inline(always)]
    pub const fn set_pwrst(&mut self, val: super::vals::Pwrst) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Value of the BLEHACK signal from the power control unit. If the signal is '1', the BLEH power is active and ready for use."]
    #[must_use]
    #[inline(always)]
    pub const fn blehack(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Value of the BLEHACK signal from the power control unit. If the signal is '1', the BLEH power is active and ready for use."]
    #[inline(always)]
    pub const fn set_blehack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Value of the BLEHREQ signal to the power control unit. The BLEHREQ signal is sent from the BLEIF module to the power control module to request the BLEH power up. When the BLEHACK signal is asserted, BLEH power is stable and ready for use."]
    #[must_use]
    #[inline(always)]
    pub const fn blehreq(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Value of the BLEHREQ signal to the power control unit. The BLEHREQ signal is sent from the BLEIF module to the power control module to request the BLEH power up. When the BLEHACK signal is asserted, BLEH power is stable and ready for use."]
    #[inline(always)]
    pub const fn set_blehreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Bstatus {
    #[inline(always)]
    fn default() -> Bstatus {
        Bstatus(0)
    }
}
impl core::fmt::Debug for Bstatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bstatus")
            .field("b2mstate", &self.b2mstate())
            .field("spistatus", &self.spistatus())
            .field("dcdcreq", &self.dcdcreq())
            .field("dcdcflag", &self.dcdcflag())
            .field("wakeup", &self.wakeup())
            .field("bleirq", &self.bleirq())
            .field("pwrst", &self.pwrst())
            .field("blehack", &self.blehack())
            .field("blehreq", &self.blehreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bstatus {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Bstatus {{ b2mstate: {:?}, spistatus: {=bool:?}, dcdcreq: {=bool:?}, dcdcflag: {=bool:?}, wakeup: {=bool:?}, bleirq: {=bool:?}, pwrst: {:?}, blehack: {=bool:?}, blehreq: {=bool:?} }}" , self . b2mstate () , self . spistatus () , self . dcdcreq () , self . dcdcflag () , self . wakeup () , self . bleirq () , self . pwrst () , self . blehack () , self . blehreq ())
    }
}
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
    pub const fn fsel(&self) -> super::vals::Fsel {
        let val = (self.0 >> 8usize) & 0x07;
        super::vals::Fsel::from_bits(val as u8)
    }
    #[doc = "Select the input clock frequency."]
    #[inline(always)]
    pub const fn set_fsel(&mut self, val: super::vals::Fsel) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
    }
    #[doc = "Enable for the 32Khz clock to the BLE module."]
    #[must_use]
    #[inline(always)]
    pub const fn clk32ken(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable for the 32Khz clock to the BLE module."]
    #[inline(always)]
    pub const fn set_clk32ken(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enable of the divide by 3 of the source IOCLK."]
    #[must_use]
    #[inline(always)]
    pub const fn div3(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enable of the divide by 3 of the source IOCLK."]
    #[inline(always)]
    pub const fn set_div3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
            .field("clk32ken", &self.clk32ken())
            .field("div3", &self.div3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkcfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Clkcfg {{ ioclken: {=bool:?}, fsel: {:?}, clk32ken: {=bool:?}, div3: {=bool:?} }}",
            self.ioclken(),
            self.fsel(),
            self.clk32ken(),
            self.div3()
        )
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
    #[doc = "Command Specific selection information."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdsel(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Command Specific selection information."]
    #[inline(always)]
    pub const fn set_cmdsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "This register holds the low order byte of offset to be used in the transaction. The number of offset bytes to use is set with bits 1:0 of the command. Offset bytes are transferred starting from the highest byte first."]
    #[must_use]
    #[inline(always)]
    pub const fn offsetlo(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "This register holds the low order byte of offset to be used in the transaction. The number of offset bytes to use is set with bits 1:0 of the command. Offset bytes are transferred starting from the highest byte first."]
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
#[doc = "Command Repeat Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmdrpt(pub u32);
impl Cmdrpt {
    #[doc = "Count of number of times to repeat the next command."]
    #[must_use]
    #[inline(always)]
    pub const fn cmdrpt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Count of number of times to repeat the next command."]
    #[inline(always)]
    pub const fn set_cmdrpt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Cmdrpt {
    #[inline(always)]
    fn default() -> Cmdrpt {
        Cmdrpt(0)
    }
}
impl core::fmt::Debug for Cmdrpt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cmdrpt")
            .field("cmdrpt", &self.cmdrpt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cmdrpt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cmdrpt {{ cmdrpt: {=u8:?} }}", self.cmdrpt())
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
    pub const fn cmdstat(&self) -> super::vals::Cmdstat {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::Cmdstat::from_bits(val as u8)
    }
    #[doc = "The current status of the command execution."]
    #[inline(always)]
    pub const fn set_cmdstat(&mut self, val: super::vals::Cmdstat) {
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
    #[doc = "Provides for a per-bit mask of the flags used to invoke an interrupt. A '1' in the bit position will enable the pause event to trigger the interrupt, if the CQWT_int interrupt is enabled. Bits definitions are the same as CQPAUSE."]
    #[must_use]
    #[inline(always)]
    pub const fn cqirqmask(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Provides for a per-bit mask of the flags used to invoke an interrupt. A '1' in the bit position will enable the pause event to trigger the interrupt, if the CQWT_int interrupt is enabled. Bits definitions are the same as CQPAUSE."]
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
    pub const fn dmadir(&self) -> super::vals::Dmadir {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dmadir::from_bits(val as u8)
    }
    #[doc = "Direction."]
    #[inline(always)]
    pub const fn set_dmadir(&mut self, val: super::vals::Dmadir) {
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
    #[doc = "DMA Transfer Complete. This signals the end of the DMA operation. This bit can be cleared by writing to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn dmacpl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Transfer Complete. This signals the end of the DMA operation. This bit can be cleared by writing to 0."]
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
    #[doc = "Trigger DMA upon command complete. Enables the trigger of the DMA when a command is completed. When this event is triggered, the number of words transferred will be the lesser of the remaining TOTCOUNT bytes, or the number of bytes in the FIFO when the command completed. If this is disabled, and the number of bytes in the FIFO is equal or greater than the TOTCOUNT bytes, a transfer of TOTCOUNT bytes will be done to ensure read data is stored when the DMA is completed."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmdcmpen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger DMA upon command complete. Enables the trigger of the DMA when a command is completed. When this event is triggered, the number of words transferred will be the lesser of the remaining TOTCOUNT bytes, or the number of bytes in the FIFO when the command completed. If this is disabled, and the number of bytes in the FIFO is equal or greater than the TOTCOUNT bytes, a transfer of TOTCOUNT bytes will be done to ensure read data is stored when the DMA is completed."]
    #[inline(always)]
    pub const fn set_dcmdcmpen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Trigger DMA upon THR level reached. For M2P DMA operations (IOM writes), the trigger will assert when the write FIFO has (WTHR/4) number of words free in the write FIFO, and will transfer (WTHR/4) number of words or, if the number of words left to transfer is less than the WTHR value, will transfer the remaining byte count. For P2M DMA operations, the trigger will assert when the read FIFO has (RTHR/4) words available in the read FIFO, and will transfer (RTHR/4) words to SRAM. This trigger will NOT assert when the transaction completes and there are less than RTHR bytes left in the fifo, since the RTHR has not been reached. In this case, enabling the CMDCMP trigger will transfer the remaining data from the commmand. If the CMDCMP trigger is not enabled, the module will initiate a transfer when the amount of data in the FIFO is equal to or greater than the remaining data in the DMA. In cases where one DMA setup covers multiple commands, this will only occur at the end of the last transaction when the DMA is near complete."]
    #[must_use]
    #[inline(always)]
    pub const fn dthren(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger DMA upon THR level reached. For M2P DMA operations (IOM writes), the trigger will assert when the write FIFO has (WTHR/4) number of words free in the write FIFO, and will transfer (WTHR/4) number of words or, if the number of words left to transfer is less than the WTHR value, will transfer the remaining byte count. For P2M DMA operations, the trigger will assert when the read FIFO has (RTHR/4) words available in the read FIFO, and will transfer (RTHR/4) words to SRAM. This trigger will NOT assert when the transaction completes and there are less than RTHR bytes left in the fifo, since the RTHR has not been reached. In this case, enabling the CMDCMP trigger will transfer the remaining data from the commmand. If the CMDCMP trigger is not enabled, the module will initiate a transfer when the amount of data in the FIFO is equal to or greater than the remaining data in the DMA. In cases where one DMA setup covers multiple commands, this will only occur at the end of the last transaction when the DMA is near complete."]
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
    #[doc = "Read FIFO Underflow interrupt. Asserted when a pop operation is done to a empty read FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn fundfl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Read FIFO Underflow interrupt. Asserted when a pop operation is done to a empty read FIFO."]
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
    #[doc = "B2M State change interrupt. Asserted on any change in the B2M_STATE signal from the BLE Core."]
    #[must_use]
    #[inline(always)]
    pub const fn b2mst(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "B2M State change interrupt. Asserted on any change in the B2M_STATE signal from the BLE Core."]
    #[inline(always)]
    pub const fn set_b2mst(&mut self, val: bool) {
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
    #[doc = "BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is asserted, indicating the availability of read data from the BLE Core."]
    #[must_use]
    #[inline(always)]
    pub const fn blecirq(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is asserted, indicating the availability of read data from the BLE Core."]
    #[inline(always)]
    pub const fn set_blecirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "BLE Core SPI Status interrupt. Asserted when the SPI_STATUS signal from the BLE Core is asserted, indicating that SPI writes can be done to the BLE Core. Transfers to the BLE Core should only be done when this signal is high."]
    #[must_use]
    #[inline(always)]
    pub const fn blecsstat(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "BLE Core SPI Status interrupt. Asserted when the SPI_STATUS signal from the BLE Core is asserted, indicating that SPI writes can be done to the BLE Core. Transfers to the BLE Core should only be done when this signal is high."]
    #[inline(always)]
    pub const fn set_blecsstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state."]
    #[inline(always)]
    pub const fn set_dcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
    #[must_use]
    #[inline(always)]
    pub const fn derr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
    #[inline(always)]
    pub const fn set_derr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
    #[must_use]
    #[inline(always)]
    pub const fn cqpaused(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
    #[inline(always)]
    pub const fn set_cqpaused(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Command queue write operation executed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
    #[must_use]
    #[inline(always)]
    pub const fn cqupd(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue write operation executed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
    #[inline(always)]
    pub const fn set_cqupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Command queue error during processing. When an error occurs, the system will stop processing and halt operations to allow software to take recovery actions."]
    #[must_use]
    #[inline(always)]
    pub const fn cqerr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue error during processing. When an error occurs, the system will stop processing and halt operations to allow software to take recovery actions."]
    #[inline(always)]
    pub const fn set_cqerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "The B2M_STATE from the BLE Core transitioned into the sleep state."]
    #[must_use]
    #[inline(always)]
    pub const fn b2msleep(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "The B2M_STATE from the BLE Core transitioned into the sleep state."]
    #[inline(always)]
    pub const fn set_b2msleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Revision A: The B2M_STATE from the BLE Core transitioned into the active state Revision B: Falling BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is de-asserted (1 -> 0)."]
    #[must_use]
    #[inline(always)]
    pub const fn b2mactive(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Revision A: The B2M_STATE from the BLE Core transitioned into the active state Revision B: Falling BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is de-asserted (1 -> 0)."]
    #[inline(always)]
    pub const fn set_b2mactive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Revision A: The B2M_STATE from the BLE Core transitioned into shutdown state Revision B: Falling BLE Core Status signal. Asserted when the BLE_STATUS signal from the BLE Core is de-asserted (1 -> 0)."]
    #[must_use]
    #[inline(always)]
    pub const fn b2mshutdn(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Revision A: The B2M_STATE from the BLE Core transitioned into shutdown state Revision B: Falling BLE Core Status signal. Asserted when the BLE_STATUS signal from the BLE Core is de-asserted (1 -> 0)."]
    #[inline(always)]
    pub const fn set_b2mshutdn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
            .field("b2mst", &self.b2mst())
            .field("iacc", &self.iacc())
            .field("icmd", &self.icmd())
            .field("blecirq", &self.blecirq())
            .field("blecsstat", &self.blecsstat())
            .field("dcmp", &self.dcmp())
            .field("derr", &self.derr())
            .field("cqpaused", &self.cqpaused())
            .field("cqupd", &self.cqupd())
            .field("cqerr", &self.cqerr())
            .field("b2msleep", &self.b2msleep())
            .field("b2mactive", &self.b2mactive())
            .field("b2mshutdn", &self.b2mshutdn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intclr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Intclr {{ cmdcmp: {=bool:?}, thr: {=bool:?}, fundfl: {=bool:?}, fovfl: {=bool:?}, b2mst: {=bool:?}, iacc: {=bool:?}, icmd: {=bool:?}, blecirq: {=bool:?}, blecsstat: {=bool:?}, dcmp: {=bool:?}, derr: {=bool:?}, cqpaused: {=bool:?}, cqupd: {=bool:?}, cqerr: {=bool:?}, b2msleep: {=bool:?}, b2mactive: {=bool:?}, b2mshutdn: {=bool:?} }}" , self . cmdcmp () , self . thr () , self . fundfl () , self . fovfl () , self . b2mst () , self . iacc () , self . icmd () , self . blecirq () , self . blecsstat () , self . dcmp () , self . derr () , self . cqpaused () , self . cqupd () , self . cqerr () , self . b2msleep () , self . b2mactive () , self . b2mshutdn ())
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
    #[doc = "Read FIFO Underflow interrupt. Asserted when a pop operation is done to a empty read FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn fundfl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Read FIFO Underflow interrupt. Asserted when a pop operation is done to a empty read FIFO."]
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
    #[doc = "B2M State change interrupt. Asserted on any change in the B2M_STATE signal from the BLE Core."]
    #[must_use]
    #[inline(always)]
    pub const fn b2mst(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "B2M State change interrupt. Asserted on any change in the B2M_STATE signal from the BLE Core."]
    #[inline(always)]
    pub const fn set_b2mst(&mut self, val: bool) {
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
    #[doc = "BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is asserted, indicating the availability of read data from the BLE Core."]
    #[must_use]
    #[inline(always)]
    pub const fn blecirq(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is asserted, indicating the availability of read data from the BLE Core."]
    #[inline(always)]
    pub const fn set_blecirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "BLE Core SPI Status interrupt. Asserted when the SPI_STATUS signal from the BLE Core is asserted, indicating that SPI writes can be done to the BLE Core. Transfers to the BLE Core should only be done when this signal is high."]
    #[must_use]
    #[inline(always)]
    pub const fn blecsstat(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "BLE Core SPI Status interrupt. Asserted when the SPI_STATUS signal from the BLE Core is asserted, indicating that SPI writes can be done to the BLE Core. Transfers to the BLE Core should only be done when this signal is high."]
    #[inline(always)]
    pub const fn set_blecsstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state."]
    #[inline(always)]
    pub const fn set_dcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
    #[must_use]
    #[inline(always)]
    pub const fn derr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
    #[inline(always)]
    pub const fn set_derr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
    #[must_use]
    #[inline(always)]
    pub const fn cqpaused(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
    #[inline(always)]
    pub const fn set_cqpaused(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Command queue write operation executed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
    #[must_use]
    #[inline(always)]
    pub const fn cqupd(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue write operation executed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
    #[inline(always)]
    pub const fn set_cqupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Command queue error during processing. When an error occurs, the system will stop processing and halt operations to allow software to take recovery actions."]
    #[must_use]
    #[inline(always)]
    pub const fn cqerr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue error during processing. When an error occurs, the system will stop processing and halt operations to allow software to take recovery actions."]
    #[inline(always)]
    pub const fn set_cqerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "The B2M_STATE from the BLE Core transitioned into the sleep state."]
    #[must_use]
    #[inline(always)]
    pub const fn b2msleep(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "The B2M_STATE from the BLE Core transitioned into the sleep state."]
    #[inline(always)]
    pub const fn set_b2msleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Revision A: The B2M_STATE from the BLE Core transitioned into the active state Revision B: Falling BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is de-asserted (1 -> 0)."]
    #[must_use]
    #[inline(always)]
    pub const fn b2mactive(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Revision A: The B2M_STATE from the BLE Core transitioned into the active state Revision B: Falling BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is de-asserted (1 -> 0)."]
    #[inline(always)]
    pub const fn set_b2mactive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Revision A: The B2M_STATE from the BLE Core transitioned into shutdown state Revision B: Falling BLE Core Status signal. Asserted when the BLE_STATUS signal from the BLE Core is de-asserted (1 -> 0)."]
    #[must_use]
    #[inline(always)]
    pub const fn b2mshutdn(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Revision A: The B2M_STATE from the BLE Core transitioned into shutdown state Revision B: Falling BLE Core Status signal. Asserted when the BLE_STATUS signal from the BLE Core is de-asserted (1 -> 0)."]
    #[inline(always)]
    pub const fn set_b2mshutdn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
            .field("b2mst", &self.b2mst())
            .field("iacc", &self.iacc())
            .field("icmd", &self.icmd())
            .field("blecirq", &self.blecirq())
            .field("blecsstat", &self.blecsstat())
            .field("dcmp", &self.dcmp())
            .field("derr", &self.derr())
            .field("cqpaused", &self.cqpaused())
            .field("cqupd", &self.cqupd())
            .field("cqerr", &self.cqerr())
            .field("b2msleep", &self.b2msleep())
            .field("b2mactive", &self.b2mactive())
            .field("b2mshutdn", &self.b2mshutdn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Inten {{ cmdcmp: {=bool:?}, thr: {=bool:?}, fundfl: {=bool:?}, fovfl: {=bool:?}, b2mst: {=bool:?}, iacc: {=bool:?}, icmd: {=bool:?}, blecirq: {=bool:?}, blecsstat: {=bool:?}, dcmp: {=bool:?}, derr: {=bool:?}, cqpaused: {=bool:?}, cqupd: {=bool:?}, cqerr: {=bool:?}, b2msleep: {=bool:?}, b2mactive: {=bool:?}, b2mshutdn: {=bool:?} }}" , self . cmdcmp () , self . thr () , self . fundfl () , self . fovfl () , self . b2mst () , self . iacc () , self . icmd () , self . blecirq () , self . blecsstat () , self . dcmp () , self . derr () , self . cqpaused () , self . cqupd () , self . cqerr () , self . b2msleep () , self . b2mactive () , self . b2mshutdn ())
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
    #[doc = "Read FIFO Underflow interrupt. Asserted when a pop operation is done to a empty read FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn fundfl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Read FIFO Underflow interrupt. Asserted when a pop operation is done to a empty read FIFO."]
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
    #[doc = "B2M State change interrupt. Asserted on any change in the B2M_STATE signal from the BLE Core."]
    #[must_use]
    #[inline(always)]
    pub const fn b2mst(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "B2M State change interrupt. Asserted on any change in the B2M_STATE signal from the BLE Core."]
    #[inline(always)]
    pub const fn set_b2mst(&mut self, val: bool) {
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
    #[doc = "BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is asserted, indicating the availability of read data from the BLE Core."]
    #[must_use]
    #[inline(always)]
    pub const fn blecirq(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is asserted, indicating the availability of read data from the BLE Core."]
    #[inline(always)]
    pub const fn set_blecirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "BLE Core SPI Status interrupt. Asserted when the SPI_STATUS signal from the BLE Core is asserted, indicating that SPI writes can be done to the BLE Core. Transfers to the BLE Core should only be done when this signal is high."]
    #[must_use]
    #[inline(always)]
    pub const fn blecsstat(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "BLE Core SPI Status interrupt. Asserted when the SPI_STATUS signal from the BLE Core is asserted, indicating that SPI writes can be done to the BLE Core. Transfers to the BLE Core should only be done when this signal is high."]
    #[inline(always)]
    pub const fn set_blecsstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state."]
    #[inline(always)]
    pub const fn set_dcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
    #[must_use]
    #[inline(always)]
    pub const fn derr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
    #[inline(always)]
    pub const fn set_derr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
    #[must_use]
    #[inline(always)]
    pub const fn cqpaused(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
    #[inline(always)]
    pub const fn set_cqpaused(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Command queue write operation executed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
    #[must_use]
    #[inline(always)]
    pub const fn cqupd(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue write operation executed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
    #[inline(always)]
    pub const fn set_cqupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Command queue error during processing. When an error occurs, the system will stop processing and halt operations to allow software to take recovery actions."]
    #[must_use]
    #[inline(always)]
    pub const fn cqerr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue error during processing. When an error occurs, the system will stop processing and halt operations to allow software to take recovery actions."]
    #[inline(always)]
    pub const fn set_cqerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "The B2M_STATE from the BLE Core transitioned into the sleep state."]
    #[must_use]
    #[inline(always)]
    pub const fn b2msleep(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "The B2M_STATE from the BLE Core transitioned into the sleep state."]
    #[inline(always)]
    pub const fn set_b2msleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Revision A: The B2M_STATE from the BLE Core transitioned into the active state Revision B: Falling BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is de-asserted (1 -> 0)."]
    #[must_use]
    #[inline(always)]
    pub const fn b2mactive(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Revision A: The B2M_STATE from the BLE Core transitioned into the active state Revision B: Falling BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is de-asserted (1 -> 0)."]
    #[inline(always)]
    pub const fn set_b2mactive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Revision A: The B2M_STATE from the BLE Core transitioned into shutdown state Revision B: Falling BLE Core Status signal. Asserted when the BLE_STATUS signal from the BLE Core is de-asserted (1 -> 0)."]
    #[must_use]
    #[inline(always)]
    pub const fn b2mshutdn(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Revision A: The B2M_STATE from the BLE Core transitioned into shutdown state Revision B: Falling BLE Core Status signal. Asserted when the BLE_STATUS signal from the BLE Core is de-asserted (1 -> 0)."]
    #[inline(always)]
    pub const fn set_b2mshutdn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
            .field("b2mst", &self.b2mst())
            .field("iacc", &self.iacc())
            .field("icmd", &self.icmd())
            .field("blecirq", &self.blecirq())
            .field("blecsstat", &self.blecsstat())
            .field("dcmp", &self.dcmp())
            .field("derr", &self.derr())
            .field("cqpaused", &self.cqpaused())
            .field("cqupd", &self.cqupd())
            .field("cqerr", &self.cqerr())
            .field("b2msleep", &self.b2msleep())
            .field("b2mactive", &self.b2mactive())
            .field("b2mshutdn", &self.b2mshutdn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intset {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Intset {{ cmdcmp: {=bool:?}, thr: {=bool:?}, fundfl: {=bool:?}, fovfl: {=bool:?}, b2mst: {=bool:?}, iacc: {=bool:?}, icmd: {=bool:?}, blecirq: {=bool:?}, blecsstat: {=bool:?}, dcmp: {=bool:?}, derr: {=bool:?}, cqpaused: {=bool:?}, cqupd: {=bool:?}, cqerr: {=bool:?}, b2msleep: {=bool:?}, b2mactive: {=bool:?}, b2mshutdn: {=bool:?} }}" , self . cmdcmp () , self . thr () , self . fundfl () , self . fovfl () , self . b2mst () , self . iacc () , self . icmd () , self . blecirq () , self . blecsstat () , self . dcmp () , self . derr () , self . cqpaused () , self . cqupd () , self . cqerr () , self . b2msleep () , self . b2mactive () , self . b2mshutdn ())
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
    #[doc = "Read FIFO Underflow interrupt. Asserted when a pop operation is done to a empty read FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn fundfl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Read FIFO Underflow interrupt. Asserted when a pop operation is done to a empty read FIFO."]
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
    #[doc = "B2M State change interrupt. Asserted on any change in the B2M_STATE signal from the BLE Core."]
    #[must_use]
    #[inline(always)]
    pub const fn b2mst(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "B2M State change interrupt. Asserted on any change in the B2M_STATE signal from the BLE Core."]
    #[inline(always)]
    pub const fn set_b2mst(&mut self, val: bool) {
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
    #[doc = "BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is asserted, indicating the availability of read data from the BLE Core."]
    #[must_use]
    #[inline(always)]
    pub const fn blecirq(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is asserted, indicating the availability of read data from the BLE Core."]
    #[inline(always)]
    pub const fn set_blecirq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "BLE Core SPI Status interrupt. Asserted when the SPI_STATUS signal from the BLE Core is asserted, indicating that SPI writes can be done to the BLE Core. Transfers to the BLE Core should only be done when this signal is high."]
    #[must_use]
    #[inline(always)]
    pub const fn blecsstat(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "BLE Core SPI Status interrupt. Asserted when the SPI_STATUS signal from the BLE Core is asserted, indicating that SPI writes can be done to the BLE Core. Transfers to the BLE Core should only be done when this signal is high."]
    #[inline(always)]
    pub const fn set_blecsstat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state."]
    #[must_use]
    #[inline(always)]
    pub const fn dcmp(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Complete. Processing of the DMA operation has completed and the DMA submodule is returned into the idle state."]
    #[inline(always)]
    pub const fn set_dcmp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
    #[must_use]
    #[inline(always)]
    pub const fn derr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Error encountered during the processing of the DMA command. The DMA error could occur when the memory access specified in the DMA operation is not available or incorrectly specified."]
    #[inline(always)]
    pub const fn set_derr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
    #[must_use]
    #[inline(always)]
    pub const fn cqpaused(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue is paused due to an active event enabled in the PAUSEEN register. The interrupt is posted when the event is enabled within the PAUSEEN register, the mask is active in the CQIRQMASK field and the event occurs."]
    #[inline(always)]
    pub const fn set_cqpaused(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Command queue write operation executed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
    #[must_use]
    #[inline(always)]
    pub const fn cqupd(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue write operation executed a register write with the register address bit 0 set to 1. The low address bits in the CQ address fields are unused and bit 0 can be used to trigger an interrupt to indicate when this register write is performed by the CQ operation."]
    #[inline(always)]
    pub const fn set_cqupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Command queue error during processing. When an error occurs, the system will stop processing and halt operations to allow software to take recovery actions."]
    #[must_use]
    #[inline(always)]
    pub const fn cqerr(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Command queue error during processing. When an error occurs, the system will stop processing and halt operations to allow software to take recovery actions."]
    #[inline(always)]
    pub const fn set_cqerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "The B2M_STATE from the BLE Core transitioned into the sleep state."]
    #[must_use]
    #[inline(always)]
    pub const fn b2msleep(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "The B2M_STATE from the BLE Core transitioned into the sleep state."]
    #[inline(always)]
    pub const fn set_b2msleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Revision A: The B2M_STATE from the BLE Core transitioned into the active state Revision B: Falling BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is de-asserted (1 -> 0)."]
    #[must_use]
    #[inline(always)]
    pub const fn b2mactive(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Revision A: The B2M_STATE from the BLE Core transitioned into the active state Revision B: Falling BLE Core IRQ signal. Asserted when the BLE_IRQ signal from the BLE Core is de-asserted (1 -> 0)."]
    #[inline(always)]
    pub const fn set_b2mactive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Revision A: The B2M_STATE from the BLE Core transitioned into shutdown state Revision B: Falling BLE Core Status signal. Asserted when the BLE_STATUS signal from the BLE Core is de-asserted (1 -> 0)."]
    #[must_use]
    #[inline(always)]
    pub const fn b2mshutdn(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Revision A: The B2M_STATE from the BLE Core transitioned into shutdown state Revision B: Falling BLE Core Status signal. Asserted when the BLE_STATUS signal from the BLE Core is de-asserted (1 -> 0)."]
    #[inline(always)]
    pub const fn set_b2mshutdn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
            .field("b2mst", &self.b2mst())
            .field("iacc", &self.iacc())
            .field("icmd", &self.icmd())
            .field("blecirq", &self.blecirq())
            .field("blecsstat", &self.blecsstat())
            .field("dcmp", &self.dcmp())
            .field("derr", &self.derr())
            .field("cqpaused", &self.cqpaused())
            .field("cqupd", &self.cqupd())
            .field("cqerr", &self.cqerr())
            .field("b2msleep", &self.b2msleep())
            .field("b2mactive", &self.b2mactive())
            .field("b2mshutdn", &self.b2mshutdn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Intstat {{ cmdcmp: {=bool:?}, thr: {=bool:?}, fundfl: {=bool:?}, fovfl: {=bool:?}, b2mst: {=bool:?}, iacc: {=bool:?}, icmd: {=bool:?}, blecirq: {=bool:?}, blecsstat: {=bool:?}, dcmp: {=bool:?}, derr: {=bool:?}, cqpaused: {=bool:?}, cqupd: {=bool:?}, cqerr: {=bool:?}, b2msleep: {=bool:?}, b2mactive: {=bool:?}, b2mshutdn: {=bool:?} }}" , self . cmdcmp () , self . thr () , self . fundfl () , self . fovfl () , self . b2mst () , self . iacc () , self . icmd () , self . blecirq () , self . blecsstat () , self . dcmp () , self . derr () , self . cqpaused () , self . cqupd () , self . cqerr () , self . b2msleep () , self . b2mactive () , self . b2mshutdn ())
    }
}
#[doc = "SPI module master configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mspicfg(pub u32);
impl Mspicfg {
    #[doc = "This bit selects SPI polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn spol(&self) -> super::vals::Spol {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Spol::from_bits(val as u8)
    }
    #[doc = "This bit selects SPI polarity."]
    #[inline(always)]
    pub const fn set_spol(&mut self, val: super::vals::Spol) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Selects the SPI phase; When 1, will shift the sampling edge by 1/2 clock."]
    #[must_use]
    #[inline(always)]
    pub const fn spha(&self) -> super::vals::Spha {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Spha::from_bits(val as u8)
    }
    #[doc = "Selects the SPI phase; When 1, will shift the sampling edge by 1/2 clock."]
    #[inline(always)]
    pub const fn set_spha(&mut self, val: super::vals::Spha) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Full Duplex mode. Capture read data during writes operations."]
    #[must_use]
    #[inline(always)]
    pub const fn fulldup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Full Duplex mode. Capture read data during writes operations."]
    #[inline(always)]
    pub const fn set_fulldup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables flow control of new write transactions based on the SPI_STATUS signal from the BLE Core."]
    #[must_use]
    #[inline(always)]
    pub const fn wtfc(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Enables flow control of new write transactions based on the SPI_STATUS signal from the BLE Core."]
    #[inline(always)]
    pub const fn set_wtfc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Enables flow control of new read transactions based on the SPI_STATUS signal from the BLE Core."]
    #[must_use]
    #[inline(always)]
    pub const fn rdfc(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Enables flow control of new read transactions based on the SPI_STATUS signal from the BLE Core."]
    #[inline(always)]
    pub const fn set_rdfc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Selects the write flow control signal polarity. The transfers are halted when the selected flow control signal is OPPOSITE polarity of this bit. (For example: WTFCPOL = 0 will allow a SPI_STATUS=1 to pause transfers)."]
    #[must_use]
    #[inline(always)]
    pub const fn wtfcpol(&self) -> super::vals::Wtfcpol {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Wtfcpol::from_bits(val as u8)
    }
    #[doc = "Selects the write flow control signal polarity. The transfers are halted when the selected flow control signal is OPPOSITE polarity of this bit. (For example: WTFCPOL = 0 will allow a SPI_STATUS=1 to pause transfers)."]
    #[inline(always)]
    pub const fn set_wtfcpol(&mut self, val: super::vals::Wtfcpol) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Selects the read flow control signal polarity. When set, the clock will be held low until the flow control is de-asserted."]
    #[must_use]
    #[inline(always)]
    pub const fn rdfcpol(&self) -> super::vals::Rdfcpol {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Rdfcpol::from_bits(val as u8)
    }
    #[doc = "Selects the read flow control signal polarity. When set, the clock will be held low until the flow control is de-asserted."]
    #[inline(always)]
    pub const fn set_rdfcpol(&mut self, val: super::vals::Rdfcpol) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Selects data transfer as MSB first (0) or LSB first (1) for the data portion of the SPI transaction. The offset bytes are always transmitted MSB first."]
    #[must_use]
    #[inline(always)]
    pub const fn spilsb(&self) -> super::vals::Spilsb {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Spilsb::from_bits(val as u8)
    }
    #[doc = "Selects data transfer as MSB first (0) or LSB first (1) for the data portion of the SPI transaction. The offset bytes are always transmitted MSB first."]
    #[inline(always)]
    pub const fn set_spilsb(&mut self, val: super::vals::Spilsb) {
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
    #[doc = "Bit is deprecated. setting it will have no effect."]
    #[must_use]
    #[inline(always)]
    pub const fn mspirst(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Bit is deprecated. setting it will have no effect."]
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
        defmt :: write ! (f , "Mspicfg {{ spol: {:?}, spha: {:?}, fulldup: {=bool:?}, wtfc: {=bool:?}, rdfc: {=bool:?}, wtfcpol: {:?}, rdfcpol: {:?}, spilsb: {:?}, dindly: {=u8:?}, doutdly: {=u8:?}, mspirst: {=bool:?} }}" , self . spol () , self . spha () , self . fulldup () , self . wtfc () , self . rdfc () , self . wtfcpol () , self . rdfcpol () , self . spilsb () , self . dindly () , self . doutdly () , self . mspirst ())
    }
}
#[doc = "High order offset bytes."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Offsethi(pub u32);
impl Offsethi {
    #[doc = "Holds the high order bytes of the 2 or 3 byte offset phase of a transaction."]
    #[must_use]
    #[inline(always)]
    pub const fn offsethi(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Holds the high order bytes of the 2 or 3 byte offset phase of a transaction."]
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
#[doc = "BLE Power command interface."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwrcmd(pub u32);
impl Pwrcmd {
    #[doc = "Wake request from the MCU. When asserted (1), the BLE Interface logic will assert the wakeup request signal to the BLE Core. Only recognized when in the sleep state."]
    #[must_use]
    #[inline(always)]
    pub const fn wakereq(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Wake request from the MCU. When asserted (1), the BLE Interface logic will assert the wakeup request signal to the BLE Core. Only recognized when in the sleep state."]
    #[inline(always)]
    pub const fn set_wakereq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Restart the BLE Core after going into the shutdown state. Only valid when in the shutdown state."]
    #[must_use]
    #[inline(always)]
    pub const fn restart(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Restart the BLE Core after going into the shutdown state. Only valid when in the shutdown state."]
    #[inline(always)]
    pub const fn set_restart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Pwrcmd {
    #[inline(always)]
    fn default() -> Pwrcmd {
        Pwrcmd(0)
    }
}
impl core::fmt::Debug for Pwrcmd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pwrcmd")
            .field("wakereq", &self.wakereq())
            .field("restart", &self.restart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pwrcmd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pwrcmd {{ wakereq: {=bool:?}, restart: {=bool:?} }}",
            self.wakereq(),
            self.restart()
        )
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
