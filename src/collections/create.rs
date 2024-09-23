use crate::env::Config;
use std::{
    fs::{self, File},
    io,
    path::Path,
};

use super::Collection;

impl Collection {
    pub fn create(config: &'static Config) -> Create {
        Create { config }
    }
}

pub struct Create {
    config: &'static Config,
}

impl Create {
    pub fn create_collection(&self, name: &str) -> Result<String, io::Error> {
        let path = Path::new(&self.config.default_path).join(name);

        if path.exists() {
            return Ok(String::from("collection already exists!"));
        }

        fs::create_dir_all(&path)?;
        let path = path.join("data.srb");
        File::create(path)?;
        Ok(String::from("collection created successfully!"))
    }

    pub fn get_all_collections(&self) -> Vec<String> {
        let path = Path::new(&self.config.default_path);

        let mut collections: Vec<String> = vec![];

        for entry in fs::read_dir(path).unwrap() {
            collections.push(entry.unwrap().file_name().into_string().unwrap());
        }
        collections
    }
}
