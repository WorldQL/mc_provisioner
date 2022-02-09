use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ServerMemory(String, u64);

// region: Traits
impl Display for ServerMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq for ServerMemory {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl Eq for ServerMemory {}

impl PartialOrd for ServerMemory {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

impl Ord for ServerMemory {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}
// endregion

// region: Parsing
#[derive(Debug, Error)]
pub enum MemoryParseError {
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
        let value = parsed * multi;

        let mem = ServerMemory(memory.to_owned(), value);
        Ok(mem)
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
// endregion

// region: Tests
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    macro_rules! normalize_mem_size_ok {
        ($input:tt, $expected:tt) => {
            let input = $input;
            let result = super::ServerMemory::from_str(&input);

            assert!(result.is_ok());
            assert_eq!(result.unwrap().1, $expected);
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
// endregion
