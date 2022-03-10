use dust_bin::EffectFile;
use std::{path::PathBuf, str::FromStr};

fn main() {
    let effect = EffectFile::from_file(PathBuf::from_str("test-files/agbQuiz.ptcl").unwrap());
    match effect {
        Ok(_) => (),
        Err(c) => println!("{}", c),
    }
}
