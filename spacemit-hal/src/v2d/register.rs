//! K1/M1 and K3 V2D registers.

use crate::register::{RW1C, RWNoModify};
use volatile_register::WO;
use volatile_register::{RO, RW};

// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/soc/spacemit/v2d/v2d_reg.h
// https://github.com/spacemit-com/linux-6.18/blob/4158237f35b8fd62ba198c1627e5a66e5a34c50f/drivers/soc/spacemit/v2d/v2d_hw.c
// Packed configuration aliases share one field; palette entries are 32-bit RGBA.

/// K1/M1 and K3 V2D registers.
#[repr(C)]
pub struct RegisterBlock {
    /// V2D_AUTO_CLK_REG.
    pub auto_clk_reg: RWNoModify<u32>,
    /// V2D_ERR_IRQ_MASK.
    pub err_irq_mask: RW<u32>,
    /// V2D_IRQ_MASK.
    pub irq_mask: RW<u32>,
    /// V2D_ERR_IRQ_STATUS.
    pub err_irq_status: RW1C<u32>,
    /// V2D_IRQ_STATUS.
    pub irq_status: RW1C<u32>,
    /// V2D_ERR_IRQ_RAW.
    pub err_irq_raw: RO<u32>,
    /// V2D_IRQ_RAW.
    pub irq_raw: RO<u32>,
    /// V2D_AXI_BUS_CTRL.
    pub axi_bus_ctrl: RW<u32>,
    _padding_0x020: [u32; 56],
    /// V2D_CTRL_REG.
    pub ctrl_reg: RWNoModify<u32>,
    /// Scaler coefficients.
    pub scaler_coefficients: [RW<u32>; 24],
    /// V2D_BLEND_REG0.
    pub blend_reg0: RW<u32>,
    /// V2D_BLEND_REG1.
    pub blend_reg1: RW<u32>,
    /// V2D_BLD_MASK_REG0.
    pub bld_mask_reg0: RW<u32>,
    /// V2D_BLD_MASK_REG1.
    pub bld_mask_reg1: RW<u32>,
    /// V2D_BLD_MASK_REG2.
    pub bld_mask_reg2: RW<u32>,
    /// V2D_OUTPUT_Y_ADDR_L.
    pub output_y_addr_l: RW<u32>,
    /// V2D_OUTPUT_Y_ADDR_H.
    pub output_y_addr_h: RW<u32>,
    /// V2D_OUTPUT_UV_ADDR_L.
    pub output_uv_addr_l: RW<u32>,
    /// V2D_OUTPUT_UV_ADDR_H / V2D_OUTPUT_WIDTH.
    pub output_uv_addr_h_output_width: RW<u32>,
    /// V2D_OUTPUT_HEIGHT.
    pub output_height: RW<u32>,
    /// V2D_OUTPUT_CRTL0.
    pub output_crtl0: RW<u32>,
    /// V2D_OUTPUT_CRTL1.
    pub output_crtl1: RW<u32>,
    /// V2D_OUTPUT_CRTL2.
    pub output_crtl2: RW<u32>,
    /// V2D_MASK_ADDR_L.
    pub mask_addr_l: RW<u32>,
    /// V2D_MASK_ADDR_H / V2D_MASK_WIDTH.
    pub mask_addr_h_mask_width: RW<u32>,
    /// V2D_MASK_HEIGHT.
    pub mask_height: RW<u32>,
    /// V2D_MASK_CROP_REG0.
    pub mask_crop_reg0: RW<u32>,
    /// V2D_MASK_CROP_REG1.
    pub mask_crop_reg1: RW<u32>,
    /// Input layers.
    pub layer: [Layer; 2],
    _padding_0x254: [u32; 42],
    /// V2D_DEBUG_REG0.
    pub debug_reg0: RWNoModify<u32>,
    /// V2D_DEBUG_REG1.
    pub debug_reg1: RWNoModify<u32>,
    /// V2D_DMA_CTRL.
    pub dma_ctrl: RW<u32>,
    /// RGBA palette.
    pub palette: [RW<u32>; 256],
    _padding_0x708: [u32; 62],
    /// FBC encoder.
    pub encoder: Encoder,
    _padding_0x84c: [u32; 45],
    /// FBC decoders.
    pub decoder: [Decoder; 2],
}

