//! K1 v0.2 DDR training and destructive pre-boot memory checks.

use core::{
    arch::{asm, naked_asm},
    ffi::{CStr, c_char},
    ptr,
};
use embedded_io::Write;

// Firmware: spacemit-com/spacemit-firmware, commit 5969642a5b46fee5ba7f21ca54c3129bff7bb049.
// ABI/address: firmware README -> https://lists.denx.de/pipermail/u-boot/2026-February/609826.html
// Cache operations: vendor U-Boot arch/riscv/cpu/x60/cache.c (64-byte lines).
// Loaded directly at its execution address; the blob owns mutable in-place data.
#[unsafe(link_section = ".ddr_firmware")]
static mut FIRMWARE: [u8; 36_248] =
    *include_bytes!("../../../vendor/spacemit-firmware/k1/v0.2/ddr_fw.bin");

unsafe extern "C" {
    static __ddr_info: u8;
    static __ddr_fw_end: u8;
    static __ddr_stack_bottom: u8;
    static __ddr_stack_top: u8;
}

const DRAM_START: usize = 0x0080_0000;
const DRAM_END: usize = 0x0800_0000;
const TEST_SIZE: usize = 0x0010_0000;
const STACK_PATTERN: u8 = 0xa5;

pub(crate) struct Config {
    pub chip_selects: u32,
    pub data_rate: u32,
}

/// DDR training or memory verification failed.
#[derive(Debug, ufmt::derive::uDebug)]
pub enum Error {
    /// EEPROM parameters differ from the supported board configuration.
    Config,
    /// Training exhausted its reserved stack guard.
    Stack,
    /// A physical memory read did not match its test pattern.
    Memory {
        address: usize,
        expected: u64,
        actual: u64,
    },
}

// SAFETY contract is inherited from Board::init_ddr; no DRAM objects exist yet.
pub(crate) unsafe fn init(config: &Config) -> Result<i32, Error> {
    // SAFETY: The caller reserves the firmware's SRAM and private stack.
    unsafe { prepare_training() };
    let bottom = ptr::addr_of!(__ddr_stack_bottom) as usize;
    let top = ptr::addr_of!(__ddr_stack_top) as usize;
    // SAFETY: Fixed, hash-checked C ABI; clocks and rails are ready on the boot hart.
    let firmware_status =
        unsafe { train(0xc000_0000, config.chip_selects, config.data_rate, puts) };
    // SAFETY: Training has returned; its private stack has no remaining users.
    unsafe { flush(bottom, top) };
    for offset in 0..256 {
        // SAFETY: This byte lies in the initialized, exclusively reserved stack.
        if unsafe { ptr::read_volatile((bottom + offset) as *const u8) } != STACK_PATTERN {
            return Err(Error::Stack);
        }
    }
    // SAFETY: No payload, allocator, DMA or secondary hart uses the test windows.
    unsafe {
        test_memory()?;
        publish_config(config);
    }
    Ok(firmware_status)
}

