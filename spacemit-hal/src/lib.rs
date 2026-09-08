//! Peripheral abstractions for SpacemiT K1/M1 and K3 SoCs.

#![no_std]
#![deny(missing_docs)]

pub mod adma;
pub mod ahbdma;
pub mod apbc;
pub mod apbc2;
pub mod apbs;
pub mod aplic;
pub mod apmu;
pub mod can;
pub mod ccic;
pub mod ciu;
pub mod clock;
pub mod counter;
pub mod dciu;
pub mod dpu;
pub mod dsi;
pub mod emac;
pub mod espi;
pub mod gpio;
pub mod hdma;
pub mod hsio_phy;
pub mod i2c;
pub mod iopmp;
pub mod ir;
pub mod mailbox;
pub mod mfpr;
pub mod mpmu;
pub mod onewire;
pub mod pcie;
pub mod pdma;
pub mod plic;
pub mod prelude;
pub mod pwm;
pub mod qspi;
pub mod rcpu;
pub mod register;
pub mod ri2s;
pub mod rpmu;
pub mod rtc;
pub mod sdh;
pub mod sec_ciu;
pub mod spi;
pub mod spinlock;
pub mod timer;
pub mod trng;
pub mod tsensor;
pub mod uart;
pub mod ufs;
pub mod usb2;
pub mod usb2_phy;
pub mod usb3;
pub mod v2d;
pub mod vpu;
