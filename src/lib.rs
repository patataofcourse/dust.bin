use std::{fs::File, io, io::Read, path::PathBuf};

mod etc_dec;

pub struct EffectFile {
    pub version: u32,
    pub unk: u64,
    pub emitter_sets: Vec<EmitterSet>,
    pub texture_folder: Vec<Texture>,
}

pub struct EmitterSet {
    pub name: String,
    pub unk1: u32,
    pub unk2: u32,
    pub description: String, //?
    pub name_pointer: u32,   //?
    pub emitters: Vec<Emitter>,
}

pub struct Emitter {
    pub unk: [u8; 0x56 / 4], // Switch Toolbox ignores this
}

pub struct Texture {}

impl EffectFile {
    pub fn from_file(fname: PathBuf) -> io::Result<Self> {
        let mut f = File::open(fname)?;

        let mut magic = [0, 0, 0, 0];
        f.read(&mut magic)?;
        if magic != "SPBD".as_bytes() {
            println!("Not an SPBD PTCL file");
            return Err(io::Error::from(io::ErrorKind::Other));
        }

        Ok(Self {
            version: 0,
            unk: 0,
            emitter_sets: vec![],
            texture_folder: vec![],
        })
    }
}
