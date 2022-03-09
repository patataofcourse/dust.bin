mod etc_dec;

pub struct EffectFile {
    pub version: u32,
    pub unk: u64,
    pub emitter_sets: Vec<EmitterSet>,
    pub texture_folder: Vec<Texture>,
}

pub struct EmitterSet {}

pub struct Texture {}
