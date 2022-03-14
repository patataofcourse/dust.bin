use dust_bin::EffectFile;
use std::{path::PathBuf, str::FromStr};

fn main() -> anyhow::Result<()> {
    println!("--Quiz Show--");
    let effect_quiz = EffectFile::from_file(PathBuf::from_str("test-files/agbQuiz.ptcl").unwrap())?;
    println!("{:#?}", effect_quiz);
    println!("--MK7--");
    let effect_kart = EffectFile::from_file(PathBuf::from_str("test-files/Kart.ptcl").unwrap())?;
    println!("{:#?}", effect_kart);
    Ok(())
}
