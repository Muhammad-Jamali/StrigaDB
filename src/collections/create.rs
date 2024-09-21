use std::{fs, path::Path};

use crate::env::get_config;

pub fn create_collection(name: &str) {
    let config = get_config();
    let path = Path::new(&config.default_path).join(name);

    if path.exists() {
        eprintln!("collection already exists ignoring creation for collection {name}");
        return;
    }

    let _ = fs::create_dir(path);
    println!("{name} collection successfully created");
}

pub fn get_all_collections() {
    let config = get_config();
    let path = Path::new(&config.default_path);

    for entry in fs::read_dir(path).unwrap() {
        println!("{:?}", entry.unwrap().file_name())
    }
}

pub fn get_collection(name: &str) -> bool {
    let config = get_config();
    let path = Path::new(&config.default_path).join(name);

    if !path.exists() {
        println!("path doesn't exists!");
        return false;
    }

    true
}
