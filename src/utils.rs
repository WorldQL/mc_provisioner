use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

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

#[derive(Debug)]
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

        let reserved = vec!["level-seed", "motd", "query.port", "server-port"];

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
