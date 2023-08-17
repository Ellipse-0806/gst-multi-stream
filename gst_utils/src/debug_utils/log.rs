use log::info;
use std::{env, fs};

const ENV_PATH: &str = "GST_DEBUG_FILE";
const DEFAULT_SAVE_PATH: &str = "logs/log";
const DEFAULT_LOG_FILE_NAME: &str = "gst.log";
const DEBUG_LEVEL_PATH: &str = "GST_DEBUG";

pub fn setup() -> std::io::Result<()> {
    if env::var(DEBUG_LEVEL_PATH).is_ok() {
        if env::var(ENV_PATH).is_err() {
            env::set_var(
                ENV_PATH,
                format!("{}/{}", DEFAULT_SAVE_PATH, DEFAULT_LOG_FILE_NAME),
            );
        }
        info!("log-file path : {}", env::var(ENV_PATH).unwrap());
        fs::create_dir_all(DEFAULT_SAVE_PATH)?;
    } else {
        info!("log : ✖");
    }
    Ok(())
}
