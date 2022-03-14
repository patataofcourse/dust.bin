use bytestream::{ByteOrder, StreamReader};
use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
    path::PathBuf,
};

mod etc_dec;
mod util;

#[derive(Debug)]
pub struct EffectFile {
    pub version: u32,
    pub unk: Option<u64>,
    pub emitter_sets: Vec<EmitterSet>,
}

#[derive(Debug)]
pub struct EmitterSet {
    pub name: String,
    pub unk1: u32,
    pub unk2: u32,
    pub description: u32,
    pub name_pointer: u32,
    pub emitters: Vec<Emitter>,
}

#[derive(Debug)]
pub struct Emitter {
    pub name: String,
    pub unknown_offset: u32,
    pub unk_data: EmitterUnknownData, // Switch Toolbox ignores this
}

#[derive(Debug)]
pub enum EmitterUnknownData {
    Old([u8; 0xC]),
    New([u8; 0x38]),
}

#[derive(Debug)]
pub struct Texture {}

impl EffectFile {
    pub fn from_file(fname: PathBuf) -> anyhow::Result<Self> {
        let mut f = File::open(fname)?;

        let magic = util::read_str_sized::<4>(&mut f)?;
        if magic != "SPBD" {
            println!("Not an SPBD PTCL file");
            Err(io::Error::from(io::ErrorKind::Other))?;
        }
        let version = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
        println!("Version {:#X} ", version);
        if version <= 0xB {
            println!("Warning: version might not be fully supported")
        }

        //now here comes a bunch of data we can't use yet
        let emitter_count = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
        let header_padding = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
        println!(" header padding: {:#010X}", header_padding);
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
        let texture_tab_total_size = if version > 0xB {
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
            let description = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
            let unk1 = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
            let name_offset = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
            let name = util::read_str_at(&mut f, (effect_name_table + name_offset) as u64)?;
            let name_pointer = u32::read_from(&mut f, ByteOrder::LittleEndian)?; //Only used in non-SPBD according to Switch Toolbox
            let emitter_count = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
            let emitter_table_pos = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
            let unk2 = u32::read_from(&mut f, ByteOrder::LittleEndian)?;

            let pos = f.stream_position()?;
            f.seek(SeekFrom::Start(emitter_table_pos.into()))?;
            let mut emitters = vec![];
            for _ in 0..emitter_count {
                let emitter_pos = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
                u32::read_from(&mut f, ByteOrder::LittleEndian)?; // padding
                let unknown_offset = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
                u32::read_from(&mut f, ByteOrder::LittleEndian)?; // padding

                let pos = f.stream_position()?;
                f.seek(SeekFrom::Start(emitter_pos.into()))?;

                let unk_data: EmitterUnknownData;
                if version <= 0xB {
                    let mut data = [0; 0xC];
                    f.read(&mut data)?;
                    unk_data = EmitterUnknownData::Old(data);
                } else {
                    let mut data = [0; 0x38];
                    f.read(&mut data)?;
                    unk_data = EmitterUnknownData::New(data);
                }

                let name_offset = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
                let name = util::read_str_at(&mut f, (effect_name_table + name_offset).into())
                    .unwrap_or("ERROR".to_string());

                f.seek(SeekFrom::Start(pos))?;
                emitters.push(Emitter {
                    unk_data,
                    unknown_offset,
                    name,
                })
            }
            f.seek(SeekFrom::Start(pos))?;

            emitter_sets.push(EmitterSet {
                name,
                unk1,
                unk2,
                description,
                name_pointer,
                emitters,
            })
        }
        println!();

        let out = Self {
            version: version,
            unk,
            emitter_sets,
        };
        println!("{:#?}", out);
        Ok(out)
    }
}
