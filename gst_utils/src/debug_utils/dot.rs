use log::info;
use std::{env, fs};

const ENV_PATH: &str = "GST_DEBUG_DUMP_DOT_DIR";
const DEFAULT_SAVE_PATH: &str = "logs/temp";
const FILE_NAME: &str = "pipeline";

pub fn setup() -> std::io::Result<()> {
    if env::var(ENV_PATH).is_err() {
        env::set_var(ENV_PATH, DEFAULT_SAVE_PATH);
    }
    info!("dot-file path : {}", env::var(ENV_PATH).unwrap());

    fs::create_dir_all(DEFAULT_SAVE_PATH)?;
    Ok(())
}

pub fn enable(pipeline: &gst::Pipeline) {
    gst::debug_bin_to_dot_file(pipeline, gst::DebugGraphDetails::all(), FILE_NAME);
}
