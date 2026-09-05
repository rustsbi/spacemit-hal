use embedded_hal::digital::PinState;

use super::register::{bank::BankRegisters, k1, k3};

#[derive(Clone, Copy)]
pub(super) struct GpioInner<'a> {
    registers: BankRegisters<'a>,
    mask: u32,
}

impl<'a> GpioInner<'a> {
    #[inline]
    pub(super) fn new_k1(bank: u8, number: u8, gpio: &'a k1::RegisterBlock) -> Self {
        Self::new(bank, number, gpio.bank(bank))
    }

    #[inline]
    pub(super) fn new_k3(bank: u8, number: u8, gpio: &'a k3::RegisterBlock) -> Self {
        Self::new(bank, number, gpio.bank(bank))
    }

    #[inline]
    fn new(bank: u8, number: u8, registers: Option<BankRegisters<'a>>) -> Self {
        assert!(number < 32, "GPIO pin number must be in 0..32");
        let registers = registers.unwrap_or_else(|| panic!("GPIO bank {bank} must be in 0..4"));
        Self {
            registers,
            mask: 1 << number,
        }
    }

    #[inline]
    pub(super) fn configure_input(self) {
        // SAFETY: Selects only the uniquely owned pin.
        unsafe { self.registers.direction_clear.write(self.mask) };
    }

    #[inline]
    pub(super) fn configure_output(self, initial_state: PinState) {
        // Program the output latch before enabling the output driver so the
        // transition cannot expose an old, unknown latch value on the pad.
        self.set_state(initial_state);
        // SAFETY: Selects only the uniquely owned pin.
        unsafe { self.registers.direction_set.write(self.mask) };
    }

    #[inline]
    pub(super) fn is_high(self) -> bool {
        self.registers.pin_level.read() & self.mask != 0
    }

    #[inline]
    pub(super) fn set_state(self, state: PinState) {
        let register = match state {
            PinState::Low => self.registers.pin_output_clear,
            PinState::High => self.registers.pin_output_set,
        };
        // SAFETY: Selects only the uniquely owned pin.
        unsafe { register.write(self.mask) };
    }
}