// Vendor k1-x.h ddr_training_info_t and spl.c update_ddr_config_info.
// Old U-Boot reads CS count here; this record does not contain reusable training results.
unsafe fn publish_config(config: &Config) {
    let address = ptr::addr_of!(__ddr_info) as usize;
    // SAFETY: BootROM no longer uses this header; link.x reserves all 1536 bytes.
    let bytes = unsafe { core::slice::from_raw_parts_mut(address as *mut u8, 1536) };
    bytes.fill(0);
    for (offset, value) in [
        (0, 0x5452_4444u32),
        (24, 0x0001_0000),
        (28, config.chip_selects),
        (32, config.data_rate),
    ] {
        bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    bytes[36..43].copy_from_slice(b"LPDDR4x");
    let crc = crate::image::layout::crc32(&bytes[8..]);
    bytes[4..8].copy_from_slice(&crc.to_le_bytes());
    // SAFETY: The complete metadata region is aligned and exclusively reserved SRAM.
    unsafe { flush(address, address + 1536) };
}

#[inline]
unsafe fn prepare_training() {
    let firmware = ptr::addr_of!(FIRMWARE) as usize;
    let tail = firmware + core::mem::size_of::<[u8; 36_248]>();
    let workspace_end = ptr::addr_of!(__ddr_fw_end) as usize;
    let bottom = ptr::addr_of!(__ddr_stack_bottom) as usize;
    let top = ptr::addr_of!(__ddr_stack_top) as usize;
    // SAFETY: link.x keeps the firmware tail and private stack outside all live sections.
    unsafe {
        ptr::write_bytes(tail as *mut u8, 0, workspace_end - tail);
        ptr::write_bytes(bottom as *mut u8, STACK_PATTERN, top - bottom);
        flush(firmware, workspace_end);
        flush(bottom, top);
    }
    riscv::asm::fence_i();
}

// The blob uses the C integer ABI; save the caller's stack, gp and tp across it.
// s0 is callee-saved, so it can retain the old frame while sp uses DDR_STACK.
#[unsafe(naked)]
unsafe extern "C" fn train(
    _controller: u64,
    _chip_selects: u32,
    _data_rate: u32,
    _puts: unsafe extern "C" fn(*const c_char),
) -> i32 {
    naked_asm!(
        ".option push
        .option norelax
        addi    sp, sp, -32
        sd      ra, 0(sp)
        sd      s0, 8(sp)
        sd      gp, 16(sp)
        sd      tp, 24(sp)
        mv      s0, sp
        lla     sp, {stack_top}
        lla     t0, {firmware}
        jalr    ra, t0, 0
        mv      sp, s0
        ld      ra, 0(sp)
        ld      s0, 8(sp)
        ld      gp, 16(sp)
        ld      tp, 24(sp)
        addi    sp, sp, 32
        ret
        .option pop",
        stack_top = sym __ddr_stack_top,
        firmware = sym FIRMWARE,
    );
}

unsafe extern "C" fn puts(text: *const c_char) {
    // SAFETY: The vendor puts callback receives a live NUL-terminated string.
    let bytes = unsafe { CStr::from_ptr(text) }.to_bytes();
    let mut output = crate::io::stdout();
    output.write_all(bytes).ok();
    output.flush().ok();
}

// Both endpoints must be 64-byte aligned and belong to accessible memory.
unsafe fn flush(start: usize, end: usize) {
    riscv::asm::fence();
    let mut address = start;
    while address < end {
        // SAFETY: K1 X60 implements Zicbom; M-mode owns these complete cache lines.
        unsafe {
            asm!(
                ".option push",
                ".option arch, +zicbom",
                "cbo.flush ({address})",
                ".option pop",
                address = in(reg) address,
                options(nostack),
            );
        }
        address += 64;
    }
    riscv::asm::fence();
}

unsafe fn check(address: usize, expected: u64) -> Result<(), Error> {
    // SAFETY: Caller supplies an aligned location in the initialized test window.
    let actual = unsafe { ptr::read_volatile(address as *const u64) };
    if actual != expected {
        return Err(Error::Memory {
            address,
            expected,
            actual,
        });
    }
    Ok(())
}

unsafe fn test_memory() -> Result<(), Error> {
    // SAFETY: Both stages use only the caller's exclusively owned test windows.
    unsafe {
        test_patterns()?;
        test_address_lines()
    }
}

#[inline]
unsafe fn test_patterns() -> Result<(), Error> {
    // Flush after writing: volatile accesses alone could pass entirely in cache.
    for &pattern in &[0, u64::MAX, 0x5555_5555_5555_5555, 0xaaaa_aaaa_aaaa_aaaa] {
        let mut address = DRAM_START;
        while address < DRAM_START + TEST_SIZE {
            // SAFETY: Aligned unused DRAM, after training; never address zero or MMIO.
            unsafe { ptr::write_volatile(address as *mut u64, pattern ^ address as u64) };
            address += 8;
        }
        // SAFETY: The complete test window consists of exclusively owned cache lines.
        unsafe { flush(DRAM_START, DRAM_START + TEST_SIZE) };
        let mut address = DRAM_START;
        while address < DRAM_START + TEST_SIZE {
            // SAFETY: Same initialized window; compare only after writeback/invalidation.
            unsafe { check(address, pattern ^ address as u64)? };
            address += 8;
        }
    }
    Ok(())
}

#[inline]
unsafe fn test_address_lines() -> Result<(), Error> {
    // Separate address-line probes catch aliases beyond the contiguous window.
    static ADDRESSES: [usize; 26] = {
        let mut addresses = [DRAM_START; 26];
        let mut bit = 3;
        while bit < 27 {
            addresses[bit - 2] = DRAM_START + (1 << bit);
            bit += 1;
        }
        addresses[25] = DRAM_END - 8;
        addresses
    };
    for &pattern in &[0x5555_5555_5555_5555, 0xaaaa_aaaa_aaaa_aaaa] {
        let mut index = 0;
        while index < ADDRESSES.len() {
            let address = ADDRESSES[index];
            // SAFETY: Each probe is aligned, below DRAM_END and outside live images.
            unsafe { ptr::write_volatile(address as *mut u64, pattern ^ index as u64) };
            index += 1;
        }
        for &address in &ADDRESSES {
            // SAFETY: Round each probe down to its exclusively owned cache line.
            unsafe { flush(address & !63, (address & !63) + 64) };
        }
        let mut index = 0;
        while index < ADDRESSES.len() {
            let address = ADDRESSES[index];
            // SAFETY: Same initialized address-line probe locations.
            unsafe { check(address, pattern ^ index as u64)? };
            index += 1;
        }
    }
    Ok(())
}
