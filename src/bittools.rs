

// macro_rules! SignExtended {
//     () => {
        
//     };
// }

const sign_mask: [u32; 32] = [
    0xFFFF_FFFE, 0xFFFF_FFFC, 0xFFFF_FFF8, 0xFFFF_FFF0,
    0xFFFF_FFE0, 0xFFFF_FFC0, 0xFFFF_FF80, 0xFFFF_FF00,
    0xFFFF_FE00, 0xFFFF_FC00, 0xFFFF_F800, 0xFFFF_F000,
    0xFFFF_E000, 0xFFFF_C000, 0xFFFF_8000, 0xFFFF_0000,
    0xFFFE_0000, 0xFFFC_0000, 0xFFF8_0000, 0xFFF0_0000,
    0xFFE0_0000, 0xFFC0_0000, 0xFF80_0000, 0xFF00_0000,
    0xFE00_0000, 0xFC00_0000, 0xF800_0000, 0xF000_0000,
    0xE000_0000, 0xC000_0000, 0x8000_0000, 0x0000_0000,
];

fn signExtend(val: u32, width: u32) -> u32 {
    let mut sign_extended_val: u32 = val;
    if (val >> (width - 1) & 1) == 1 {
        sign_extended_val |= sign_mask[(width - 1) as usize];
    } else {
        sign_extended_val &= !sign_mask[(width - 1) as usize];
    }
    return sign_extended_val;
}

fn zeroExtend(val: u32, width: u32) -> u32 {
    let mut zero_extended_val: u32 = val;
    zero_extended_val &= !sign_mask[(width - 1) as usize];
    return zero_extended_val;
}