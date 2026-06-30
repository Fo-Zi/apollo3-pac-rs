#[doc = "I/O Slave Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "This bit selects the I/O interface."]
    #[must_use]
    #[inline(always)]
    pub const fn ifcsel(&self) -> super::vals::Ifcsel {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ifcsel::from_bits(val as u8)
    }
    #[doc = "This bit selects the I/O interface."]
    #[inline(always)]
    pub const fn set_ifcsel(&mut self, val: super::vals::Ifcsel) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "This bit selects SPI polarity."]
    #[must_use]
    #[inline(always)]
    pub const fn spol(&self) -> super::vals::Spol {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Spol::from_bits(val as u8)
    }
    #[doc = "This bit selects SPI polarity."]
    #[inline(always)]
    pub const fn set_spol(&mut self, val: super::vals::Spol) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "This bit selects the transfer bit ordering."]
    #[must_use]
    #[inline(always)]
    pub const fn lsb(&self) -> super::vals::Lsb {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Lsb::from_bits(val as u8)
    }
    #[doc = "This bit selects the transfer bit ordering."]
    #[inline(always)]
    pub const fn set_lsb(&mut self, val: super::vals::Lsb) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "This bit holds the cycle to initiate an I/O RAM read."]
    #[must_use]
    #[inline(always)]
    pub const fn startrd(&self) -> super::vals::Startrd {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Startrd::from_bits(val as u8)
    }
    #[doc = "This bit holds the cycle to initiate an I/O RAM read."]
    #[inline(always)]
    pub const fn set_startrd(&mut self, val: super::vals::Startrd) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "7-bit or 10-bit I2C device address."]
    #[must_use]
    #[inline(always)]
    pub const fn i2caddr(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "7-bit or 10-bit I2C device address."]
    #[inline(always)]
    pub const fn set_i2caddr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
    #[doc = "IOSLAVE interface enable."]
    #[must_use]
    #[inline(always)]
    pub const fn ifcen(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "IOSLAVE interface enable."]
    #[inline(always)]
    pub const fn set_ifcen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
            .field("ifcsel", &self.ifcsel())
            .field("spol", &self.spol())
            .field("lsb", &self.lsb())
            .field("startrd", &self.startrd())
            .field("i2caddr", &self.i2caddr())
            .field("ifcen", &self.ifcen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cfg {{ ifcsel: {:?}, spol: {:?}, lsb: {:?}, startrd: {:?}, i2caddr: {=u16:?}, ifcen: {=bool:?} }}" , self . ifcsel () , self . spol () , self . lsb () , self . startrd () , self . i2caddr () , self . ifcen ())
    }
}
#[doc = "FIFO Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifocfg(pub u32);
impl Fifocfg {
    #[doc = "These bits hold the base address of the I/O FIFO in 8 byte segments. The IO Slave FIFO is situated in LRAM at (FIFOBASE*8) to (FIFOMAX*8-1)."]
    #[must_use]
    #[inline(always)]
    pub const fn fifobase(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits hold the base address of the I/O FIFO in 8 byte segments. The IO Slave FIFO is situated in LRAM at (FIFOBASE*8) to (FIFOMAX*8-1)."]
    #[inline(always)]
    pub const fn set_fifobase(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "These bits hold the maximum FIFO address in 8 byte segments. It is also the beginning of the RAM area of the LRAM. Note that no RAM area is configured if FIFOMAX is set to 0x1F."]
    #[must_use]
    #[inline(always)]
    pub const fn fifomax(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "These bits hold the maximum FIFO address in 8 byte segments. It is also the beginning of the RAM area of the LRAM. Note that no RAM area is configured if FIFOMAX is set to 0x1F."]
    #[inline(always)]
    pub const fn set_fifomax(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Defines the read-only area. The IO Slave read-only area is situated in LRAM at (ROBASE*8) to (FIFOBASE*8-1)."]
    #[must_use]
    #[inline(always)]
    pub const fn robase(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "Defines the read-only area. The IO Slave read-only area is situated in LRAM at (ROBASE*8) to (FIFOBASE*8-1)."]
    #[inline(always)]
    pub const fn set_robase(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for Fifocfg {
    #[inline(always)]
    fn default() -> Fifocfg {
        Fifocfg(0)
    }
}
impl core::fmt::Debug for Fifocfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifocfg")
            .field("fifobase", &self.fifobase())
            .field("fifomax", &self.fifomax())
            .field("robase", &self.robase())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifocfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fifocfg {{ fifobase: {=u8:?}, fifomax: {=u8:?}, robase: {=u8:?} }}",
            self.fifobase(),
            self.fifomax(),
            self.robase()
        )
    }
}
#[doc = "Overall FIFO Counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifoctr(pub u32);
impl Fifoctr {
    #[doc = "Virtual FIFO byte count."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoctr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Virtual FIFO byte count."]
    #[inline(always)]
    pub const fn set_fifoctr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Fifoctr {
    #[inline(always)]
    fn default() -> Fifoctr {
        Fifoctr(0)
    }
}
impl core::fmt::Debug for Fifoctr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifoctr")
            .field("fifoctr", &self.fifoctr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifoctr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fifoctr {{ fifoctr: {=u16:?} }}", self.fifoctr())
    }
}
#[doc = "Overall FIFO Counter Increment."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifoinc(pub u32);
impl Fifoinc {
    #[doc = "Increment the Overall FIFO Counter by this value on a write."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoinc(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Increment the Overall FIFO Counter by this value on a write."]
    #[inline(always)]
    pub const fn set_fifoinc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for Fifoinc {
    #[inline(always)]
    fn default() -> Fifoinc {
        Fifoinc(0)
    }
}
impl core::fmt::Debug for Fifoinc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifoinc")
            .field("fifoinc", &self.fifoinc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifoinc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fifoinc {{ fifoinc: {=u16:?} }}", self.fifoinc())
    }
}
#[doc = "Current FIFO Pointer."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifoptr(pub u32);
impl Fifoptr {
    #[doc = "Current FIFO pointer."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoptr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Current FIFO pointer."]
    #[inline(always)]
    pub const fn set_fifoptr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "The number of bytes currently in the hardware FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn fifosiz(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "The number of bytes currently in the hardware FIFO."]
    #[inline(always)]
    pub const fn set_fifosiz(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
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
            .field("fifoptr", &self.fifoptr())
            .field("fifosiz", &self.fifosiz())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifoptr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fifoptr {{ fifoptr: {=u8:?}, fifosiz: {=u8:?} }}",
            self.fifoptr(),
            self.fifosiz()
        )
    }
}
#[doc = "FIFO Threshold Configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifothr(pub u32);
impl Fifothr {
    #[doc = "FIFO size interrupt threshold."]
    #[must_use]
    #[inline(always)]
    pub const fn fifothr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "FIFO size interrupt threshold."]
    #[inline(always)]
    pub const fn set_fifothr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
