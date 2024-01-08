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

pub fn decrypt_input(password: &[u8], input: &[u8]) -> Result<String, String> {
    let cocoon = Cocoon::new(password);
    let output = cocoon.unwrap(input);
    if output.is_err() {
        let e = output.err().unwrap();
        return Err(format!("Could not decrypt input: {e:?}"));
    }
    let output = output.unwrap();
    let output = String::from_utf8(output);
    match output {
        Ok(output) => Ok(output),
        Err(e) => Err(format!("Could not decrypt input: {e:?}")),
    }
}

#[cfg(test)]
mod tests {
    use crate::encrypt::encrypt_input;

    use super::*;

    #[test]
    fn test_decrypt_input_ok() {
        let password = "password".as_bytes();
        let input = "This is a test".to_string();
        let encrypted = encrypt_input(password, input.clone());
        assert!(encrypted.is_ok());
        let encrypted = encrypted.unwrap();
        let decrypted = decrypt_input(password, &encrypted);
        assert!(decrypted.is_ok());
        let decrypted = decrypted.unwrap();
        assert_eq!(decrypted, input);
    }
}
