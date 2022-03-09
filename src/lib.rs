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

pub struct Emitter {}

pub struct Texture {}
