use once_cell::sync::Lazy;
use reqwest::blocking::{Client, ClientBuilder};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
pub(super) static CLIENT: Lazy<Client> = Lazy::new(|| {
    ClientBuilder::new()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap()
});
