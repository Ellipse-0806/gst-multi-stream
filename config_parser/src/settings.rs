use cfg::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct Source {
    uri: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Source {
    pub fn get_uri(&self) -> &String {
        &self.uri
    }
    pub fn get_x_pos(&self) -> &i32 {
        &self.x
    }
    pub fn get_y_pos(&self) -> &i32 {
        &self.y
    }
    pub fn get_width(&self) -> &i32 {
        &self.width
    }
    pub fn get_height(&self) -> &i32 {
        &self.height
    }
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

    pub fn get_sources(&self) -> &Vec<Source> {
        &self.sources
    }
}
