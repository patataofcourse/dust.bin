use bytestream::{ByteOrder, StreamReader};
use std::{fs::File, io, io::Read, path::PathBuf};

mod etc_dec;
mod util;

pub struct EffectFile {
    pub version: u32,
    pub unk: Option<u64>,
    pub emitter_sets: Vec<EmitterSet>,
    pub texture_folder: Vec<Texture>,
}

pub struct EmitterSet {
    pub name: String,
    pub unk1: u32,
    pub unk2: u32,
    pub description: u32,  //?
    pub name_pointer: u32, //?
    pub emitters: Vec<Emitter>,
}

pub struct Emitter {
    pub unk: EmitterUnknownData, // Switch Toolbox ignores this
}

pub enum EmitterUnknownData {
    Old([u8; 0xC]),
    New([u8; 0x56]),
}

pub struct Texture {}

impl EffectFile {
    pub fn from_file(fname: PathBuf) -> io::Result<Self> {
        let mut f = File::open(fname)?;

        let magic = util::read_str_sized(&mut f, 4);
        if magic != "SPBD" {
            println!("Not an SPBD PTCL file");
            return Err(io::Error::from(io::ErrorKind::Other));
        }
        let version = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
        if version < 0xB {
            println!(
                "Warning: version {:#X} might not be fully supported",
                version
            )
        } else {
            println!("Version {:#X} ", version);
        }

        //now here comes a bunch of data we can't use yet
        let emitter_count = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
        let header_padding = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
        let effect_name_table = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
        let texture_table_pos = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
        let texture_table_size = u32::read_from(&mut f, ByteOrder::LittleEndian)?;

        // "Fun" part - stuff that depends on version
        let shader_gtx_tab_pos = if version > 0xB {
            Some(u32::read_from(&mut f, ByteOrder::LittleEndian)?)
        } else {
            None
        };
        let shader_gtx_tab_size = if version > 0xB {
            Some(u32::read_from(&mut f, ByteOrder::LittleEndian)?)
        } else {
            None
        };
        let keyanim_tab_pos = if version > 0xB {
            Some(u32::read_from(&mut f, ByteOrder::LittleEndian)?)
        } else {
            None
        };
        let keyanim_tab_size = if version > 0xB {
            Some(u32::read_from(&mut f, ByteOrder::LittleEndian)?)
        } else {
            None
        };
        let primative_tab_pos = if version > 0xB {
            Some(u32::read_from(&mut f, ByteOrder::LittleEndian)?)
        } else {
            None
        };
        let primative_tab_size = if version > 0xB {
            Some(u32::read_from(&mut f, ByteOrder::LittleEndian)?)
        } else {
            None
        };
        let shader_param_tab_pos = if version > 0xB {
            Some(u32::read_from(&mut f, ByteOrder::LittleEndian)?)
        } else {
            None
        };
        let shader_param_tab_size = if version > 0xB {
            Some(u32::read_from(&mut f, ByteOrder::LittleEndian)?)
        } else {
            None
        };
        let texture_tab_size = if version > 0xB {
            Some(u32::read_from(&mut f, ByteOrder::LittleEndian)?)
        } else {
            None
        };
        let unk = if version > 0xB {
            Some(u64::read_from(&mut f, ByteOrder::LittleEndian)?)
        } else {
            None
        };

        let mut emitter_sets = vec![];
        // Emitter sets
        for _ in 0..emitter_count {
            let name_offset = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
            let name = util::read_str_at(&mut f, effect_name_table + name_offset);

            emitter_sets.push(EmitterSet {
                name,
                unk1: 0,
                unk2: 0,
                description: 0,
                name_pointer: 0,
                emitters: vec![],
            })
        }

        Ok(Self {
            version: version,
            unk,
            emitter_sets,
            texture_folder: vec![],
        })
    }
}
