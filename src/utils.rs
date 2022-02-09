use std::path::PathBuf;

type ServerInfo = (u8, u16, PathBuf, String);

pub fn server_iter(
    server_count: u8,
    start_port: u16,
    directory_template: &str,
) -> impl Iterator<Item = ServerInfo> + '_ {
    (1..=server_count).into_iter().map(move |idx| {
        let port = start_port + (u16::from(idx) - 1);
        let motd = format!("{} {}", directory_template, idx);

        let directory = format!("{}_{}", directory_template, port);
        let directory = directory.to_lowercase().replace(' ', "_");
        let directory = PathBuf::from(directory);

        (idx, port, directory, motd)
    })
}
