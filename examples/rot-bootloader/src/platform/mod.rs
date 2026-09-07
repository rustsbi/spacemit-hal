//! Board-specific initialization.

pub mod muse_card_m1;

pub use muse_card_m1::Board;

#[doc(hidden)]
pub use muse_card_m1::init_board;
