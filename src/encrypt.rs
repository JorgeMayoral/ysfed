use std::path::Path;

use cocoon::Cocoon;

pub fn encrypt_file(password: &[u8], input_path: &Path, output_path: &Path) -> Result<(), String> {
    let data = std::fs::read(input_path).map_err(|e| format!("Could not read file: {e:?}"))?;
    let mut cocoon = Cocoon::new(password);
    let mut file =
        std::fs::File::create(output_path).map_err(|e| format!("Could not create file: {e:?}"))?;
    cocoon
        .dump(data, &mut file)
        .map_err(|e| format!("Could not encrypt file: {e:?}"))
}