/// V2D input layer registers.
#[repr(C)]
pub struct Layer {
    /// V2D_LAYER0_Y_ADDR_L.
    pub y_addr_l: RW<u32>,
    /// V2D_LAYER0_Y_ADDR_H.
    pub y_addr_h: RW<u32>,
    /// V2D_LAYER0_UV_ADDR_L.
    pub uv_addr_l: RW<u32>,
    /// V2D_LAYER0_UV_ADDR_H / V2D_LAYER0_BLD_FACTOR.
    pub uv_addr_h_bld_factor: RW<u32>,
    /// V2D_LAYER0_WIDTH_HEIGHT.
    pub width_height: RW<u32>,
    /// V2D_LAYER0_CTRL.
    pub ctrl: RW<u32>,
    /// V2D_LAYER0_CROP_REG0.
    pub crop_reg0: RW<u32>,
    /// V2D_LAYER0_CROP_REG1.
    pub crop_reg1: RW<u32>,
    /// V2D_LAYER0_SOLIDCOLOR_CTRL0.
    pub solidcolor_ctrl0: RW<u32>,
    /// V2D_LAYER0_SOLIDCOLOR_CTRL1 / V2D_LAYER0_CSC_CRTL0.
    pub solidcolor_ctrl1_csc_crtl0: RW<u32>,
    /// V2D_LAYER0_CSC_CRTL1.
    pub csc_crtl1: RW<u32>,
    /// V2D_LAYER0_CSC_CRTL2.
    pub csc_crtl2: RW<u32>,
    /// V2D_LAYER0_CSC_CRTL3.
    pub csc_crtl3: RW<u32>,
    /// V2D_LAYER0_CSC_CRTL4.
    pub csc_crtl4: RW<u32>,
    /// V2D_LAYER0_CSC_CRTL5.
    pub csc_crtl5: RW<u32>,
    /// V2D_LAYER0_CSC_CRTL6 / V2D_LAYER0_SCALE_MODE.
    pub csc_crtl6_scale_mode: RW<u32>,
    /// V2D_LAYER0_SCALE_DELTA_X.
    pub scale_delta_x: RW<u32>,
    /// V2D_LAYER0_SCALE_DELTA_Y / V2D_LAYER0_BLD_CTRL0.
    pub scale_delta_y_bld_ctrl0: RW<u32>,
    /// V2D_LAYER0_BLD_CTRL1.
    pub bld_ctrl1: RW<u32>,
    /// V2D_LAYER0_BLD_CTRL2.
    pub bld_ctrl2: RW<u32>,
    /// V2D_LAYER0_BLD_CTRL3.
    pub bld_ctrl3: RW<u32>,
}

/// V2D FBC encoder registers.
#[repr(C)]
pub struct Encoder {
    /// header address low.
    pub header_address_low: RW<u32>,
    /// header address high.
    pub header_address_high: RW<u32>,
    /// payload address low.
    pub payload_address_low: RW<u32>,
    /// payload address high.
    pub payload_address_high: RW<u32>,
    /// bounding box x.
    pub bounding_box_x: RW<u32>,
    /// bounding box y.
    pub bounding_box_y: RW<u32>,
    /// y buffer address.
    pub y_buffer_address: RW<u32>,
    /// y buffer pitch.
    pub y_buffer_pitch: RW<u32>,
    /// uv buffer address.
    pub uv_buffer_address: RW<u32>,
    /// uv buffer pitch.
    pub uv_buffer_pitch: RW<u32>,
    /// y buffer size.
    pub y_buffer_size: RW<u32>,
    /// uv buffer size.
    pub uv_buffer_size: RW<u32>,
    /// shadow control.
    pub shadow_control: RWNoModify<u32>,
    /// interrupt mask.
    pub interrupt_mask: RW<u32>,
    /// interrupt clear.
    pub interrupt_clear: WO<u32>,
    /// dma control.
    pub dma_control: RW<u32>,
    /// mode.
    pub mode: RW<u32>,
    /// dma length.
    pub dma_length: RW<u32>,
    /// interrupt status.
    pub interrupt_status: RW1C<u32>,
}

