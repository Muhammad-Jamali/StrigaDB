use std::env;
use std::fs;
use std::sync::Once;

pub struct Config {
    pub default_path: String,
    pub page_size: u32,
}

static INIT: Once = Once::new();
static mut CONFIG: Option<Config> = None;

pub fn load_env() {
    let envs = fs::read_to_string(".env").expect(".env file not found");

    for line in envs.lines() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }

        let mut iter = line.split('#').next().unwrap().split('=');

        if let (Some(key), Some(value)) = (iter.next(), iter.next()) {
            env::set_var(key, value);
        }
    }

    INIT.call_once(|| unsafe {
        CONFIG = Some(Config {
            default_path: env::var("DEFAULT_PATH").unwrap(),
            page_size: env::var("PAGE_SIZE")
                .unwrap_or_else(|_| "8".to_string())
                .trim()
                .parse()
                .expect("page should be a number"),
        });
    });
}

pub fn get_config() -> &'static Config {
    unsafe { CONFIG.as_ref().expect("CONFIG should be initialized") }
}
