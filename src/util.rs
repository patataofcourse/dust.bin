use std::{fs::File, io::Read};
pub fn read_str(file: &mut File) -> String {
    unimplemented!();
}

pub fn read_str_sized<const size: usize>(file: &mut File) -> anyhow::Result<String> {
    let mut bytes: [u8; size] = [0; size];
    file.read_exact(&mut bytes)?;
    Ok(String::from_utf8(bytes.to_vec())?)
}

pub fn read_str_at(file: &mut File, offset: u32) -> String {
    unimplemented!();
}
