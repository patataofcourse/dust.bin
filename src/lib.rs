use bytestream::{ByteOrder, StreamReader};
use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
    path::PathBuf,
};

mod etc_dec;
mod util;

static SUPPORTED_VERSIONS: [u32; 2] = [0xB, 0x33];

#[derive(Debug)]
pub struct EffectFile {
    pub name: String,
    pub version: u32,
    pub emitter_sets: Vec<EmitterSet>,
    pub textures: Vec<Texture>,
    pub other_offsets: Option<SwitchToolboxUnusedOffsets>,
}

#[derive(Debug)]
pub struct SwitchToolboxUnusedOffsets {
    pub shader_gtx_tab_pos: u32,
    pub shader_gtx_tab_size: u32,
    pub keyanim_tab_pos: u32,
    pub keyanim_tab_size: u32,
    pub primitive_tab_pos: u32,
    pub primitive_tab_size: u32,
    pub shader_param_tab_pos: u32,
    pub shader_param_tab_size: u32,
    pub texture_total_size: u32,
    pub shader_total_size: u32,
    pub emitter_total_size: u32,
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
            eprintln!("Not an SPBD PTCL file");
            Err(io::Error::from(io::ErrorKind::Other))?;
        }
        let version = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
        println!("Version {:#X} ", version);
        if !SUPPORTED_VERSIONS.contains(&version) {
            println!("Warning: version might not be fully supported")
        }

        //now here comes a bunch of data we can't use yet
        let emitter_count = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
        let name_pos = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
        let effect_name_table = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
        let name = util::read_str_at(&mut f, (name_pos + effect_name_table).into())?;
        let texture_table_pos = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
        let texture_table_size = u32::read_from(&mut f, ByteOrder::LittleEndian)?;

        // "Fun" part - stuff that depends on version
        let other_offsets: Option<SwitchToolboxUnusedOffsets>;
        if version > 0xB {
            other_offsets = Some(SwitchToolboxUnusedOffsets {
                shader_gtx_tab_pos: u32::read_from(&mut f, ByteOrder::LittleEndian)?,
                shader_gtx_tab_size: u32::read_from(&mut f, ByteOrder::LittleEndian)?,
                keyanim_tab_pos: u32::read_from(&mut f, ByteOrder::LittleEndian)?,
                keyanim_tab_size: u32::read_from(&mut f, ByteOrder::LittleEndian)?,
                primitive_tab_pos: u32::read_from(&mut f, ByteOrder::LittleEndian)?,
                primitive_tab_size: u32::read_from(&mut f, ByteOrder::LittleEndian)?,
                shader_param_tab_pos: u32::read_from(&mut f, ByteOrder::LittleEndian)?,
                shader_param_tab_size: u32::read_from(&mut f, ByteOrder::LittleEndian)?,
                texture_total_size: u32::read_from(&mut f, ByteOrder::LittleEndian)?,
                shader_total_size: u32::read_from(&mut f, ByteOrder::LittleEndian)?,
                emitter_total_size: u32::read_from(&mut f, ByteOrder::LittleEndian)?,
            });
        } else {
            other_offsets = None;
        }

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
                let name = util::read_str_at(&mut f, (effect_name_table + name_offset).into())?;
                u32::read_from(&mut f, ByteOrder::LittleEndian)?; // padding

                f.seek(SeekFrom::Start(pos))?;
                emitters.push(Emitter { unk_data, name })
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

        let out = Self {
            name,
            version,
            emitter_sets,
            textures: vec![],
            other_offsets,
        };
        Ok(out)
    }
}
