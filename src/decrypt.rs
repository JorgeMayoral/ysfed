use std::{io::Write, path::Path};

use cocoon::Cocoon;

pub fn decrypt_file(password: &[u8], input_path: &Path, output_path: &Path) -> Result<(), String> {
    let cocoon = Cocoon::new(password);
    let mut input_file =
        std::fs::File::open(input_path).map_err(|e| format!("Could not open file: {e:?}"))?;
    match cocoon.parse(&mut input_file) {
        Ok(data) => {
            let mut output_file = std::fs::File::create(output_path)
                .map_err(|e| format!("Could not create file: {e:?}"))?;
            output_file
                .write_all(&data)
                .map_err(|e| format!("Could not write to file: {e:?}"))
        }
        Err(e) => Err(format!("Could not decrypt file: {e:?}")),
    }
}
