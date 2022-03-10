use bytestream::{ByteOrder, StreamReader};
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
};

pub fn read_str(file: &mut File) -> anyhow::Result<String> {
    let mut bytes = vec![];
    loop {
        let byte = u8::read_from(file, ByteOrder::LittleEndian)?;
        if byte == 0 {
            break;
        }
        bytes.push(byte);
    }
    Ok(String::from_utf8(bytes)?)
}

pub fn read_str_sized<const SIZE: usize>(file: &mut File) -> anyhow::Result<String> {
    let mut bytes: [u8; SIZE] = [0; SIZE];
    file.read_exact(&mut bytes)?;
    Ok(String::from_utf8(bytes.to_vec())?)
}

pub fn read_str_at(file: &mut File, offset: u64) -> anyhow::Result<String> {
    let pos = file.stream_position()?;
    file.seek(SeekFrom::Start(offset))?;
    let string = read_str(file)?;
    file.seek(SeekFrom::Start(pos))?;
    Ok(string)
}
