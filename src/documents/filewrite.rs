use super::Documents;
use crate::env::Config;
use std::fs;
use std::io::Write;
use std::path::Path;

impl Documents {
    pub fn write(config: &'static Config) -> FileWrite {
        FileWrite { config }
    }
}

pub struct FileWrite {
    config: &'static Config,
}

impl FileWrite {
    pub fn write_to_file(&self, buffer: &[u8], collection: &str) -> Result<(), &str> {
        let path = Path::new(&self.config.default_path).join(collection);

        if !path.exists() {
            return Err("collection doesn't exists!");
        }

        let path = path.join("data.srb");

        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .expect("Can't open file");

        let _ = file.write_all(buffer).expect("unable to write to file");
        Ok(())
    }
}
