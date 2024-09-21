mod env;
use std::path::Path;
use std::{fs, io};

use env::{get_config, load_env, Config};

mod writer;
use writer::write::write_to_file;

mod collections;
use collections::create::{create_collection, get_all_collections};

fn main() {
    load_env();

    let config = get_config();
    intialize_db_dir(&config);

    let mut collection = String::new();
    io::stdin()
        .read_line(&mut collection)
        .expect("invalid input");

    let collection = collection.trim();
    create_collection(&collection);
    get_all_collections();

    write_to_file("{\"data\": \"something\"}".as_bytes(), &collection);
    // write_to_file("{\"dataaaa\": \"something\"}".as_bytes());
}

fn intialize_db_dir(config: &Config) {
    let path = Path::new(&config.default_path);

    if !path.exists() {
        fs::create_dir_all(path).expect("Unable to create the necessary directories");
    } else {
        println!("db directory exists skipping directory creation step");
    }
}
