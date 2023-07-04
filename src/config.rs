use std::net::{SocketAddrV4, Ipv4Addr};
use toml::Value;
use serde::Deserialize;
use log::info;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct Config {
    port_listen: u16,
    self_node_addr: SocketAddrV4,
    near_node_addrs: Vec<SocketAddrV4>, 
}

impl std::convert::From<Value> for Config {
    fn from(v: Value) -> Self {
        let port_listen = v["port_listen"].as_integer().unwrap() as u16;
        let self_node_addr = v["self_node_addr"].as_str().unwrap();
        let self_node_addr = self_node_addr.parse::<SocketAddrV4>().unwrap();
        let near_node_addrs = v["near_node_addrs"].as_array().unwrap();
        let near_node_addrs = near_node_addrs.iter().map(|v| {
            let addr = v.as_str().unwrap();
            addr.parse::<SocketAddrV4>().unwrap()
        }).collect::<Vec<_>>();
        Config {
            port_listen,
            self_node_addr,
            near_node_addrs,
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
                let config: Self = toml::from_str::<Value>(&config).unwrap().into();
                if config.self_node_addr.ip().is_local() {
                    info!("Your self_node_addr seems to be local, please check your config file at ./.hhc/config.toml");
                    std::process::exit(0);
                }
                config
            },
            Err(_) => {
                info!("Config file not found, creating one...");
                let config = include_str!("default_config.toml");
                std::fs::create_dir_all("./.hhc").unwrap();
                std::fs::write("./.hhc/config.toml", config).unwrap();
                info!("Please type in your socket address at self_node_addr in the config file at ./.hhc/config.toml");
                std::process::exit(0);
            }
        };
        config_toml.into()
    }
}

trait JudgeLocal {
    fn is_local(&self) -> bool;
}

impl JudgeLocal for Ipv4Addr {
    fn is_local(&self) -> bool {
        if self.is_loopback() || self.is_private() || self.is_link_local() {
            return true;
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use crate::config::Config;
        let config = Config::init();
        assert_eq!(config.self_node_addr, SocketAddrV4::from_str("192.0.2.1:23").unwrap());
    }

    fn pure_test() {
        let addr = Ipv4Addr::from_str("127.0.0.1").unwrap();
        let n: u32 = addr.into();
    }
}
