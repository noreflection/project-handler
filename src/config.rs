use serde_derive::Deserialize;
use toml;
use std::env;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub repositories: Repositories,
}

#[derive(Deserialize)]
pub struct Repositories {
    pub github: String,
    pub travis: Option<String>,
}

pub fn test_toml() -> Config {
    let config_string = fs::read_to_string("Cargo.toml")
        .expect("could not read the toml file");

    let toml_config: Config = toml::from_str(&*config_string).unwrap();
    println!("test setting from Cargo.toml:\n{}", toml_config.repositories.github);
    toml_config
    //get repository data from toml config
}