#[doc = "FIFO Update Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fupd(pub u32);
impl Fupd {
    #[doc = "This bit indicates that a FIFO update is underway."]
    #[must_use]
    #[inline(always)]
    pub const fn fifoupd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates that a FIFO update is underway."]
    #[inline(always)]
    pub const fn set_fifoupd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This bitfield indicates an IO read is active."]
    #[must_use]
    #[inline(always)]
    pub const fn ioread(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This bitfield indicates an IO read is active."]
    #[inline(always)]
    pub const fn set_ioread(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Fupd {
    #[inline(always)]
    fn default() -> Fupd {
        Fupd(0)
    }
}
impl core::fmt::Debug for Fupd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fupd")
            .field("fifoupd", &self.fifoupd())
            .field("ioread", &self.ioread())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fupd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Fupd {{ fifoupd: {=bool:?}, ioread: {=bool:?} }}",
            self.fifoupd(),
            self.ioread()
        )
    }
}
#[doc = "General Address Data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Genadd(pub u32);
impl Genadd {
    #[doc = "The data supplied on the last General Address reference."]
    #[must_use]
    #[inline(always)]
    pub const fn gadata(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The data supplied on the last General Address reference."]
    #[inline(always)]
    pub const fn set_gadata(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Genadd {
    #[inline(always)]
    fn default() -> Genadd {
        Genadd(0)
    }
}
impl core::fmt::Debug for Genadd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Genadd")
            .field("gadata", &self.gadata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Genadd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Genadd {{ gadata: {=u8:?} }}", self.gadata())
    }
}
#[doc = "IO Slave Interrupts: Clear."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intclr(pub u32);
impl Intclr {
    #[doc = "FIFO Size interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fsize(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Size interrupt."]
    #[inline(always)]
    pub const fn set_fsize(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Overflow interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fovfl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Overflow interrupt."]
    #[inline(always)]
    pub const fn set_fovfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO Underflow interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fundfl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Underflow interrupt."]
    #[inline(always)]
    pub const fn set_fundfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FIFO Read Error interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn frderr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Read Error interrupt."]
    #[inline(always)]
    pub const fn set_frderr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "I2C General Address interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn genad(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "I2C General Address interrupt."]
    #[inline(always)]
    pub const fn set_genad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IO Write interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn iointw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IO Write interrupt."]
    #[inline(always)]
    pub const fn set_iointw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transfer complete interrupt, read from FIFO space."]
    #[must_use]
    #[inline(always)]
    pub const fn xcmprf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt, read from FIFO space."]
    #[inline(always)]
    pub const fn set_xcmprf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transfer complete interrupt, read from register space."]
    #[must_use]
    #[inline(always)]
    pub const fn xcmprr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt, read from register space."]
    #[inline(always)]
    pub const fn set_xcmprr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Transfer complete interrupt, write to FIFO space."]
    #[must_use]
    #[inline(always)]
    pub const fn xcmpwf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt, write to FIFO space."]
    #[inline(always)]
    pub const fn set_xcmpwf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transfer complete interrupt, write to register space."]
    #[must_use]
    #[inline(always)]
    pub const fn xcmpwr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt, write to register space."]
    #[inline(always)]
    pub const fn set_xcmpwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
            .field("fsize", &self.fsize())
            .field("fovfl", &self.fovfl())
            .field("fundfl", &self.fundfl())
            .field("frderr", &self.frderr())
            .field("genad", &self.genad())
            .field("iointw", &self.iointw())
            .field("xcmprf", &self.xcmprf())
            .field("xcmprr", &self.xcmprr())
            .field("xcmpwf", &self.xcmpwf())
            .field("xcmpwr", &self.xcmpwr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intclr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Intclr {{ fsize: {=bool:?}, fovfl: {=bool:?}, fundfl: {=bool:?}, frderr: {=bool:?}, genad: {=bool:?}, iointw: {=bool:?}, xcmprf: {=bool:?}, xcmprr: {=bool:?}, xcmpwf: {=bool:?}, xcmpwr: {=bool:?} }}" , self . fsize () , self . fovfl () , self . fundfl () , self . frderr () , self . genad () , self . iointw () , self . xcmprf () , self . xcmprr () , self . xcmpwf () , self . xcmpwr ())
    }
}
#[doc = "IO Slave Interrupts: Enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "FIFO Size interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fsize(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Size interrupt."]
    #[inline(always)]
    pub const fn set_fsize(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Overflow interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fovfl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Overflow interrupt."]
    #[inline(always)]
    pub const fn set_fovfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO Underflow interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fundfl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Underflow interrupt."]
    #[inline(always)]
    pub const fn set_fundfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FIFO Read Error interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn frderr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Read Error interrupt."]
    #[inline(always)]
    pub const fn set_frderr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "I2C General Address interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn genad(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "I2C General Address interrupt."]
    #[inline(always)]
    pub const fn set_genad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IO Write interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn iointw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IO Write interrupt."]
    #[inline(always)]
    pub const fn set_iointw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transfer complete interrupt, read from FIFO space."]
    #[must_use]
    #[inline(always)]
    pub const fn xcmprf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt, read from FIFO space."]
    #[inline(always)]
    pub const fn set_xcmprf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transfer complete interrupt, read from register space."]
    #[must_use]
    #[inline(always)]
    pub const fn xcmprr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt, read from register space."]
    #[inline(always)]
    pub const fn set_xcmprr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Transfer complete interrupt, write to FIFO space."]
    #[must_use]
    #[inline(always)]
    pub const fn xcmpwf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt, write to FIFO space."]
    #[inline(always)]
    pub const fn set_xcmpwf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transfer complete interrupt, write to register space."]
    #[must_use]
    #[inline(always)]
    pub const fn xcmpwr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt, write to register space."]
    #[inline(always)]
    pub const fn set_xcmpwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
            .field("fsize", &self.fsize())
            .field("fovfl", &self.fovfl())
            .field("fundfl", &self.fundfl())
            .field("frderr", &self.frderr())
            .field("genad", &self.genad())
            .field("iointw", &self.iointw())
            .field("xcmprf", &self.xcmprf())
            .field("xcmprr", &self.xcmprr())
            .field("xcmpwf", &self.xcmpwf())
            .field("xcmpwr", &self.xcmpwr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inten {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Inten {{ fsize: {=bool:?}, fovfl: {=bool:?}, fundfl: {=bool:?}, frderr: {=bool:?}, genad: {=bool:?}, iointw: {=bool:?}, xcmprf: {=bool:?}, xcmprr: {=bool:?}, xcmpwf: {=bool:?}, xcmpwr: {=bool:?} }}" , self . fsize () , self . fovfl () , self . fundfl () , self . frderr () , self . genad () , self . iointw () , self . xcmprf () , self . xcmprr () , self . xcmpwf () , self . xcmpwr ())
    }
}
#[doc = "IO Slave Interrupts: Set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intset(pub u32);
impl Intset {
    #[doc = "FIFO Size interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fsize(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Size interrupt."]
    #[inline(always)]
    pub const fn set_fsize(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Overflow interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fovfl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Overflow interrupt."]
    #[inline(always)]
    pub const fn set_fovfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO Underflow interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fundfl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Underflow interrupt."]
    #[inline(always)]
    pub const fn set_fundfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FIFO Read Error interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn frderr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Read Error interrupt."]
    #[inline(always)]
    pub const fn set_frderr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "I2C General Address interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn genad(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "I2C General Address interrupt."]
    #[inline(always)]
    pub const fn set_genad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IO Write interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn iointw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IO Write interrupt."]
    #[inline(always)]
    pub const fn set_iointw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transfer complete interrupt, read from FIFO space."]
    #[must_use]
    #[inline(always)]
    pub const fn xcmprf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt, read from FIFO space."]
    #[inline(always)]
    pub const fn set_xcmprf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transfer complete interrupt, read from register space."]
    #[must_use]
    #[inline(always)]
    pub const fn xcmprr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt, read from register space."]
    #[inline(always)]
    pub const fn set_xcmprr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Transfer complete interrupt, write to FIFO space."]
    #[must_use]
    #[inline(always)]
    pub const fn xcmpwf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt, write to FIFO space."]
    #[inline(always)]
    pub const fn set_xcmpwf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transfer complete interrupt, write to register space."]
    #[must_use]
    #[inline(always)]
    pub const fn xcmpwr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt, write to register space."]
    #[inline(always)]
    pub const fn set_xcmpwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
            .field("fsize", &self.fsize())
            .field("fovfl", &self.fovfl())
            .field("fundfl", &self.fundfl())
            .field("frderr", &self.frderr())
            .field("genad", &self.genad())
            .field("iointw", &self.iointw())
            .field("xcmprf", &self.xcmprf())
            .field("xcmprr", &self.xcmprr())
            .field("xcmpwf", &self.xcmpwf())
            .field("xcmpwr", &self.xcmpwr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intset {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Intset {{ fsize: {=bool:?}, fovfl: {=bool:?}, fundfl: {=bool:?}, frderr: {=bool:?}, genad: {=bool:?}, iointw: {=bool:?}, xcmprf: {=bool:?}, xcmprr: {=bool:?}, xcmpwf: {=bool:?}, xcmpwr: {=bool:?} }}" , self . fsize () , self . fovfl () , self . fundfl () , self . frderr () , self . genad () , self . iointw () , self . xcmprf () , self . xcmprr () , self . xcmpwf () , self . xcmpwr ())
    }
}
#[doc = "IO Slave Interrupts: Status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "FIFO Size interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fsize(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Size interrupt."]
    #[inline(always)]
    pub const fn set_fsize(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO Overflow interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fovfl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Overflow interrupt."]
    #[inline(always)]
    pub const fn set_fovfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO Underflow interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn fundfl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Underflow interrupt."]
    #[inline(always)]
    pub const fn set_fundfl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "FIFO Read Error interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn frderr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO Read Error interrupt."]
    #[inline(always)]
    pub const fn set_frderr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "I2C General Address interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn genad(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "I2C General Address interrupt."]
    #[inline(always)]
    pub const fn set_genad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "IO Write interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn iointw(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IO Write interrupt."]
    #[inline(always)]
    pub const fn set_iointw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Transfer complete interrupt, read from FIFO space."]
    #[must_use]
    #[inline(always)]
    pub const fn xcmprf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt, read from FIFO space."]
    #[inline(always)]
    pub const fn set_xcmprf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transfer complete interrupt, read from register space."]
    #[must_use]
    #[inline(always)]
    pub const fn xcmprr(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt, read from register space."]
    #[inline(always)]
    pub const fn set_xcmprr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Transfer complete interrupt, write to FIFO space."]
    #[must_use]
    #[inline(always)]
    pub const fn xcmpwf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt, write to FIFO space."]
    #[inline(always)]
    pub const fn set_xcmpwf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Transfer complete interrupt, write to register space."]
    #[must_use]
    #[inline(always)]
    pub const fn xcmpwr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer complete interrupt, write to register space."]
    #[inline(always)]
    pub const fn set_xcmpwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
            .field("fsize", &self.fsize())
            .field("fovfl", &self.fovfl())
            .field("fundfl", &self.fundfl())
            .field("frderr", &self.frderr())
            .field("genad", &self.genad())
            .field("iointw", &self.iointw())
            .field("xcmprf", &self.xcmprf())
            .field("xcmprr", &self.xcmprr())
            .field("xcmpwf", &self.xcmpwf())
            .field("xcmpwr", &self.xcmpwr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intstat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Intstat {{ fsize: {=bool:?}, fovfl: {=bool:?}, fundfl: {=bool:?}, frderr: {=bool:?}, genad: {=bool:?}, iointw: {=bool:?}, xcmprf: {=bool:?}, xcmprr: {=bool:?}, xcmpwf: {=bool:?}, xcmpwr: {=bool:?} }}" , self . fsize () , self . fovfl () , self . fundfl () , self . frderr () , self . genad () , self . iointw () , self . xcmprf () , self . xcmprr () , self . xcmpwf () , self . xcmpwr ())
    }
}
#[doc = "I/O Interrupt Control."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iointctl(pub u32);
impl Iointctl {
    #[doc = "These read-only bits indicate whether the IOINT interrupts are enabled."]
    #[must_use]
    #[inline(always)]
    pub const fn iointen(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These read-only bits indicate whether the IOINT interrupts are enabled."]
    #[inline(always)]
    pub const fn set_iointen(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "These bits read the IOINT interrupts."]
    #[must_use]
    #[inline(always)]
    pub const fn ioint(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read the IOINT interrupts."]
    #[inline(always)]
    pub const fn set_ioint(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "This bit clears all of the IOINT interrupts when written with a 1."]
    #[must_use]
    #[inline(always)]
    pub const fn iointclr(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "This bit clears all of the IOINT interrupts when written with a 1."]
    #[inline(always)]
    pub const fn set_iointclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "These bits set the IOINT interrupts when written with a 1."]
    #[must_use]
    #[inline(always)]
    pub const fn iointset(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "These bits set the IOINT interrupts when written with a 1."]
    #[inline(always)]
    pub const fn set_iointset(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Iointctl {
    #[inline(always)]
    fn default() -> Iointctl {
        Iointctl(0)
    }
}
impl core::fmt::Debug for Iointctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Iointctl")
            .field("iointen", &self.iointen())
            .field("ioint", &self.ioint())
            .field("iointclr", &self.iointclr())
            .field("iointset", &self.iointset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Iointctl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Iointctl {{ iointen: {=u8:?}, ioint: {=u8:?}, iointclr: {=bool:?}, iointset: {=u8:?} }}" , self . iointen () , self . ioint () , self . iointclr () , self . iointset ())
    }
}
#[doc = "I/O Slave Interrupt Priority Encode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prenc(pub u32);
impl Prenc {
    #[doc = "These bits hold the priority encode of the REGACC interrupts."]
    #[must_use]
    #[inline(always)]
    pub const fn prenc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "These bits hold the priority encode of the REGACC interrupts."]
    #[inline(always)]
    pub const fn set_prenc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Prenc {
    #[inline(always)]
    fn default() -> Prenc {
        Prenc(0)
    }
}
impl core::fmt::Debug for Prenc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Prenc")
            .field("prenc", &self.prenc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Prenc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Prenc {{ prenc: {=u8:?} }}", self.prenc())
    }
}
