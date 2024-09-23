mod env;
use std::path::Path;
use std::{fs, io};

use env::{get_config, load_env, Config};

mod collections;
use collections::Collection;

mod documents;
use documents::Documents;

fn main() {
    load_env();
    let config = get_config();

    intialize_db_dir(&config);
    let collection_create = Collection::create(config);
    let document_write = Documents::write(config);
    let document_read = Documents::reader(config);

    println!(
        "\
    Options to choose from: 
    1) Create a new collection
    2) List all collection
    3) Add a new document
    4) List all Documents of a collection"
    );

    loop {
        println!("Choice: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error reading input");

        let input: i32 = input.trim().parse().unwrap_or_default();

        if input < 1 {
            eprintln!("invalid input please enter a valid one");
            continue;
        }

        match input {
            1 => match collection_create.create_collection(&get_collection_name()) {
                Ok(val) => println!("{val}"),
                Err(e) => eprintln!("{e}"),
            },
            2 => println!("{:?}", collection_create.get_all_collections()),
            3 => {
                if let Err(e) = document_write.write_to_file(
                    "{\"data\": \"something\"}".as_bytes(),
                    &get_collection_name(),
                ) {
                    eprint!("{e}");
                }
            }
            4 => println!("{}", document_read.read_data_all(&get_collection_name())),
            _ => println!("unkown option"),
        }

        println!();
    }
}

fn get_collection_name() -> String {
    println!("Enter collection name: ");
    let mut collection = String::new();
    io::stdin()
        .read_line(&mut collection)
        .expect("invalid input");

    String::from(collection.trim())
}

fn intialize_db_dir(config: &Config) {
    let path = Path::new(&config.default_path);

    if !path.exists() {
        fs::create_dir_all(path).expect("Unable to create the necessary directories");
    } else {
        println!("db directory exists skipping directory creation step");
    }
}
