use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use toml;

pub const RUSTYGO_CONFIG: &str = "RUSTYGO_CONFIG";

lazy_static! {
    /// Lazy static reference to core configuration loaded from `rustygo_config.toml`.
    pub static ref CORE_CONFIG: Config = open_config_file(Path::new(std::env::var(RUSTYGO_CONFIG).unwrap().as_str()));
}

fn open_config_file(path: &Path) -> Config {
    parse_toml(
        &fs::read_to_string(path).expect(
            format!(
                "Unable to find the file {}. Please check the path is correct and this file exists",
                RUSTYGO_CONFIG
            )
            .as_str(),
        ),
    )
    .expect(
        format!(
            "Unable to read the file {}. Please check the contents of this file.",
            RUSTYGO_CONFIG
        )
        .as_str(),
    )
}

/// Parses and returns core configuration.
fn parse_toml(toml_str: &str) -> Result<Config, toml::de::Error> {
    toml::from_str::<Config>(toml_str)
}

pub fn core_config() -> &'static CORE_CONFIG {
    &CORE_CONFIG
}

// #[derive(Serialize, Deserialize, PartialEq, Debug)]
// pub struct AgentConfig {
//     /// Config param for Agent

// }

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct RLConfig {
    pub INIT_Q_VALUES: f32,
    pub SARSA_N: u8,
    pub GAMMA: f32,
    pub ALPHA: f32,
    pub EPSILON: f32,
    pub MULTI_POLICY: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    // pub agent: AgentConfig,
    pub rl: RLConfig,
}
