use std::collections::BTreeMap;
use std::fmt::Display;
use std::num::ParseIntError;
use std::path::PathBuf;
use std::str::FromStr;

use color_eyre::Result;
use serde::Deserialize;
use thiserror::Error;

type ServerInfo = (u8, u16, PathBuf, String);

pub fn server_iter(
    server_count: u8,
    start_port: u16,
    directory_template: &str,
) -> impl Iterator<Item = ServerInfo> + '_ {
    (1..=server_count).into_iter().map(move |idx| {
        let port = start_port + (idx as u16 - 1);
        let motd = format!("{} {}", directory_template, idx);

        let directory = format!("{}_{}", directory_template, port);
        let directory = directory.to_lowercase().replace(' ', "_");
        let directory = PathBuf::from(directory);

        (idx, port, directory, motd)
    })
}

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

        let first = split.get(0).unwrap().to_string();
        let second = split.get(1).unwrap().to_string();

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

#[derive(Debug)]
pub struct ServerMemory(u64);

impl ServerMemory {
    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Error)]
enum MemoryParseError {
    #[error("string cannot be empty")]
    EmptyString,

    #[error("unknown size suffix: {0}")]
    InvalidSize(char),

    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

impl FromStr for ServerMemory {
    type Err = MemoryParseError;

    fn from_str(memory: &str) -> Result<Self, Self::Err> {
        if memory.is_empty() {
            return Err(MemoryParseError::EmptyString);
        }

        let len = memory.len();
        let number = &memory[..len - 1];
        let scale = (&memory[len - 1..]).to_lowercase().chars().next().unwrap();

        let multi = match scale {
            'k' => 1024,
            'm' => u64::pow(1024, 2),
            'g' => u64::pow(1024, 3),

            _ => return Err(MemoryParseError::InvalidSize(scale)),
        };

        let parsed = number.parse::<u64>()?;
        Ok(ServerMemory(parsed * multi))
    }
}

impl From<&str> for ServerMemory {
    fn from(memory: &str) -> Self {
        Self::from_str(memory).unwrap()
    }
}

impl<'de> Deserialize<'de> for ServerMemory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;
        str.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    macro_rules! normalize_mem_size_ok {
        ($input:tt, $expected:tt) => {
            let input = $input;
            let result = super::ServerMemory::from_str(&input);

            assert!(result.is_ok());
            assert_eq!(result.unwrap().value(), $expected);
        };
    }

    #[test]
    fn test_normalize_mem_size_ok() {
        normalize_mem_size_ok!("1k", 1024);
        normalize_mem_size_ok!("1K", 1024);
        normalize_mem_size_ok!("10k", 10240);
        normalize_mem_size_ok!("10K", 10240);
        normalize_mem_size_ok!("10m", 10485760);
        normalize_mem_size_ok!("10M", 10485760);
        normalize_mem_size_ok!("10g", 10737418240);
        normalize_mem_size_ok!("10G", 10737418240);
    }

    macro_rules! normalize_mem_size_err {
        ($input:tt) => {
            let input = $input;
            let result = super::ServerMemory::from_str(&input);

            assert!(result.is_err());
        };
    }

    #[test]
    fn test_normalize_mem_size_err() {
        normalize_mem_size_err!("");
        normalize_mem_size_err!("10");
        normalize_mem_size_err!("abc");
        normalize_mem_size_err!("10kb");
        normalize_mem_size_err!("10KB");
        normalize_mem_size_err!("10gb");
        normalize_mem_size_err!("10GB");
    }
}
