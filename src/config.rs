use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub title: String,
    pub server: Server,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub ports: Vec<u16>,
    pub location: String,
}

type TomlResult = Result<Config, toml::de::Error>;

impl Config {
    pub fn from_toml(toml: &str) -> TomlResult {
        toml::from_str(toml)
    }
}

impl Server {
    pub fn socket_addresses(&self) -> Vec<SocketAddr> {
        self.ports
            .clone()
            .into_iter()
            .map(|port| SocketAddr::from(([127, 0, 0, 1], port)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_config_from_toml_str() {
        let toml = "
title = \"Tomcrab TOML configuration example\"

[server]
ports = [80, 443, 8888]
location = \"./index.html\"
";

        let config = Config::from_toml(toml);
        println!("{:?}", config);
        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.title, "Tomcrab TOML configuration example");

        let server = config.server;
        assert_eq!(server.ports.len(), 3);
        assert_eq!(server.ports[0], 80);
        assert_eq!(server.ports[1], 443);
        assert_eq!(server.ports[2], 8888);
        assert_eq!(server.location, "./index.html");
        assert_eq!(server.socket_addresses().len(), 3);
    }
}
