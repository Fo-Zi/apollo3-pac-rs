#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum En {
    #[doc = "Enable CT0 for output value."]
    En = 0x0,
    #[doc = "Disable CT0 for output value."]
    Dis = 0x01,
}
impl En {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> En {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for En {
    #[inline(always)]
    fn from(val: u8) -> En {
        En::from_bits(val)
    }
}
impl From<En> for u8 {
    #[inline(always)]
    fn from(val: En) -> u8 {
        En::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpioincfg {
    #[doc = "Read the GPIO pin data value."]
    Read = 0x0,
    #[doc = "INTD = 0 - Readback will always be zero value."]
    Rdzero = 0x01,
}
impl Gpioincfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpioincfg {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpioincfg {
    #[inline(always)]
    fn from(val: u8) -> Gpioincfg {
        Gpioincfg::from_bits(val)
    }
}
impl From<Gpioincfg> for u8 {
    #[inline(always)]
    fn from(val: Gpioincfg) -> u8 {
        Gpioincfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpiointdIntmode {
    #[doc = "INCFG = 1 - No interrupt on GPIO transition value."]
    Intdis = 0x0,
    #[doc = "INCFG = 1 - Interrupt on either low to high or high to low GPIO transition value."]
    Intboth = 0x01,
}
impl GpiointdIntmode {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpiointdIntmode {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpiointdIntmode {
    #[inline(always)]
    fn from(val: u8) -> GpiointdIntmode {
        GpiointdIntmode::from_bits(val)
    }
}
impl From<GpiointdIntmode> for u8 {
    #[inline(always)]
    fn from(val: GpiointdIntmode) -> u8 {
        GpiointdIntmode::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpiointdNcepol {
    #[doc = "FNCSEL = 0x7 - nCE polarity active low value."]
    NCelow = 0x0,
    #[doc = "FNCSEL = 0x7 - nCE polarity active high value."]
    NCehigh = 0x01,
}
impl GpiointdNcepol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> GpiointdNcepol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for GpiointdNcepol {
    #[inline(always)]
    fn from(val: u8) -> GpiointdNcepol {
        GpiointdNcepol::from_bits(val)
    }
}
impl From<GpiointdNcepol> for u8 {
    #[inline(always)]
    fn from(val: GpiointdNcepol) -> u8 {
        GpiointdNcepol::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Gpiooutcfg {
    #[doc = "FNCSEL = 0x3 - Output disabled value."]
    Dis = 0x0,
    #[doc = "FNCSEL = 0x3 - Output is push-pull value."]
    Pushpull = 0x01,
    #[doc = "FNCSEL = 0x3 - Output is open drain value."]
    Od = 0x02,
    #[doc = "FNCSEL = 0x3 - Output is tri-state value."]
    Ts = 0x03,
}
impl Gpiooutcfg {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpiooutcfg {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpiooutcfg {
    #[inline(always)]
    fn from(val: u8) -> Gpiooutcfg {
        Gpiooutcfg::from_bits(val)
    }
}
impl From<Gpiooutcfg> for u8 {
    #[inline(always)]
    fn from(val: Gpiooutcfg) -> u8 {
        Gpiooutcfg::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad0fncsel {
    #[doc = "Configure as the IOSLAVE I2C SCL signal value."]
    Slscl = 0x0,
    #[doc = "Configure as the IOSLAVE SPI SCK signal value."]
    Slsck = 0x01,
    #[doc = "Configure as the CLKOUT signal value."]
    Clkout = 0x02,
    #[doc = "Configure as GPIO0 value."]
    Gpio0 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "MSPI data connection 4 value."]
    Mspi4 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "IOM/MSPI nCE group 0 value."]
    Nce0 = 0x07,
}
impl Pad0fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad0fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad0fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad0fncsel {
        Pad0fncsel::from_bits(val)
    }
}
impl From<Pad0fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad0fncsel) -> u8 {
        Pad0fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad10fncsel {
    _RESERVED_0 = 0x0,
    #[doc = "Configure as the IOMSTR1 SPI MOSI signal value."]
    M1mosi = 0x01,
    #[doc = "IOM/MSPI nCE group 10 value."]
    Nce10 = 0x02,
    #[doc = "Configure as GPIO10 value."]
    Gpio10 = 0x03,
    #[doc = "PDM serial clock out value."]
    Pdmclk = 0x04,
    #[doc = "Configure as the UART1 RTS output signal value."]
    Ua1rts = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad10fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad10fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad10fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad10fncsel {
        Pad10fncsel::from_bits(val)
    }
}
impl From<Pad10fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad10fncsel) -> u8 {
        Pad10fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad11fncsel {
    #[doc = "Configure as the analog input for ADC single ended input 2 value."]
    Adcse2 = 0x0,
    #[doc = "IOM/MSPI nCE group 11 value."]
    Nce11 = 0x01,
    #[doc = "CTIMER connection 31 value."]
    Ct31 = 0x02,
    #[doc = "Configure as GPIO11 value."]
    Gpio11 = 0x03,
    #[doc = "Configure as the IOSLAVE interrupt out signal value."]
    Slint = 0x04,
    #[doc = "Configure as the UART1 CTS input signal value."]
    Ua1cts = 0x05,
    #[doc = "Configure as the UART0 RX input signal value."]
    Uart0rx = 0x06,
    #[doc = "Configure as the PDM Data input signal value."]
    PdmData = 0x07,
}
impl Pad11fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad11fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad11fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad11fncsel {
        Pad11fncsel::from_bits(val)
    }
}
impl From<Pad11fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad11fncsel) -> u8 {
        Pad11fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad12fncsel {
    #[doc = "Configure as the ADC Differential pair 0 N, or Single Ended input 9 analog input signal. Determination of the D0N vs SE9 usage is done when the particular channel is selected within the ADC module value."]
    Adcd0nse9 = 0x0,
    #[doc = "IOM/MSPI nCE group 12 value."]
    Nce12 = 0x01,
    #[doc = "CTIMER connection 0 value."]
    Ct0 = 0x02,
    #[doc = "Configure as GPIO12 value."]
    Gpio12 = 0x03,
    #[doc = "Configure as the IOSLAVE SPI nCE signal value."]
    SLnCe = 0x04,
    #[doc = "PDM serial clock output value."]
    Pdmclk = 0x05,
    #[doc = "Configure as the UART0 CTS input signal value."]
    Ua0cts = 0x06,
    #[doc = "Configure as the UART1 TX output signal value."]
    Uart1tx = 0x07,
}
impl Pad12fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad12fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad12fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad12fncsel {
        Pad12fncsel::from_bits(val)
    }
}
impl From<Pad12fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad12fncsel) -> u8 {
        Pad12fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad13fncsel {
    #[doc = "Configure as the ADC Differential pair 0 P, or Single Ended input 8 analog input signal. Determination of the D0P vs SE8 usage is done when the particular channel is selected within the ADC module value."]
    Adcd0pse8 = 0x0,
    #[doc = "IOM/MSPI nCE group 13 value."]
    Nce13 = 0x01,
    #[doc = "CTIMER connection 2 value."]
    Ct2 = 0x02,
    #[doc = "Configure as GPIO13 value."]
    Gpio13 = 0x03,
    #[doc = "I2C interface bit clock value."]
    I2sbclk = 0x04,
    #[doc = "Configure as the external HFRC oscillator input value."]
    Exthfb = 0x05,
    #[doc = "Configure as the UART0 RTS signal output value."]
    Ua0rts = 0x06,
    #[doc = "Configure as the UART1 RX input signal value."]
    Uart1rx = 0x07,
}
impl Pad13fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad13fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad13fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad13fncsel {
        Pad13fncsel::from_bits(val)
    }
}
impl From<Pad13fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad13fncsel) -> u8 {
        Pad13fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad14fncsel {
    #[doc = "Configure as the analog ADC differential pair 1 P input signal value."]
    Adcd1p = 0x0,
    #[doc = "IOM/MSPI nCE group 14 value."]
    Nce14 = 0x01,
    #[doc = "Configure as the UART1 TX output signal value."]
    Uart1tx = 0x02,
    #[doc = "Configure as GPIO14 value."]
    Gpio14 = 0x03,
    #[doc = "PDM serial clock output value."]
    Pdmclk = 0x04,
    #[doc = "Configure as the External HFRC oscillator input select value."]
    Exthfs = 0x05,
    #[doc = "Configure as the alternate input for the SWDCK input signal value."]
    Swdck = 0x06,
    #[doc = "Configure as the 32kHz crystal output signal value."]
    _32kHzXt = 0x07,
}
impl Pad14fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad14fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad14fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad14fncsel {
        Pad14fncsel::from_bits(val)
    }
}
impl From<Pad14fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad14fncsel) -> u8 {
        Pad14fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad15fncsel {
    #[doc = "Configure as the analog ADC differential pair 1 N input signal value."]
    Adcd1n = 0x0,
    #[doc = "IOM/MSPI nCE group 15 value."]
    Nce15 = 0x01,
    #[doc = "Configure as the UART1 RX signal value."]
    Uart1rx = 0x02,
    #[doc = "Configure as GPIO15 value."]
    Gpio15 = 0x03,
    #[doc = "PDM serial data input value."]
    Pdmdata = 0x04,
    #[doc = "Configure as the external XTAL oscillator input value."]
    Extxt = 0x05,
    #[doc = "Configure as an alternate port for the SWDIO I/O signal value."]
    Swdio = 0x06,
    #[doc = "Configure as an SWO (Serial Wire Trace output) value."]
    Swo = 0x07,
}
impl Pad15fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad15fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad15fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad15fncsel {
        Pad15fncsel::from_bits(val)
    }
}
impl From<Pad15fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad15fncsel) -> u8 {
        Pad15fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad16fncsel {
    #[doc = "Configure as the analog ADC single ended port 0 input signal value."]
    Adcse0 = 0x0,
    #[doc = "IOM/MSPI nCE group 16 value."]
    Nce16 = 0x01,
    #[doc = "Configure as the ADC Trigger 0 signal value."]
    Trig0 = 0x02,
    #[doc = "Configure as GPIO16 value."]
    Gpio16 = 0x03,
    #[doc = "SCARD reset output value."]
    Sccrst = 0x04,
    #[doc = "Configure as comparator input 0 signal value."]
    Cmpin0 = 0x05,
    #[doc = "Configure as UART0 TX output signal value."]
    Uart0tx = 0x06,
    #[doc = "Configure as UART1 RTS output signal value."]
    Ua1rts = 0x07,
}
impl Pad16fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad16fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad16fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad16fncsel {
        Pad16fncsel::from_bits(val)
    }
}
impl From<Pad16fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad16fncsel) -> u8 {
        Pad16fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad17fncsel {
    #[doc = "Configure as the analog comparator reference signal 1 input signal value."]
    Cmprf1 = 0x0,
    #[doc = "IOM/MSPI nCE group 17 value."]
    Nce17 = 0x01,
    #[doc = "Configure as the ADC Trigger 1 signal value."]
    Trig1 = 0x02,
    #[doc = "Configure as GPIO17 value."]
    Gpio17 = 0x03,
    #[doc = "SCARD serial clock output value."]
    Sccclk = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Configure as UART0 RX input signal value."]
    Uart0rx = 0x06,
    #[doc = "Configure as UART1 CTS input signal value."]
    Ua1cts = 0x07,
}
impl Pad17fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad17fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad17fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad17fncsel {
        Pad17fncsel::from_bits(val)
    }
}
impl From<Pad17fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad17fncsel) -> u8 {
        Pad17fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad18fncsel {
    #[doc = "Configure as the analog comparator input 1 signal value."]
    Cmpin1 = 0x0,
    #[doc = "IOM/MSPI nCE group 18 value."]
    Nce18 = 0x01,
    #[doc = "CTIMER connection 4 value."]
    Ct4 = 0x02,
    #[doc = "Configure as GPIO18 value."]
    Gpio18 = 0x03,
    #[doc = "Configure as UART0 RTS output signal value."]
    Ua0rts = 0x04,
    #[doc = "Configure as ANATEST2 I/O signal value."]
    Anatest2 = 0x05,
    #[doc = "Configure as UART1 TX output signal value."]
    Uart1tx = 0x06,
    #[doc = "SCARD data input/output connectin value."]
    Sccio = 0x07,
}
impl Pad18fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad18fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad18fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad18fncsel {
        Pad18fncsel::from_bits(val)
    }
}
impl From<Pad18fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad18fncsel) -> u8 {
        Pad18fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad19fncsel {
    #[doc = "Configure as the analog comparator reference 0 signal value."]
    Cmprf0 = 0x0,
    #[doc = "IOM/MSPI nCE group 19 value."]
    Nce19 = 0x01,
    #[doc = "CTIMER conenction 6 value."]
    Ct6 = 0x02,
    #[doc = "Configure as GPIO19 value."]
    Gpio19 = 0x03,
    #[doc = "SCARD serial clock value."]
    Scclk = 0x04,
    #[doc = "Configure as the ANATEST1 I/O signal value."]
    Anatest1 = 0x05,
    #[doc = "Configure as the UART1 RX input signal value."]
    Uart1rx = 0x06,
    #[doc = "Configure as the PDM I2S bit clock input signal value."]
    I2sbclk = 0x07,
}
impl Pad19fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad19fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad19fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad19fncsel {
        Pad19fncsel::from_bits(val)
    }
}
impl From<Pad19fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad19fncsel) -> u8 {
        Pad19fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad1fncsel {
    #[doc = "Configure as the IOSLAVE I2C SDA or SPI WIR3 signal value."]
    Slsdawir3 = 0x0,
    #[doc = "Configure as the IOSLAVE SPI MOSI signal value."]
    Slmosi = 0x01,
    #[doc = "Configure as the UART0 TX output signal value."]
    Uart0tx = 0x02,
    #[doc = "Configure as GPIO1 value."]
    Gpio1 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "MSPI data connection 5 value."]
    Mspi5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "IOM/MSPI nCE group 1 value."]
    Nce1 = 0x07,
}
impl Pad1fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad1fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad1fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad1fncsel {
        Pad1fncsel::from_bits(val)
    }
}
impl From<Pad1fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad1fncsel) -> u8 {
        Pad1fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad20fncsel {
    #[doc = "Configure as the serial wire debug clock signal value."]
    Swdck = 0x0,
    #[doc = "IOM/MSPI nCE group 20 value."]
    Nce20 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Configure as GPIO20 value."]
    Gpio20 = 0x03,
    #[doc = "Configure as UART0 TX output signal value."]
    Uart0tx = 0x04,
    #[doc = "Configure as UART1 TX output signal value."]
    Uart1tx = 0x05,
    #[doc = "I2S byte clock input value."]
    I2sbclk = 0x06,
    #[doc = "Configure as UART1 RTS output signal value."]
    Ua1rts = 0x07,
}
impl Pad20fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad20fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad20fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad20fncsel {
        Pad20fncsel::from_bits(val)
    }
}
impl From<Pad20fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad20fncsel) -> u8 {
        Pad20fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad21fncsel {
    #[doc = "Configure as the serial wire debug data signal value."]
    Swdio = 0x0,
    #[doc = "IOM/MSPI nCE group 21 value."]
    Nce21 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "Configure as GPIO21 value."]
    Gpio21 = 0x03,
    #[doc = "Configure as UART0 RX input signal value."]
    Uart0rx = 0x04,
    #[doc = "Configure as UART1 RX input signal value."]
    Uart1rx = 0x05,
    #[doc = "I2S byte clock input value."]
    I2sbclk = 0x06,
    #[doc = "Configure as UART1 CTS input signal value."]
    Ua1cts = 0x07,
}
impl Pad21fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad21fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad21fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad21fncsel {
        Pad21fncsel::from_bits(val)
    }
}
impl From<Pad21fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad21fncsel) -> u8 {
        Pad21fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad22fncsel {
    #[doc = "Configure as the UART0 TX signal value."]
    Uart0tx = 0x0,
    #[doc = "IOM/MSPI nCE group 22 value."]
    Nce22 = 0x01,
    #[doc = "CTIMER connection 12 value."]
    Ct12 = 0x02,
    #[doc = "Configure as GPIO22 value."]
    Gpio22 = 0x03,
    #[doc = "Configure as the PDM CLK output value."]
    PdmClk = 0x04,
    #[doc = "External LFRC input value."]
    Extlf = 0x05,
    #[doc = "MSPI data connection 0 value."]
    Mspi0 = 0x06,
    #[doc = "Configure as the serial trace data output signal value."]
    Swo = 0x07,
}
impl Pad22fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad22fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad22fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad22fncsel {
        Pad22fncsel::from_bits(val)
    }
}
impl From<Pad22fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad22fncsel) -> u8 {
        Pad22fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad23fncsel {
    #[doc = "Configure as the UART0 RX signal value."]
    Uart0rx = 0x0,
    #[doc = "IOM/MSPI nCE group 23 value."]
    Nce23 = 0x01,
    #[doc = "CTIMER connection 14 value."]
    Ct14 = 0x02,
    #[doc = "Configure as GPIO23 value."]
    Gpio23 = 0x03,
    #[doc = "I2S word clock input value."]
    I2swclk = 0x04,
    #[doc = "Configure as voltage comparitor output value."]
    Cmpout = 0x05,
    #[doc = "MSPI data connection 3 value."]
    Mspi3 = 0x06,
    #[doc = "External XTAL osacillatgor input value."]
    Extxt = 0x07,
}
impl Pad23fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad23fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad23fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad23fncsel {
        Pad23fncsel::from_bits(val)
    }
}
impl From<Pad23fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad23fncsel) -> u8 {
        Pad23fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad24fncsel {
    #[doc = "Configure as UART1 TX output signal value."]
    Uart1tx = 0x0,
    #[doc = "IOM/MSPI nCE group 24 value."]
    Nce24 = 0x01,
    #[doc = "MSPI data connection 8 value."]
    Mspi8 = 0x02,
    #[doc = "Configure as GPIO24 value."]
    Gpio24 = 0x03,
    #[doc = "Configure as UART0 CTS input signal value."]
    Ua0cts = 0x04,
    #[doc = "CTIMER connection 21 value."]
    Ct21 = 0x05,
    #[doc = "Configure as the 32kHz crystal output signal value."]
    _32kHzXt = 0x06,
    #[doc = "Configure as the serial trace data output signal value."]
    Swo = 0x07,
}
impl Pad24fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad24fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad24fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad24fncsel {
        Pad24fncsel::from_bits(val)
    }
}
impl From<Pad24fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad24fncsel) -> u8 {
        Pad24fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad25fncsel {
    #[doc = "Configure as UART1 RX input signal value."]
    Uart1rx = 0x0,
    #[doc = "IOM/MSPI nCE group 25 value."]
    Nce25 = 0x01,
    #[doc = "CTIMER connection 1 value."]
    Ct1 = 0x02,
    #[doc = "Configure as GPIO25 value."]
    Gpio25 = 0x03,
    #[doc = "Configure as the IOMSTR2 I2C SDA or SPI WIR3 signal value."]
    M2sdawir3 = 0x04,
    #[doc = "Configure as the IOMSTR2 SPI MISO input signal value."]
    M2miso = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad25fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad25fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad25fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad25fncsel {
        Pad25fncsel::from_bits(val)
    }
}
impl From<Pad25fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad25fncsel) -> u8 {
        Pad25fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad26fncsel {
    #[doc = "Configure as the external HFRC oscillator input value."]
    Exthf = 0x0,
    #[doc = "IOM/MSPI nCE group 26 value."]
    Nce26 = 0x01,
    #[doc = "CTIMER connection 3 value."]
    Ct3 = 0x02,
    #[doc = "Configure as GPIO26 value."]
    Gpio26 = 0x03,
    #[doc = "SCARD reset output value."]
    Sccrst = 0x04,
    #[doc = "MSPI data connection 1 value."]
    Mspi1 = 0x05,
    #[doc = "Configure as UART0 TX output signal value."]
    Uart0tx = 0x06,
    #[doc = "Configure as UART1 CTS input signal value."]
    Ua1cts = 0x07,
}
impl Pad26fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad26fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad26fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad26fncsel {
        Pad26fncsel::from_bits(val)
    }
}
impl From<Pad26fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad26fncsel) -> u8 {
        Pad26fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad27fncsel {
    #[doc = "Configure as UART0 RX input signal value."]
    Uart0rx = 0x0,
    #[doc = "IOM/MSPI nCE group 27 value."]
    Nce27 = 0x01,
    #[doc = "CTIMER connection 5 value."]
    Ct5 = 0x02,
    #[doc = "Configure as GPIO27 value."]
    Gpio27 = 0x03,
    #[doc = "Configure as I2C clock I/O signal from IOMSTR2 value."]
    M2scl = 0x04,
    #[doc = "Configure as SPI clock output signal from IOMSTR2 value."]
    M2sck = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad27fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad27fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad27fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad27fncsel {
        Pad27fncsel::from_bits(val)
    }
}
impl From<Pad27fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad27fncsel) -> u8 {
        Pad27fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad28fncsel {
    #[doc = "Configure as the PDM I2S Word Clock input value."]
    I2sWclk = 0x0,
    #[doc = "IOM/MSPI nCE group 28 value."]
    Nce28 = 0x01,
    #[doc = "CTIMER connection 7 value."]
    Ct7 = 0x02,
    #[doc = "Configure as GPIO28 value."]
    Gpio28 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Configure as the IOMSTR2 SPI MOSI output signal value."]
    M2mosi = 0x05,
    #[doc = "Configure as the UART0 TX output signal value."]
    Uart0tx = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad28fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad28fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad28fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad28fncsel {
        Pad28fncsel::from_bits(val)
    }
}
impl From<Pad28fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad28fncsel) -> u8 {
        Pad28fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad29fncsel {
    #[doc = "Configure as the analog input for ADC single ended input 1 value."]
    Adcse1 = 0x0,
    #[doc = "IOM/MSPI nCE group 29 value."]
    Nce29 = 0x01,
    #[doc = "CTIMER connection 9 value."]
    Ct9 = 0x02,
    #[doc = "Configure as GPIO29 value."]
    Gpio29 = 0x03,
    #[doc = "Configure as the UART0 CTS input signal value."]
    Ua0cts = 0x04,
    #[doc = "Configure as the UART1 CTS input signal value."]
    Ua1cts = 0x05,
    #[doc = "Configure as the UART0 RX input signal value."]
    Uart0rx = 0x06,
    #[doc = "Configure as PDM DATA input value."]
    PdmData = 0x07,
}
impl Pad29fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad29fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad29fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad29fncsel {
        Pad29fncsel::from_bits(val)
    }
}
impl From<Pad29fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad29fncsel) -> u8 {
        Pad29fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad2fncsel {
    _RESERVED_0 = 0x0,
    #[doc = "Configure as the IOSLAVE SPI MISO signal value."]
    Slmiso = 0x01,
    #[doc = "Configure as the UART0 RX input value."]
    Uart0rx = 0x02,
    #[doc = "Configure as GPIO2 value."]
    Gpio2 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CMSPI data connection 6 value."]
    Mspi6 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "IOM/MSPI nCE group 2 value."]
    Nce2 = 0x07,
}
impl Pad2fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad2fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad2fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad2fncsel {
        Pad2fncsel::from_bits(val)
    }
}
impl From<Pad2fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad2fncsel) -> u8 {
        Pad2fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad30fncsel {
    #[doc = "Configure as the ANATEST1 I/O signal value."]
    Anatest1 = 0x0,
    #[doc = "IOM/MSPI nCE group 30 value."]
    Nce30 = 0x01,
    #[doc = "CTIMER connection 11 value."]
    Ct11 = 0x02,
    #[doc = "Configure as GPIO30 value."]
    Gpio30 = 0x03,
    #[doc = "Configure as UART0 TX output signal value."]
    Uart0tx = 0x04,
    #[doc = "Configure as UART1 RTS output signal value."]
    Ua1rts = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Configure as the PDM I2S Data output signal value."]
    I2sDat = 0x07,
}
impl Pad30fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad30fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad30fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad30fncsel {
        Pad30fncsel::from_bits(val)
    }
}
impl From<Pad30fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad30fncsel) -> u8 {
        Pad30fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad31fncsel {
    #[doc = "Configure as the analog input for ADC single ended input 3 value."]
    Adcse3 = 0x0,
    #[doc = "IOM/MSPI nCE group 31 value."]
    Nce31 = 0x01,
    #[doc = "CTIMER connection 13 value."]
    Ct13 = 0x02,
    #[doc = "Configure as GPIO31 value."]
    Gpio31 = 0x03,
    #[doc = "Configure as the UART0 RX input signal value."]
    Uart0rx = 0x04,
    #[doc = "SCARD serial clock output value."]
    Sccclk = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Configure as UART1 RTS output signal value."]
    Ua1rts = 0x07,
}
impl Pad31fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad31fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad31fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad31fncsel {
        Pad31fncsel::from_bits(val)
    }
}
impl From<Pad31fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad31fncsel) -> u8 {
        Pad31fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad32fncsel {
    #[doc = "Configure as the analog input for ADC single ended input 4 value."]
    Adcse4 = 0x0,
    #[doc = "IOM/MSPI nCE group 32 value."]
    Nce32 = 0x01,
    #[doc = "CTIMER connection 15 value."]
    Ct15 = 0x02,
    #[doc = "Configure as GPIO32 value."]
    Gpio32 = 0x03,
    #[doc = "SCARD serial data input/output value."]
    Sccio = 0x04,
    #[doc = "External input to the LFRC oscillator value."]
    Extlf = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Configure as the UART1 CTS input value."]
    Ua1cts = 0x07,
}
impl Pad32fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad32fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad32fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad32fncsel {
        Pad32fncsel::from_bits(val)
    }
}
impl From<Pad32fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad32fncsel) -> u8 {
        Pad32fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad33fncsel {
    #[doc = "Configure as the analog ADC single ended port 5 input signal value."]
    Adcse5 = 0x0,
    #[doc = "IOM/MSPI nCE group 33 value."]
    Nce33 = 0x01,
    #[doc = "Configure as the 32kHz crystal output signal value."]
    _32kHzXt = 0x02,
    #[doc = "Configure as GPIO33 value."]
    Gpio33 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Configure as the UART0 CTS input value."]
    Ua0cts = 0x05,
    #[doc = "CTIMER connection 23 value."]
    Ct23 = 0x06,
    #[doc = "Configure as the serial trace data output signal value."]
    Swo = 0x07,
}
impl Pad33fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad33fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad33fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad33fncsel {
        Pad33fncsel::from_bits(val)
    }
}
impl From<Pad33fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad33fncsel) -> u8 {
        Pad33fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad34fncsel {
    #[doc = "Configure as the analog input for ADC single ended input 6 value."]
    Adcse6 = 0x0,
    #[doc = "IOM/MSPI nCE group 34 value."]
    Nce34 = 0x01,
    #[doc = "Configure as the UART1 RTS output value."]
    Ua1rts = 0x02,
    #[doc = "Configure as GPIO34 value."]
    Gpio34 = 0x03,
    #[doc = "Configure as the analog comparator reference 2 signal value."]
    Cmprf2 = 0x04,
    #[doc = "Configure as the UART0 RTS output value."]
    Ua0rts = 0x05,
    #[doc = "Configure as the UART0 RX input value."]
    Uart0rx = 0x06,
    #[doc = "PDM serial data input value."]
    Pdmdata = 0x07,
}
impl Pad34fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad34fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad34fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad34fncsel {
        Pad34fncsel::from_bits(val)
    }
}
impl From<Pad34fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad34fncsel) -> u8 {
        Pad34fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad35fncsel {
    #[doc = "Configure as the analog input for ADC single ended input 7 value."]
    Adcse7 = 0x0,
    #[doc = "IOM/MSPI nCE group 35 value."]
    Nce35 = 0x01,
    #[doc = "Configure as the UART1 TX signal value."]
    Uart1tx = 0x02,
    #[doc = "Configure as GPIO35 value."]
    Gpio35 = 0x03,
    #[doc = "I2S serial data output value."]
    I2sdat = 0x04,
    #[doc = "CTIMER connection 27 value."]
    Ct27 = 0x05,
    #[doc = "Configure as the UART0 RTS output value."]
    Ua0rts = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad35fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad35fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad35fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad35fncsel {
        Pad35fncsel::from_bits(val)
    }
}
impl From<Pad35fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad35fncsel) -> u8 {
        Pad35fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad36fncsel {
    #[doc = "Configure as the ADC Trigger 1 signal value."]
    Trig1 = 0x0,
    #[doc = "IOM/MSPI nCE group 36 value."]
    Nce36 = 0x01,
    #[doc = "Configure as the UART1 RX input signal value."]
    Uart1rx = 0x02,
    #[doc = "Configure as GPIO36 value."]
    Gpio36 = 0x03,
    #[doc = "Configure as the 32kHz output clock from the crystal value."]
    _32kHzXt = 0x04,
    #[doc = "Configure as the UART1 CTS input signal value."]
    Ua1cts = 0x05,
    #[doc = "Configure as the UART0 CTS input signal value."]
    Ua0cts = 0x06,
    #[doc = "PDM serial data input value."]
    Pdmdata = 0x07,
}
impl Pad36fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad36fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad36fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad36fncsel {
        Pad36fncsel::from_bits(val)
    }
}
impl From<Pad36fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad36fncsel) -> u8 {
        Pad36fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad37fncsel {
    #[doc = "Configure as the ADC Trigger 2 signal value."]
    Trig2 = 0x0,
    #[doc = "IOM/MSPI nCE group 37 value."]
    Nce37 = 0x01,
    #[doc = "Configure as the UART0 RTS output signal value."]
    Ua0rts = 0x02,
    #[doc = "Configure as GPIO37 value."]
    Gpio37 = 0x03,
    #[doc = "SCARD serial data input/output value."]
    Sccio = 0x04,
    #[doc = "Configure as the UART1 TX output signal value."]
    Uart1tx = 0x05,
    #[doc = "Configure as the PDM CLK output signal value."]
    Pdmclk = 0x06,
    #[doc = "CTIMER connection 29 value."]
    Ct29 = 0x07,
}
impl Pad37fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad37fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad37fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad37fncsel {
        Pad37fncsel::from_bits(val)
    }
}
impl From<Pad37fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad37fncsel) -> u8 {
        Pad37fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad38fncsel {
    #[doc = "Configure as the ADC Trigger 3 signal value."]
    Trig3 = 0x0,
    #[doc = "IOM/MSPI nCE group 38 value."]
    Nce38 = 0x01,
    #[doc = "Configure as the UART0 CTS signal value."]
    Ua0cts = 0x02,
    #[doc = "Configure as GPIO38 value."]
    Gpio38 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Configure as the IOMSTR3 SPI MOSI output signal value."]
    M3mosi = 0x05,
    #[doc = "Configure as the UART1 RX input signal value."]
    Uart1rx = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad38fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad38fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad38fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad38fncsel {
        Pad38fncsel::from_bits(val)
    }
}
impl From<Pad38fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad38fncsel) -> u8 {
        Pad38fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad39fncsel {
    #[doc = "Configure as the UART0 TX output signal value."]
    Uart0tx = 0x0,
    #[doc = "Configure as the UART1 TX output signal value."]
    Uart1tx = 0x01,
    #[doc = "CTIMER connection 25 value."]
    Ct25 = 0x02,
    #[doc = "Configure as GPIO39 value."]
    Gpio39 = 0x03,
    #[doc = "Configure as the IOMSTR4 I2C SCL signal value."]
    M4scl = 0x04,
    #[doc = "Configure as the IOMSTR4 SPI SCK signal value."]
    M4sck = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad39fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad39fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad39fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad39fncsel {
        Pad39fncsel::from_bits(val)
    }
}
impl From<Pad39fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad39fncsel) -> u8 {
        Pad39fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad3fncsel {
    #[doc = "Configure as the UART0 RTS output value."]
    Ua0rts = 0x0,
    #[doc = "Configure as the IOSLAVE SPI nCE signal value."]
    SLnCe = 0x01,
    #[doc = "IOM/MSPI nCE group 3 value."]
    Nce3 = 0x02,
    #[doc = "Configure as GPIO3 value."]
    Gpio3 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "MSPI data connection 7 value."]
    Mspi7 = 0x05,
    #[doc = "Configure as the ADC Trigger 1 signal value."]
    Trig1 = 0x06,
    #[doc = "Configure as the PDM I2S Word Clock input value."]
    I2sWclk = 0x07,
}
impl Pad3fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad3fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad3fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad3fncsel {
        Pad3fncsel::from_bits(val)
    }
}
impl From<Pad3fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad3fncsel) -> u8 {
        Pad3fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad40fncsel {
    #[doc = "Configure as the UART0 RX input signal value."]
    Uart0rx = 0x0,
    #[doc = "Configure as the UART1 RX input signal value."]
    Uart1rx = 0x01,
    #[doc = "Configure as the ADC Trigger 0 signal value."]
    Trig0 = 0x02,
    #[doc = "Configure as GPIO40 value."]
    Gpio40 = 0x03,
    #[doc = "Configure as the IOMSTR4 I2C SDA or SPI WIR3 signal value."]
    M4sdawir3 = 0x04,
    #[doc = "Configure as the IOMSTR4 SPI MISO input signal value."]
    M4miso = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad40fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad40fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad40fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad40fncsel {
        Pad40fncsel::from_bits(val)
    }
}
impl From<Pad40fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad40fncsel) -> u8 {
        Pad40fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad41fncsel {
    #[doc = "IOM/MSPI nCE group 41 value."]
    Nce41 = 0x0,
    _RESERVED_1 = 0x01,
    #[doc = "Configure as the serial wire debug SWO signal value."]
    Swo = 0x02,
    #[doc = "Configure as GPIO41 value."]
    Gpio41 = 0x03,
    #[doc = "I2S word clock input value."]
    I2swclk = 0x04,
    #[doc = "Configure as the UART1 RTS output signal value."]
    Ua1rts = 0x05,
    #[doc = "Configure as the UART0 TX output signal value."]
    Uart0tx = 0x06,
    #[doc = "Configure as the UART0 RTS output signal value."]
    Ua0rts = 0x07,
}
impl Pad41fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad41fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad41fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad41fncsel {
        Pad41fncsel::from_bits(val)
    }
}
impl From<Pad41fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad41fncsel) -> u8 {
        Pad41fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad42fncsel {
    #[doc = "Configure as the UART1 TX output signal value."]
    Uart1tx = 0x0,
    #[doc = "IOM/MSPI nCE group 42 value."]
    Nce42 = 0x01,
    #[doc = "CTIMER connection 16 value."]
    Ct16 = 0x02,
    #[doc = "Configure as GPIO42 value."]
    Gpio42 = 0x03,
    #[doc = "Configure as the IOMSTR3 I2C SCL clock I/O signal value."]
    M3scl = 0x04,
    #[doc = "Configure as the IOMSTR3 SPI SCK output value."]
    M3sck = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad42fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad42fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad42fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad42fncsel {
        Pad42fncsel::from_bits(val)
    }
}
impl From<Pad42fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad42fncsel) -> u8 {
        Pad42fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad43fncsel {
    #[doc = "Configure as the UART1 RX input signal value."]
    Uart1rx = 0x0,
    #[doc = "IOM/MSPI nCE group 43 value."]
    Nce43 = 0x01,
    #[doc = "CTIMER connection 18 value."]
    Ct18 = 0x02,
    #[doc = "Configure as GPIO43 value."]
    Gpio43 = 0x03,
    #[doc = "Configure as the IOMSTR3 I2C SDA or SPI WIR3 signal value."]
    M3sdawir3 = 0x04,
    #[doc = "Configure as the IOMSTR3 SPI MISO signal value."]
    M3miso = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad43fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad43fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad43fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad43fncsel {
        Pad43fncsel::from_bits(val)
    }
}
impl From<Pad43fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad43fncsel) -> u8 {
        Pad43fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad44fncsel {
    #[doc = "Configure as the UART1 RTS output signal value."]
    Ua1rts = 0x0,
    #[doc = "IOM/MSPI nCE group 44 value."]
    Nce44 = 0x01,
    #[doc = "CTIMER connection 20 value."]
    Ct20 = 0x02,
    #[doc = "Configure as GPIO44 value."]
    Gpio44 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Configure as the IOMSTR4 SPI MOSI signal value."]
    M4mosi = 0x05,
    #[doc = "Configure as the SPI channel 6 nCE signal from IOMSTR5 value."]
    M5nCe6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad44fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad44fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad44fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad44fncsel {
        Pad44fncsel::from_bits(val)
    }
}
impl From<Pad44fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad44fncsel) -> u8 {
        Pad44fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad45fncsel {
    #[doc = "Configure as the UART1 CTS input signal value."]
    Ua1cts = 0x0,
    #[doc = "IOM/MSPI nCE group 45 value."]
    Nce45 = 0x01,
    #[doc = "CTIMER connection 22 value."]
    Ct22 = 0x02,
    #[doc = "Configure as GPIO45 value."]
    Gpio45 = 0x03,
    #[doc = "I2S serial data output value."]
    I2sdat = 0x04,
    #[doc = "PDM serial data input value."]
    Pdmdata = 0x05,
    #[doc = "Configure as the SPI channel 5 nCE signal from IOMSTR5 value."]
    Uart0rx = 0x06,
    #[doc = "Configure as the serial wire debug SWO signal value."]
    Swo = 0x07,
}
impl Pad45fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad45fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad45fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad45fncsel {
        Pad45fncsel::from_bits(val)
    }
}
impl From<Pad45fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad45fncsel) -> u8 {
        Pad45fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad46fncsel {
    #[doc = "Configure as the 32kHz output clock from the crystal value."]
    _32khzXt = 0x0,
    #[doc = "IOM/MSPI nCE group 46 value."]
    Nce46 = 0x01,
    #[doc = "CTIMER connection 24 value."]
    Ct24 = 0x02,
    #[doc = "Configure as GPIO46 value."]
    Gpio46 = 0x03,
    #[doc = "SCARD reset output value."]
    Sccrst = 0x04,
    #[doc = "PDM serial clock output value."]
    Pdmclk = 0x05,
    #[doc = "Configure as the UART1 TX output signal value."]
    Uart1tx = 0x06,
    #[doc = "Configure as the serial wire debug SWO signal value."]
    Swo = 0x07,
}
impl Pad46fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad46fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad46fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad46fncsel {
        Pad46fncsel::from_bits(val)
    }
}
impl From<Pad46fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad46fncsel) -> u8 {
        Pad46fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad47fncsel {
    #[doc = "Configure as the 32kHz output clock from the crystal value."]
    _32kHzXt = 0x0,
    #[doc = "IOM/MSPI nCE group 47 value."]
    Nce47 = 0x01,
    #[doc = "CTIMER connection 26 value."]
    Ct26 = 0x02,
    #[doc = "Configure as GPIO47 value."]
    Gpio47 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Configure as the IOMSTR5 SPI MOSI output signal value."]
    M5mosi = 0x05,
    #[doc = "Configure as the UART1 RX input signal value."]
    Uart1rx = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad47fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad47fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad47fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad47fncsel {
        Pad47fncsel::from_bits(val)
    }
}
impl From<Pad47fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad47fncsel) -> u8 {
        Pad47fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad48fncsel {
    #[doc = "Configure as the UART0 TX output signal value."]
    Uart0tx = 0x0,
    #[doc = "IOM/MSPI nCE group 48 value."]
    Nce48 = 0x01,
    #[doc = "CTIMER conenction 28 value."]
    Ct28 = 0x02,
    #[doc = "Configure as GPIO48 value."]
    Gpio48 = 0x03,
    #[doc = "Configure as the IOMSTR5 I2C SCL clock I/O signal value."]
    M5scl = 0x04,
    #[doc = "Configure as the IOMSTR5 SPI SCK output value."]
    M5sck = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad48fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad48fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad48fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad48fncsel {
        Pad48fncsel::from_bits(val)
    }
}
impl From<Pad48fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad48fncsel) -> u8 {
        Pad48fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad49fncsel {
    #[doc = "Configure as the UART0 RX input signal value."]
    Uart0rx = 0x0,
    #[doc = "IOM/MSPPI nCE group 49 value."]
    Nce49 = 0x01,
    #[doc = "CTIMER connection 30 value."]
    Ct30 = 0x02,
    #[doc = "Configure as GPIO49 value."]
    Gpio49 = 0x03,
    #[doc = "Configure as the IOMSTR5 I2C SDA or SPI WIR3 signal value."]
    M5sdawir3 = 0x04,
    #[doc = "Configure as the IOMSTR5 SPI MISO input signal value."]
    M5miso = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad49fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad49fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad49fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad49fncsel {
        Pad49fncsel::from_bits(val)
    }
}
impl From<Pad49fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad49fncsel) -> u8 {
        Pad49fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad4fncsel {
    #[doc = "Configure as the UART0 CTS input signal value."]
    Ua0cts = 0x0,
    #[doc = "Configure as the IOSLAVE interrupt out signal value."]
    Slint = 0x01,
    #[doc = "IOM/SPI nCE group 4 value."]
    Nce4 = 0x02,
    #[doc = "Configure as GPIO4 value."]
    Gpio4 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Configure as the UART0 RX input value."]
    Uart0rx = 0x05,
    #[doc = "CTIMER connection 17 value."]
    Ct17 = 0x06,
    #[doc = "MSPI data connection 2 value."]
    Mspi2 = 0x07,
}
impl Pad4fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad4fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad4fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad4fncsel {
        Pad4fncsel::from_bits(val)
    }
}
impl From<Pad4fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad4fncsel) -> u8 {
        Pad4fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad5fncsel {
    #[doc = "Configure as the IOMSTR0 I2C SCL signal value."]
    M0scl = 0x0,
    #[doc = "Configure as the IOMSTR0 SPI SCK signal value."]
    M0sck = 0x01,
    #[doc = "Configure as the UART0 RTS signal output value."]
    Ua0rts = 0x02,
    #[doc = "Configure as GPIO5 value."]
    Gpio5 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "Configure as the External HFA input clock value."]
    Exthfa = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "CTIMER connection 8 value."]
    Ct8 = 0x07,
}
impl Pad5fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad5fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad5fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad5fncsel {
        Pad5fncsel::from_bits(val)
    }
}
impl From<Pad5fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad5fncsel) -> u8 {
        Pad5fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad6fncsel {
    #[doc = "Configure as the IOMSTR0 I2C SDA or SPI WIR3 signal value."]
    M0sdawir3 = 0x0,
    #[doc = "Configure as the IOMSTR0 SPI MISO signal value."]
    M0miso = 0x01,
    #[doc = "Configure as the UART0 CTS input signal value."]
    Ua0cts = 0x02,
    #[doc = "Configure as GPIO6 value."]
    Gpio6 = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "CTIMER connection 10 value."]
    Ct10 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "Configure as the PDM I2S Data output signal value."]
    I2sDat = 0x07,
}
impl Pad6fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad6fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad6fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad6fncsel {
        Pad6fncsel::from_bits(val)
    }
}
impl From<Pad6fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad6fncsel) -> u8 {
        Pad6fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad7fncsel {
    #[doc = "IOM/MSPI nCE group 7 value."]
    Nce7 = 0x0,
    #[doc = "Configure as the IOMSTR0 SPI MOSI signal value."]
    M0mosi = 0x01,
    #[doc = "Configure as the CLKOUT signal value."]
    Clkout = 0x02,
    #[doc = "Configure as GPIO7 value."]
    Gpio7 = 0x03,
    #[doc = "Configure as the ADC Trigger 0 signal value."]
    Trig0 = 0x04,
    #[doc = "Configure as the UART0 TX output signal value."]
    Uart0tx = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "CTIMER connection 19 value."]
    Ct19 = 0x07,
}
impl Pad7fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad7fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad7fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad7fncsel {
        Pad7fncsel::from_bits(val)
    }
}
impl From<Pad7fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad7fncsel) -> u8 {
        Pad7fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad8fncsel {
    #[doc = "Configure as the IOMSTR1 I2C SCL signal value."]
    M1scl = 0x0,
    #[doc = "Configure as the IOMSTR1 SPI SCK signal value."]
    M1sck = 0x01,
    #[doc = "IOM/MSPI nCE group 8 value."]
    Nce8 = 0x02,
    #[doc = "Configure as GPIO8 value."]
    Gpio8 = 0x03,
    #[doc = "SCARD serial clock output value."]
    Scclk = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Configure as the UART1 TX output signal value."]
    Uart1tx = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad8fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad8fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad8fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad8fncsel {
        Pad8fncsel::from_bits(val)
    }
}
impl From<Pad8fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad8fncsel) -> u8 {
        Pad8fncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pad9fncsel {
    #[doc = "Configure as the IOMSTR1 I2C SDA or SPI WIR3 signal value."]
    M1sdawir3 = 0x0,
    #[doc = "Configure as the IOMSTR1 SPI MISO signal value."]
    M1miso = 0x01,
    #[doc = "IOM/MSPI nCE group 9 value."]
    Nce9 = 0x02,
    #[doc = "Configure as GPIO9 value."]
    Gpio9 = 0x03,
    #[doc = "SCARD data I/O connection value."]
    Sccio = 0x04,
    _RESERVED_5 = 0x05,
    #[doc = "Configure as UART1 RX input signal value."]
    Uart1rx = 0x06,
    _RESERVED_7 = 0x07,
}
impl Pad9fncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pad9fncsel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pad9fncsel {
    #[inline(always)]
    fn from(val: u8) -> Pad9fncsel {
        Pad9fncsel::from_bits(val)
    }
}
impl From<Pad9fncsel> for u8 {
    #[inline(always)]
    fn from(val: Pad9fncsel) -> u8 {
        Pad9fncsel::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Padkey(u32);
impl Padkey {
    #[doc = "Key value."]
    pub const Key: Self = Self(0x73);
}
impl Padkey {
    pub const fn from_bits(val: u32) -> Padkey {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl core::fmt::Debug for Padkey {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.0 {
            0x73 => f.write_str("Key"),
            other => core::write!(f, "0x{:02X}", other),
        }
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Padkey {
    fn format(&self, f: defmt::Formatter) {
        match self.0 {
            0x73 => defmt::write!(f, "Key"),
            other => defmt::write!(f, "0x{:02X}", other),
        }
    }
}
impl From<u32> for Padkey {
    #[inline(always)]
    fn from(val: u32) -> Padkey {
        Padkey::from_bits(val)
    }
}
impl From<Padkey> for u32 {
    #[inline(always)]
    fn from(val: Padkey) -> u32 {
        Padkey::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Padrsel {
    #[doc = "Pullup is ~1.5 KOhms value."]
    Pull15k = 0x0,
    #[doc = "Pullup is ~6 KOhms value."]
    Pull6k = 0x01,
    #[doc = "Pullup is ~12 KOhms value."]
    Pull12k = 0x02,
    #[doc = "Pullup is ~24 KOhms value."]
    Pull24k = 0x03,
}
impl Padrsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Padrsel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Padrsel {
    #[inline(always)]
    fn from(val: u8) -> Padrsel {
        Padrsel::from_bits(val)
    }
}
impl From<Padrsel> for u8 {
    #[inline(always)]
    fn from(val: Padrsel) -> u8 {
        Padrsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Padstrng {
    #[doc = "Low drive strength value."]
    Low = 0x0,
    #[doc = "High drive strength value."]
    High = 0x01,
}
impl Padstrng {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Padstrng {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Padstrng {
    #[inline(always)]
    fn from(val: u8) -> Padstrng {
        Padstrng::from_bits(val)
    }
}
impl From<Padstrng> for u8 {
    #[inline(always)]
    fn from(val: Padstrng) -> u8 {
        Padstrng::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Stpol {
    #[doc = "Capture on low to high GPIO transition value."]
    Caplh = 0x0,
    #[doc = "Capture on high to low GPIO transition value."]
    Caphl = 0x01,
}
impl Stpol {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Stpol {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Stpol {
    #[inline(always)]
    fn from(val: u8) -> Stpol {
        Stpol::from_bits(val)
    }
}
impl From<Stpol> for u8 {
    #[inline(always)]
    fn from(val: Stpol) -> u8 {
        Stpol::to_bits(val)
    }
}
