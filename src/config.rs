use std::net::{SocketAddrV4, Ipv4Addr};
use toml::Value;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    self_addr: SocketAddrV4,
    nodes_addr: Vec<SocketAddrV4>, 
}

impl std::convert::From<Value> for Config {
    fn from(v: Value) -> Self {
        let self_addr = v["self_addr"].as_str().unwrap();
        let self_addr = self_addr.parse::<SocketAddrV4>().unwrap();
        let nodes_addr = v["nodes_addr"].as_array().unwrap();
        let nodes_addr = nodes_addr.iter().map(|v| {
            let addr = v.as_str().unwrap();
            addr.parse::<SocketAddrV4>().unwrap()
        }).collect::<Vec<_>>();
        Config {
            self_addr,
            nodes_addr,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let config = include_str!("default_config.toml");
        let config = toml::from_str::<Value>(config).unwrap();
        config.into()
    }
}

impl Config {
    pub fn init() -> Self {
        let config_toml = match std::fs::metadata("./.hhc/config.toml") {
            Ok(_) => {
                let config = std::fs::read_to_string("./.hhc/config.toml").unwrap();
                let config = toml::from_str::<Value>(&config).unwrap();
                config
            },
            Err(_) => {
                println!("Config file not found, creating one...");
                let config = include_str!("default_config.toml");
                std::fs::create_dir_all("./.hhc").unwrap();
                std::fs::write("./.hhc/config.toml", config).unwrap();
                println!("Please type in your socket address at self_addr in the config file at ./.hhc/config.toml");
                std::process::exit(0);
            }
        };
        config_toml.into()
    }
}

#[cfg(test)]
mod tests {
    use std::{net::SocketAddrV4, str::FromStr};

    #[test]
    fn it_works() {
        use crate::config::Config;
        let config = Config::init();
        assert_eq!(config.self_addr, SocketAddrV4::from_str("127.0.0.1:23").unwrap());
    }
}