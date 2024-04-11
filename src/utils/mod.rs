use std::fs;
use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


pub trait JsonConfigs: Default + Serialize + for<'a> Deserialize<'a> {
    fn from_file(filename: &str) -> Self {
        if fs::metadata(filename).is_err() {
            println!("[!] {} not found", filename);
            return Self::default()
        }
        let file_contents: String = fs::read_to_string(filename).unwrap();
        serde_json::from_str::<Self>(&file_contents.clone()).unwrap_or_else(|e| {
            println!("[!] Config file is corrupted: {:?}", e);
            Self::default()
        })
    }
    fn into_file(&self, filename: &str) -> Result<()> {
        let mut file = File::create(filename).expect("[!] Can't create config file");
        let data = serde_json::to_string(self).unwrap();
        file.write_all(data.as_bytes()).unwrap();
        Ok(())
    }
}