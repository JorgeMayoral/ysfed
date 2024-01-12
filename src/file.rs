use std::{fs::File, io::Write, path::PathBuf};

use anyhow::Result;

use crate::Ybf;

pub fn write_file(ybf: &Ybf, output_file_path: PathBuf) -> Result<()> {
    let mut output_file = File::create(output_file_path)?;
    output_file.write_all(&ybf.to_le_bytes())?;

    Ok(())
}

pub fn read_file(input_file_path: PathBuf, password: Option<&str>) -> Result<Ybf> {
    let input_data = std::fs::read(&input_file_path)?;
    if !Ybf::is_valid(&input_data) {
        anyhow::bail!("Invalid file");
    }
    let ybf = match Ybf::is_protected(&input_data) {
        true => Ybf::read_protected_file(&input_file_path, password.unwrap()).unwrap(),
        false => Ybf::read_file(&input_file_path).unwrap(),
    };

    Ok(ybf)
}
