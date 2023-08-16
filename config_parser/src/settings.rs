use cfg::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct Source {
    uri: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct Outputfmt {
    format: String,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct AppConfig {
    // debug: bool,
    sources: Vec<Source>,
    output: Outputfmt,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .add_source(Environment::with_prefix("app"))
            .build()?;

        // println!("debug: {:?}", s.get_bool("debug"));
        println!("sources: {:#?}", s.get::<Vec<Source>>("sources"));
        println!("output: {:#?}", s.get::<String>("output.format"));

        s.try_deserialize()
    }
}
