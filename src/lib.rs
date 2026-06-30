#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![doc = "Peripheral access API (generated using chiptool v0.1.0 (bcf538a 2026-05-18))"]
#![no_std]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Interrupt {
    #[doc = "0 - BROWNOUT"]
    BROWNOUT = 0,
    #[doc = "1 - WDT"]
    WDT = 1,
    #[doc = "2 - RTC"]
    RTC = 2,
    #[doc = "3 - VCOMP"]
    VCOMP = 3,
    #[doc = "4 - IOSLAVE"]
    IOSLAVE = 4,
    #[doc = "5 - IOSLAVEACC"]
    IOSLAVEACC = 5,
    #[doc = "6 - IOMSTR0"]
    IOMSTR0 = 6,
    #[doc = "7 - IOMSTR1"]
    IOMSTR1 = 7,
    #[doc = "8 - IOMSTR2"]
    IOMSTR2 = 8,
    #[doc = "9 - IOMSTR3"]
    IOMSTR3 = 9,
    #[doc = "10 - IOMSTR4"]
    IOMSTR4 = 10,
    #[doc = "11 - IOMSTR5"]
    IOMSTR5 = 11,
    #[doc = "12 - BLE"]
    BLE = 12,
    #[doc = "13 - GPIO"]
    GPIO = 13,
    #[doc = "14 - CTIMER"]
    CTIMER = 14,
    #[doc = "15 - UART0"]
    UART0 = 15,
    #[doc = "16 - UART1"]
    UART1 = 16,
    #[doc = "17 - SCARD"]
    SCARD = 17,
    #[doc = "18 - ADC"]
    ADC = 18,
    #[doc = "19 - PDM"]
    PDM = 19,
    #[doc = "20 - MSPI"]
    MSPI = 20,
    #[doc = "22 - STIMER"]
    STIMER = 22,
    #[doc = "23 - STIMER_CMPR0"]
    STIMER_CMPR0 = 23,
    #[doc = "24 - STIMER_CMPR1"]
    STIMER_CMPR1 = 24,
    #[doc = "25 - STIMER_CMPR2"]
    STIMER_CMPR2 = 25,
    #[doc = "26 - STIMER_CMPR3"]
    STIMER_CMPR3 = 26,
    #[doc = "27 - STIMER_CMPR4"]
    STIMER_CMPR4 = 27,
    #[doc = "28 - STIMER_CMPR5"]
    STIMER_CMPR5 = 28,
    #[doc = "29 - STIMER_CMPR6"]
    STIMER_CMPR6 = 29,
    #[doc = "30 - STIMER_CMPR7"]
    STIMER_CMPR7 = 30,
    #[doc = "31 - CLKGEN"]
    CLKGEN = 31,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
#[doc = "MCU Reset Generator"]
pub const RSTGEN: rstgen::Rstgen = unsafe { rstgen::Rstgen::from_ptr(0x4000_0000usize as _) };
#[doc = "Clock Generator"]
pub const CLKGEN: clkgen::Clkgen = unsafe { clkgen::Clkgen::from_ptr(0x4000_4000usize as _) };
#[doc = "Real Time Clock"]
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_4200usize as _) };
#[doc = "Counter/Timer"]
pub const CTIMER: ctimer::Ctimer = unsafe { ctimer::Ctimer::from_ptr(0x4000_8000usize as _) };
#[doc = "Voltage Comparator"]
pub const VCOMP: vcomp::Vcomp = unsafe { vcomp::Vcomp::from_ptr(0x4000_c000usize as _) };
#[doc = "General Purpose IO"]
pub const GPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4001_0000usize as _) };
#[doc = "APB DMA Register Interfaces"]
pub const APBDMA: apbdma::Apbdma = unsafe { apbdma::Apbdma::from_ptr(0x4001_1000usize as _) };
#[doc = "Flash Cache Controller"]
pub const CACHECTRL: cachectrl::Cachectrl =
    unsafe { cachectrl::Cachectrl::from_ptr(0x4001_8000usize as _) };
#[doc = "Serial UART"]
pub const UART0: uart0::Uart0 = unsafe { uart0::Uart0::from_ptr(0x4001_c000usize as _) };
pub const UART1: uart0::Uart0 = unsafe { uart0::Uart0::from_ptr(0x4001_d000usize as _) };
#[doc = "MCU Miscellaneous Control Logic"]
pub const MCUCTRL: mcuctrl::Mcuctrl = unsafe { mcuctrl::Mcuctrl::from_ptr(0x4002_0000usize as _) };
#[doc = "PWR Controller Register Bank"]
pub const PWRCTRL: pwrctrl::Pwrctrl = unsafe { pwrctrl::Pwrctrl::from_ptr(0x4002_1000usize as _) };
#[doc = "Watchdog Timer"]
pub const WDT: wdt::Wdt = unsafe { wdt::Wdt::from_ptr(0x4002_4000usize as _) };
#[doc = "Security Interfaces"]
pub const SECURITY: security::Security =
    unsafe { security::Security::from_ptr(0x4003_0000usize as _) };
#[doc = "Serial ISO7816"]
pub const SCARD: scard::Scard = unsafe { scard::Scard::from_ptr(0x4008_0000usize as _) };
#[doc = "I2C/SPI Slave"]
pub const IOSLAVE: ioslave::Ioslave = unsafe { ioslave::Ioslave::from_ptr(0x5000_0000usize as _) };
#[doc = "IO Peripheral Master"]
pub const IOM0: iom0::Iom0 = unsafe { iom0::Iom0::from_ptr(0x5000_4000usize as _) };
pub const IOM1: iom0::Iom0 = unsafe { iom0::Iom0::from_ptr(0x5000_5000usize as _) };
pub const IOM2: iom0::Iom0 = unsafe { iom0::Iom0::from_ptr(0x5000_6000usize as _) };
pub const IOM3: iom0::Iom0 = unsafe { iom0::Iom0::from_ptr(0x5000_7000usize as _) };
pub const IOM4: iom0::Iom0 = unsafe { iom0::Iom0::from_ptr(0x5000_8000usize as _) };
pub const IOM5: iom0::Iom0 = unsafe { iom0::Iom0::from_ptr(0x5000_9000usize as _) };
#[doc = "BLE Interface"]
pub const BLEIF: bleif::Bleif = unsafe { bleif::Bleif::from_ptr(0x5000_c000usize as _) };
#[doc = "Analog Digital Converter Control"]
pub const ADC: adc::Adc = unsafe { adc::Adc::from_ptr(0x5001_0000usize as _) };
#[doc = "PDM Audio"]
pub const PDM: pdm::Pdm = unsafe { pdm::Pdm::from_ptr(0x5001_1000usize as _) };
#[doc = "Multibit SPI Master"]
pub const MSPI: mspi::Mspi = unsafe { mspi::Mspi::from_ptr(0x5001_4000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub mod adc;
pub mod apbdma;
pub mod bleif;
pub mod cachectrl;
pub mod clkgen;
pub mod common;
pub mod ctimer;
pub mod gpio;
pub mod iom0;
pub mod ioslave;
pub mod mcuctrl;
pub mod mspi;
pub mod pdm;
pub mod pwrctrl;
pub mod rstgen;
pub mod rtc;
pub mod scard;
pub mod security;
pub mod uart0;
pub mod vcomp;
pub mod wdt;