/// V2D FBC decoder registers.
#[repr(C)]
pub struct Decoder {
    /// header address low.
    pub header_address_low: RW<u32>,
    /// header address high.
    pub header_address_high: RW<u32>,
    /// bounding box x.
    pub bounding_box_x: RW<u32>,
    /// bounding box y.
    pub bounding_box_y: RW<u32>,
    /// image size.
    pub image_size: RW<u32>,
    /// mode.
    pub mode: RW<u32>,
    /// dma control.
    pub dma_control: RW<u32>,
    /// interrupt mask.
    pub interrupt_mask: RW<u32>,
    /// raw interrupt.
    pub raw_interrupt: RO<u32>,
    /// interrupt status.
    pub interrupt_status: RWNoModify<u32>,
    /// trigger control.
    pub trigger_control: RWNoModify<u32>,
    /// output y address.
    pub output_y_address: RW<u32>,
    /// output uv address.
    pub output_uv_address: RW<u32>,
    /// output stride.
    pub output_stride: RW<u32>,
    _padding_0x038: [u32; 50],
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem::{offset_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(offset_of!(Layer, y_addr_l), 0x000);
        assert_eq!(offset_of!(Layer, y_addr_h), 0x004);
        assert_eq!(offset_of!(Layer, uv_addr_l), 0x008);
        assert_eq!(offset_of!(Layer, uv_addr_h_bld_factor), 0x00c);
        assert_eq!(offset_of!(Layer, width_height), 0x010);
        assert_eq!(offset_of!(Layer, ctrl), 0x014);
        assert_eq!(offset_of!(Layer, crop_reg0), 0x018);
        assert_eq!(offset_of!(Layer, crop_reg1), 0x01c);
        assert_eq!(offset_of!(Layer, solidcolor_ctrl0), 0x020);
        assert_eq!(offset_of!(Layer, solidcolor_ctrl1_csc_crtl0), 0x024);
        assert_eq!(offset_of!(Layer, csc_crtl1), 0x028);
        assert_eq!(offset_of!(Layer, csc_crtl2), 0x02c);
        assert_eq!(offset_of!(Layer, csc_crtl3), 0x030);
        assert_eq!(offset_of!(Layer, csc_crtl4), 0x034);
        assert_eq!(offset_of!(Layer, csc_crtl5), 0x038);
        assert_eq!(offset_of!(Layer, csc_crtl6_scale_mode), 0x03c);
        assert_eq!(offset_of!(Layer, scale_delta_x), 0x040);
        assert_eq!(offset_of!(Layer, scale_delta_y_bld_ctrl0), 0x044);
        assert_eq!(offset_of!(Layer, bld_ctrl1), 0x048);
        assert_eq!(offset_of!(Layer, bld_ctrl2), 0x04c);
        assert_eq!(offset_of!(Layer, bld_ctrl3), 0x050);
        assert_eq!(size_of::<Layer>(), 0x054);
        assert_eq!(offset_of!(Encoder, header_address_low), 0x000);
        assert_eq!(offset_of!(Encoder, header_address_high), 0x004);
        assert_eq!(offset_of!(Encoder, payload_address_low), 0x008);
        assert_eq!(offset_of!(Encoder, payload_address_high), 0x00c);
        assert_eq!(offset_of!(Encoder, bounding_box_x), 0x010);
        assert_eq!(offset_of!(Encoder, bounding_box_y), 0x014);
        assert_eq!(offset_of!(Encoder, y_buffer_address), 0x018);
        assert_eq!(offset_of!(Encoder, y_buffer_pitch), 0x01c);
        assert_eq!(offset_of!(Encoder, uv_buffer_address), 0x020);
        assert_eq!(offset_of!(Encoder, uv_buffer_pitch), 0x024);
        assert_eq!(offset_of!(Encoder, y_buffer_size), 0x028);
        assert_eq!(offset_of!(Encoder, uv_buffer_size), 0x02c);
        assert_eq!(offset_of!(Encoder, shadow_control), 0x030);
        assert_eq!(offset_of!(Encoder, interrupt_mask), 0x034);
        assert_eq!(offset_of!(Encoder, interrupt_clear), 0x038);
        assert_eq!(offset_of!(Encoder, dma_control), 0x03c);
        assert_eq!(offset_of!(Encoder, mode), 0x040);
        assert_eq!(offset_of!(Encoder, dma_length), 0x044);
        assert_eq!(offset_of!(Encoder, interrupt_status), 0x048);
        assert_eq!(size_of::<Encoder>(), 0x04c);
        assert_eq!(offset_of!(Decoder, header_address_low), 0x000);
        assert_eq!(offset_of!(Decoder, header_address_high), 0x004);
        assert_eq!(offset_of!(Decoder, bounding_box_x), 0x008);
        assert_eq!(offset_of!(Decoder, bounding_box_y), 0x00c);
        assert_eq!(offset_of!(Decoder, image_size), 0x010);
        assert_eq!(offset_of!(Decoder, mode), 0x014);
        assert_eq!(offset_of!(Decoder, dma_control), 0x018);
        assert_eq!(offset_of!(Decoder, interrupt_mask), 0x01c);
        assert_eq!(offset_of!(Decoder, raw_interrupt), 0x020);
        assert_eq!(offset_of!(Decoder, interrupt_status), 0x024);
        assert_eq!(offset_of!(Decoder, trigger_control), 0x028);
        assert_eq!(offset_of!(Decoder, output_y_address), 0x02c);
        assert_eq!(offset_of!(Decoder, output_uv_address), 0x030);
        assert_eq!(offset_of!(Decoder, output_stride), 0x034);
        assert_eq!(size_of::<Decoder>(), 0x100);
        assert_eq!(offset_of!(RegisterBlock, auto_clk_reg), 0x000);
        assert_eq!(offset_of!(RegisterBlock, err_irq_mask), 0x004);
        assert_eq!(offset_of!(RegisterBlock, irq_mask), 0x008);
        assert_eq!(offset_of!(RegisterBlock, err_irq_status), 0x00c);
        assert_eq!(offset_of!(RegisterBlock, irq_status), 0x010);
        assert_eq!(offset_of!(RegisterBlock, err_irq_raw), 0x014);
        assert_eq!(offset_of!(RegisterBlock, irq_raw), 0x018);
        assert_eq!(offset_of!(RegisterBlock, axi_bus_ctrl), 0x01c);
        assert_eq!(offset_of!(RegisterBlock, ctrl_reg), 0x100);
        assert_eq!(offset_of!(RegisterBlock, scaler_coefficients), 0x104);
        assert_eq!(offset_of!(RegisterBlock, blend_reg0), 0x164);
        assert_eq!(offset_of!(RegisterBlock, blend_reg1), 0x168);
        assert_eq!(offset_of!(RegisterBlock, bld_mask_reg0), 0x16c);
        assert_eq!(offset_of!(RegisterBlock, bld_mask_reg1), 0x170);
        assert_eq!(offset_of!(RegisterBlock, bld_mask_reg2), 0x174);
        assert_eq!(offset_of!(RegisterBlock, output_y_addr_l), 0x178);
        assert_eq!(offset_of!(RegisterBlock, output_y_addr_h), 0x17c);
        assert_eq!(offset_of!(RegisterBlock, output_uv_addr_l), 0x180);
        assert_eq!(
            offset_of!(RegisterBlock, output_uv_addr_h_output_width),
            0x184
        );
        assert_eq!(offset_of!(RegisterBlock, output_height), 0x188);
        assert_eq!(offset_of!(RegisterBlock, output_crtl0), 0x18c);
        assert_eq!(offset_of!(RegisterBlock, output_crtl1), 0x190);
        assert_eq!(offset_of!(RegisterBlock, output_crtl2), 0x194);
        assert_eq!(offset_of!(RegisterBlock, mask_addr_l), 0x198);
        assert_eq!(offset_of!(RegisterBlock, mask_addr_h_mask_width), 0x19c);
        assert_eq!(offset_of!(RegisterBlock, mask_height), 0x1a0);
        assert_eq!(offset_of!(RegisterBlock, mask_crop_reg0), 0x1a4);
        assert_eq!(offset_of!(RegisterBlock, mask_crop_reg1), 0x1a8);
        assert_eq!(offset_of!(RegisterBlock, layer), 0x1ac);
        assert_eq!(offset_of!(RegisterBlock, debug_reg0), 0x2fc);
        assert_eq!(offset_of!(RegisterBlock, debug_reg1), 0x300);
        assert_eq!(offset_of!(RegisterBlock, dma_ctrl), 0x304);
        assert_eq!(offset_of!(RegisterBlock, palette), 0x308);
        assert_eq!(offset_of!(RegisterBlock, encoder), 0x800);
        assert_eq!(offset_of!(RegisterBlock, decoder), 0x900);
        assert_eq!(size_of::<RegisterBlock>(), 0xb00);
    }
}
