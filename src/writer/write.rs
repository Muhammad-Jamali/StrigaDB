use crate::env::get_config;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn write_to_file(buffer: &[u8], collection: &str) -> bool {
    let config = get_config();
    let max_bytes = config.page_size * 1000;
    let path = Path::new(&config.default_path).join(collection);

    let len = fs::read_dir(path).expect("something went wrong!").count();

    let path = path.join(format!("data{len}.srb"));
    if len == 0 {
        let mut data_file = fs::File::create(&path).expect("creation failed");
        data_file.write(buffer).expect("write failed");
        data_file.write("\n".as_bytes()).expect("write failed");
        return true;
    }

    let len = len - 1;
    let metadata = fs::symlink_metadata(&path).expect("failed to retrieve metadata");

    if metadata.len() < u64::from(max_bytes) {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .expect("Can't open file");

        file.write(buffer).expect("write failed");
        file.write("\n".as_bytes()).expect("write failed");
    }
    false
}
