use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;

use color_eyre::Result;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ServerProperty(String, String);

impl Display for ServerProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.0, self.1)
    }
}

impl FromStr for ServerProperty {
    type Err = ServerPropertyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split('=').collect::<Vec<_>>();
        if split.len() != 2 {
            return Err(ServerPropertyError::InvalidProperty);
        }

        let first = (*split.get(0).unwrap()).to_owned();
        let second = (*split.get(1).unwrap()).to_owned();

        let reserved = vec![
            "level-seed",
            "motd",
            "query.port",
            "server-port",
            "white-list",
            "level-name",
        ];

        let lower = first.to_lowercase();
        if reserved.contains(&lower.as_ref()) {
            return Err(ServerPropertyError::DisallowedProperty(lower));
        }

        Ok(Self(first, second))
    }
}

#[derive(Debug, Error)]
pub enum ServerPropertyError {
    #[error("invalid server property")]
    InvalidProperty,

    #[error("disallowed property: {0}")]
    DisallowedProperty(String),
}

pub fn map_to_properties(map: BTreeMap<String, String>) -> Result<Vec<ServerProperty>> {
    let mut vec = Vec::with_capacity(map.len());
    for (key, value) in map.into_iter() {
        let prop = format!("{}={}", key, value).parse::<ServerProperty>()?;
        vec.push(prop);
    }

    Ok(vec)
}

pub fn properties_to_map(vec: Vec<ServerProperty>) -> BTreeMap<String, String> {
    vec.into_iter()
        .map(|p| (p.0, p.1))
        .collect::<BTreeMap<_, _>>()
}
