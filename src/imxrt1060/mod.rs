//! Pads for the i.MX RT 1060 processor family
//!
//! The module exports all of the i.MX RT 1060 processor's pads. Pads that can support peripheral
//! functions are tagged with `imxrt-iomuxc` traits.
//!
//! # Example
//!
//! In the example below, we implement a hypothetical `uart_new` function, which is responsible
//! for preparing a UART peripheral. To properly configure the peripheral, we need the two
//! pads that represent a peripheral's TX and RX pins. The implementation will use the
//! `imxrt_iomuxc::lpuart::prepare()` function to prepare the pins.
//!
//! Note the trait bounds on `uart_new`. The usage requires that
//!
//! - the user provides one TX and one RX pin
//! - the modules for each pin match
//!
//! ```no_run
//! use imxrt_iomuxc as iomuxc;
//! use iomuxc::lpuart::{Pin, Tx, Rx};
//!
//! # struct UART;
//! /// Creates a UART peripheral from the TX and RX pads, and a baud rate
//! fn uart_new<T, R>(mut tx: T, mut rx: R, baud: u32) -> UART
//! where
//!     T: Pin<Direction = Tx>,
//!     R: Pin<Direction = Rx, Module = <T as Pin>::Module>,
//! {
//!     // Check the imxrt-iomuxc documentation to understand why
//!     // this is unsafe.
//!     unsafe {
//!         iomuxc::lpuart::prepare(&mut tx);
//!         iomuxc::lpuart::prepare(&mut rx);
//!     }
//!     // Prepare the rest of the UART peripheral, and return it...
//!     # UART
//! }
//!
//! # let gpio_ad_b0_13 = unsafe { imxrt_iomuxc::imxrt1060::gpio_ad_b0::GPIO_AD_B0_13::new() };
//! # let gpio_ad_b0_12 = unsafe { imxrt_iomuxc::imxrt1060::gpio_ad_b0::GPIO_AD_B0_12::new() };
//! // GPIO_AD_B0_13 and GPIO_AD_B0_12 are a suitable pair of UART pins
//! uart_new(gpio_ad_b0_12, gpio_ad_b0_13, 115_200);
//! ```
//!
//! Specifically, the trait bounds guard against non-UART pins:
//!
//! ```compile_fail
//! # use imxrt_iomuxc as iomuxc;
//! # use iomuxc::lpuart::{Pin, Tx, Rx};
//! # struct UART;
//! # fn uart_new<T, R>(mut tx: T, mut rx: R, baud: u32) -> UART
//! # where
//! #     T: Pin<Direction = Tx>,
//! #     R: Pin<Direction = Rx, Module = <T as Pin>::Module>,
//! # {
//! #     unsafe {
//! #         iomuxc::lpuart::prepare(&mut tx);
//! #         iomuxc::lpuart::prepare(&mut rx);
//! #     }
//! #     UART
//! # }
//! # let gpio_ad_b0_10 = unsafe { imxrt_iomuxc::imxrt1060::gpio_ad_b0::GPIO_AD_B0_10::new() };
//! # let gpio_ad_b0_11 = unsafe { imxrt_iomuxc::imxrt1060::gpio_ad_b0::GPIO_AD_B0_11::new() };
//! // Neither pad is a valid UART pin
//! uart_new(gpio_ad_b0_10, gpio_ad_b0_11, 115_200);
//! ```
//!
//! It also guards against mismatched UART pins:
//!
//! ```compile_fail
//! # use imxrt_iomuxc as iomuxc;
//! # use iomuxc::lpuart::{Pin, Tx, Rx};
//! # struct UART;
//! # fn uart_new<T, R>(mut tx: T, mut rx: R, baud: u32) -> UART
//! # where
//! #     T: Pin<Direction = Tx>,
//! #     R: Pin<Direction = Rx, Module = <T as Pin>::Module>,
//! # {
//! #     unsafe {
//! #         iomuxc::lpuart::prepare(&mut tx);
//! #         iomuxc::lpuart::prepare(&mut rx);
//! #     }
//! #     UART
//! # }
//! # let gpio_ad_b0_13 = unsafe { imxrt_iomuxc::imxrt1060::gpio_ad_b0::GPIO_AD_B0_13::new() };
//! # let gpio_ad_b1_02 = unsafe { imxrt_iomuxc::imxrt1060::gpio_ad_b0::GPIO_AD_B1_02::new() };
//! // GPIO_AD_B1_02 is a UART2 TX pin, but GPIO_AD_B0_13 is a UART1 RX pin
//! uart_new(gpio_ad_b1_02, gpio_ad_b0_13, 115_200);
//! ```

mod adc;
mod flexpwm;
mod lpi2c;
mod lpspi;
mod lpuart;
mod sai;
include!(concat!(env!("OUT_DIR"), "/imxrt1060.rs"));
pub use pads::*;

mod bases {
    define_base!(GPIO_EMC, 0x401F_8014, 0x401F_8204);
    define_base!(GPIO_AD_B0, 0x401F_80BC, 0x401F_82AC);
    define_base!(GPIO_AD_B1, 0x401F_80FC, 0x401F_82EC);
    define_base!(GPIO_B0, 0x401F_813C, 0x401F_832C);
    define_base!(GPIO_B1, 0x401F_817C, 0x401F_836C);
    define_base!(GPIO_SD_B0, 0x401F_81BC, 0x401F_83AC);
    define_base!(GPIO_SD_B1, 0x401F_81D4, 0x401F_83C4);
}
