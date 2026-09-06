use embedded_hal::digital::PinState;

use super::register::{bank::BankRegisters, k1, k3};

#[derive(Clone, Copy)]
pub(super) struct GpioInner<'a> {
    registers: BankRegisters<'a>,
    mask: u32,
    configuration: Option<&'a volatile_register::RW<u32>>,
    gpio_function: u8,
    _not_send_sync: core::marker::PhantomData<*mut ()>,
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
            configuration: None,
            gpio_function: 0,
            _not_send_sync: core::marker::PhantomData,
        }
    }

    pub(super) fn with_configuration(
        mut self,
        configuration: &'a volatile_register::RW<u32>,
        gpio_function: u8,
    ) -> Self {
        self.configuration = Some(configuration);
        self.gpio_function = gpio_function;
        self
    }

    pub(super) fn configure_function(self, function: u8) {
        assert!(function < 8, "GPIO alternate function must be in 0..8");
        if let Some(configuration) = self.configuration {
            // SAFETY: Exclusive MFPR register; change only AF_SEL [2:0], preserving RW EDGE_CLEAR.
            unsafe { configuration.modify(|bits| (bits & !0x7) | u32::from(function)) };
        }
    }

    #[inline]
    pub(super) fn configure_input(self) {
        // SAFETY: Selects only the uniquely owned pin.
        unsafe { self.registers.direction_clear.write(self.mask) };
        self.configure_function(self.gpio_function);
    }

    #[inline]
    pub(super) fn configure_output(self, initial_state: PinState) {
        // Set the latch before enabling output.
        self.set_state(initial_state);
        // SAFETY: Selects only the uniquely owned pin.
        unsafe { self.registers.direction_set.write(self.mask) };
        self.configure_function(self.gpio_function);
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
