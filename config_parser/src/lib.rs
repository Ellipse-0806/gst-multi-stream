mod settings;

use cfg::ConfigError;
use settings::AppConfig;

pub fn load() -> Result<AppConfig, ConfigError> {
    let config = AppConfig::new().unwrap();
    // println!("{:#?}", config);
    Ok(config)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
