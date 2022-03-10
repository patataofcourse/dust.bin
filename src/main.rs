use dust_bin::EffectFile;
use std::{path::PathBuf, str::FromStr};

fn main() {
    println!("--Quiz Show--");
    let effect_quiz = EffectFile::from_file(PathBuf::from_str("test-files/agbQuiz.ptcl").unwrap());
    match effect_quiz {
        Ok(_) => (),
        Err(c) => println!("Error: {}", c),
    }
    println!("--MK7--");
    let effect_kart = EffectFile::from_file(PathBuf::from_str("test-files/Kart.ptcl").unwrap());
    match effect_kart {
        Ok(_) => (),
        Err(c) => println!("Error: {}", c),
    }
}
