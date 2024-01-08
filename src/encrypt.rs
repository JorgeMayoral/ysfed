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

pub fn encrypt_input(password: &[u8], input: String) -> Result<Vec<u8>, String> {
    let mut cocoon = Cocoon::new(password);
    let data = input.as_bytes();
    let output = cocoon.wrap(data);
    match output {
        Ok(output) => Ok(output),
        Err(e) => Err(format!("Could not encrypt input: {e:?}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_input_ok() {
        let password = "password".as_bytes();
        let input = "This is a test".to_string();
        let output = encrypt_input(password, input.clone());
        assert!(output.is_ok());
        let output = output.unwrap();
        assert_ne!(output, input.as_bytes());
    }
}
