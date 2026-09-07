#![no_std]
#![no_main]

use rot_bootloader::{Board, entry, println};

#[entry]
fn main(_b: Board) {
    println!("Hello world!");
}
