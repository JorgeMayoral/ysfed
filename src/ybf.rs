use std::{fs::File, io::Read, path::PathBuf};

use crate::{error::Error, YBF_CURRENT_VERSION, YBF_MAGIC, YBF_OFFSET_TO_DATA};

#[derive(Debug)]
pub struct Ybf {
    magic: [u8; 8],
    version: u8,
    protected: bool,
    length: u32,
    offset_to_data: u8,
    data: Vec<u8>,
}

impl Default for Ybf {
    fn default() -> Self {
        Self::new(YBF_CURRENT_VERSION, false, vec![])
    }
}

impl Ybf {
    pub fn new(version: u8, protected: bool, data: Vec<u8>) -> Self {
        Self {
            magic: YBF_MAGIC,
            version,
            protected,
            length: data.len() as u32,
            offset_to_data: YBF_OFFSET_TO_DATA,
            data,
        }
    }

    pub fn create_unprotected(data: Vec<u8>) -> Self {
        Self::new(YBF_CURRENT_VERSION, false, data)
    }

    pub fn create_protected(password: &str, data: Vec<u8>) -> Self {
        Self::new(YBF_CURRENT_VERSION, true, Self::encrypt(password, data))
    }

    fn encrypt(password: &str, data: Vec<u8>) -> Vec<u8> {
        let mut protected = vec![];
        for (i, byte) in data.iter().enumerate() {
            protected.push(byte ^ password.as_bytes()[i % password.len()]);
        }
        protected
    }

    fn decrypt(password: &str, data: Vec<u8>) -> Vec<u8> {
        Self::encrypt(password, data)
    }

    pub fn decrypt_data(&self, password: &str) -> Vec<u8> {
        Self::decrypt(password, self.data().to_vec())
    }

    pub fn read_file(path: &PathBuf) -> Result<Self, Error> {
        let mut file = File::open(path).unwrap();
        let mut magic = [0; 8];
        file.read_exact(&mut magic).unwrap();
        if magic != YBF_MAGIC {
            return Err(Error::InvalidFile);
        }
        let mut version = [0; 1];
        file.read_exact(&mut version).unwrap();
        if version[0] != YBF_CURRENT_VERSION {
            return Err(Error::InvalidVersion);
        }
        let mut protected = [0; 1];
        file.read_exact(&mut protected).unwrap();
        let protected = match protected[0] {
            0 => false,
            1 => true,
            _ => panic!("Invalid protected value"),
        };
        let mut length = [0; 4];
        file.read_exact(&mut length).unwrap();
        let mut offset_to_data = [0; 1];
        file.read_exact(&mut offset_to_data).unwrap();
        let mut _offset = [0; YBF_OFFSET_TO_DATA as usize];
        file.read_exact(&mut _offset).unwrap();
        let mut data = vec![];
        file.read_to_end(&mut data).unwrap();
        Ok(Self {
            magic,
            version: version[0],
            protected,
            length: u32::from_le_bytes(length),
            offset_to_data: offset_to_data[0],
            data,
        })
    }

    pub fn read_protected_file(path: &PathBuf, password: &str) -> Result<Self, Error> {
        let file = Self::read_file(path)?;
        let data = Self::decrypt(password, file.data().to_vec());
        Ok(Self::new(file.version, false, data))
    }

    pub(crate) fn to_le_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.magic);
        bytes.extend_from_slice(&self.version.to_le_bytes());
        match self.protected {
            true => bytes.push(1),
            false => bytes.push(0),
        }
        bytes.extend_from_slice(&self.length.to_le_bytes());
        bytes.extend_from_slice(&self.offset_to_data.to_le_bytes());
        bytes.resize(bytes.len() + YBF_OFFSET_TO_DATA as usize, 0);
        bytes.extend_from_slice(self.data.as_slice());
        bytes
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn is_protected(bytes: &[u8]) -> bool {
        bytes.len() >= 16 && bytes[8..16] == YBF_MAGIC
    }

    pub fn is_valid(bytes: &[u8]) -> bool {
        bytes.len() >= 16 && bytes[0..8] == YBF_MAGIC
    }
}
