pub mod dot;
pub mod log;
use env_logger;
use std::env;

const DEBUG_LEVEL_PATH: &str = "GST_DEBUG";

pub fn init_debug_utils() {
    env::set_var(DEBUG_LEVEL_PATH, "*:3");
    env_logger::init();
    dot::setup().unwrap();
    log::setup().unwrap();
}

pub fn enable_debug_utils(pipeline: &gst::Pipeline) {
    dot::enable(pipeline);
}
