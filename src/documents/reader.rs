use std::{
    fs::File,
    io::{Cursor, Read, Seek, SeekFrom},
    path::Path,
};

use crate::env::Config;

use super::Documents;

impl Documents {
    pub fn reader(config: &'static Config) -> Reader {
        Reader { config }
    }
}

pub struct Reader {
    config: &'static Config,
}

impl Reader {
    pub fn read_data_all(&self, collection: &String) -> String {
        let path = Path::new(&self.config.default_path)
            .join(collection)
            .join("data.srb");

        let mut file = File::open(&path).unwrap();
        let _ = file.seek(SeekFrom::Start(10));

        let mut result = String::new();
        let _ = file.read_to_string(&mut result).unwrap();
        // let mut buffer = vec![];
        // let bytes_read = file.read(&mut buffer).unwrap();

        // buffer.resize(bytes_read, 0);

        // let result = String::from_utf8(buffer).expect("Error converting to string");

        result
    }
}
