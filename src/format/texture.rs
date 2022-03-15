#[derive(Debug)]
pub enum Texture {
    Old {
        width: u16,
        height: u16,
        format: SurfaceFormatOld,
        unk1: u16,
        unk2: u32,
        size: u32, // ????
        data_pos: u32,
        unk3: u64,
    },
    New {
        format: SurfaceFormatNew, // Shut it with the warnings, rustc
    },
}

// https://github.com/KillzXGaming/Switch-Toolbox/blob/488b689c2794096744c17e2132ab6dfcd14e5557/Switch_Toolbox_Library/Texture%20Decoding/3DS/Swizzle_3DS.cs#L14
#[derive(Debug)]
pub enum SurfaceFormatOld {
    RGBA8 = 0,
    RGB8 = 1,
    RGBA5551 = 2,
    RGB565 = 3,
    RGBA4 = 4,
    LA8 = 5,
    HiLo8 = 6,
    L8 = 7,
    A8 = 8,
    LA4 = 9,
    L4 = 10,
    A4 = 11,
    ETC1 = 12,
    ETC1A4 = 13,
}

// https://github.com/KillzXGaming/Switch-Toolbox/blob/488b689c2794096744c17e2132ab6dfcd14e5557/File_Format_Library/FileFormats/Effects/PTCL_3DS.cs#L450
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum SurfaceFormatNew {
    INVALID = 0x0,
    TCS_R8_G8_B8_A8 = 2,
    T_BC1_UNORM = 3,
    T_BC1_SRGB = 4,
    T_BC2_UNORM = 5,
    T_BC2_SRGB = 6,
    T_BC3_UNORM = 7,
    T_BC3_SRGB = 8,
    T_BC4_UNORM = 9,
    T_BC4_SNORM = 10,
    T_BC5_UNORM = 11,
    T_BC5_SNORM = 12,
    TC_R8_UNORM = 13,
    TC_R8_G8_UNORM = 14,
    TC_R8_G8_B8_A8_SRGB = 15,
    TC_R8_SNORM = 16,
    TC_R4_R4_SNORM = 17,
    ETC1_A4 = 18,
    ETC1 = 19,
    HIL08 = 20,
    L4 = 21,
    A4 = 22,
    L8 = 23,
    A8 = 24,
    LA4 = 25,
    LA8 = 26,
    TCS_R5_G5_B5_A1_UNORM = 27,
    TC_R4_G4_B4_UNORM = 28,
    TC_R8_G8_B8_A8_UNORM = 29,
    TC_R8_G8_B8_UNORM = 30,
    TCS_R5_G6_B5_UNORM = 31,
}
