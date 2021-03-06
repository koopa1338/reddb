use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::default::Default;

mod bin;
mod json;
mod ron;
mod yaml;

#[cfg(feature = "bin_ser")]
pub use self::bin::Bin;
#[cfg(feature = "json_ser")]
pub use self::json::Json;
#[cfg(feature = "ron_ser")]
pub use self::ron::Ron;
#[cfg(feature = "yaml_ser")]
pub use self::yaml::Yaml;

#[derive(Debug, Clone)]
pub enum Serializers {
    Bin(String),
    Json(String),
    Yaml(String),
    Ron(String),
}

pub trait Serializer<'a>: Default {
    fn format(&self) -> &Serializers;
    fn serialize<T>(&self, val: &T) -> Result<Vec<u8>, Error>
    where
        for<'de> T: Serialize + Deserialize<'de>;

    fn deserialize<T>(&self, val: &[u8]) -> Result<T, Error>
    where
        for<'de> T: Serialize + Deserialize<'de>;
}
