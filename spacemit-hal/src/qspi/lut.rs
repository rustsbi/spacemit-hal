use super::{Error, Transfer, Width};

// SDR LUT encoding: [opcode:6 | log2(lines):2 | operand:8], two per u32.
// Address bytes use MODE, as in Linux spi-fsl-qspi.c, avoiding SFAR address aliasing.
pub(super) fn encode(transfer: Transfer, data: Option<bool>) -> Result<[u32; 4], Error> {
    if transfer.dummy_cycles > 64 {
        return Err(Error::InvalidTransfer);
    }
    let mut words = [0; 4];
    let mut index = 0;
    let mut push = |opcode: u32, width, operand: u8| {
        let pads = match width {
            Width::Single => 0,
            Width::Dual => 1,
            Width::Quad => 2,
        };
        words[index / 2] |=
            ((opcode << 10) | (pads << 8) | u32::from(operand)) << (16 * (index % 2));
        index += 1;
    };
    push(1, transfer.instruction_width, transfer.opcode);
    if let Some(address) = transfer.address {
        let bytes = address.size as u32;
        if bytes < 4 && address.value >> (bytes * 8) != 0 {
            return Err(Error::InvalidTransfer);
        }
        for byte in address.value.to_be_bytes().iter().skip(4 - bytes as usize) {
            push(4, address.width, *byte);
        }
    }
    if transfer.dummy_cycles != 0 {
        push(3, transfer.dummy_width, transfer.dummy_cycles);
    }
    if let Some(read) = data {
        push(if read { 7 } else { 8 }, transfer.data_width, 0);
    }
    // At most seven instructions; the remaining zero halfword is STOP.
    Ok(words)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qspi::AddressSize;

    #[test]
    fn command_address_dummy_and_widths() {
        assert_eq!(
            encode(Transfer::new(0x9f), Some(true)).unwrap(),
            [0x1c00_049f, 0, 0, 0]
        );
        let mut transfer = Transfer::new(0x6b)
            .with_address(0x12345678, AddressSize::Four)
            .with_dummy_cycles(8);
        transfer.data_width = Width::Quad;
        assert_eq!(
            encode(transfer, Some(true)).unwrap(),
            [0x1012_046b, 0x1056_1034, 0x0c08_1078, 0x0000_1e00]
        );
        assert!(
            encode(
                transfer.with_address(0x1000000, AddressSize::Three),
                Some(true)
            )
            .is_err()
        );
        assert!(encode(transfer.with_dummy_cycles(65), Some(true)).is_err());
        assert_eq!(encode(Transfer::new(6), None).unwrap(), [0x406, 0, 0, 0]);
        assert_eq!(
            encode(Transfer::new(1), Some(false)).unwrap(),
            [0x2000_0401, 0, 0, 0]
        );
    }
}
