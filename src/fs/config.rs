
use std::convert::From;
use std::fs::read_to_string;

use serde::Deserialize;
use serde_json::from_str;

fn get_default_verbose() -> u8 { 0 }
fn get_default_listen() -> String { String::from("127.0.0.1:9000") }
fn get_default_cluster() -> Cluster { Cluster::new() }
fn get_default_id() -> String { String::from("cluster-1") }
fn get_default_enabled() -> bool { false }

#[derive(Debug, Deserialize, Clone)]
pub struct Cluster {
    #[serde(default = "get_default_enabled")]
    pub enabled: bool,

    id: String,
    nodes: Option<Vec<String>>,
}

impl Cluster {
    pub fn new() -> Self {
        Self {
            enabled: get_default_enabled(),
            id: get_default_id(),
            nodes: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "get_default_verbose")]
    verbose: u8,

    #[serde(default = "get_default_listen")]
    pub listen: String,

    #[serde(default = "get_default_cluster")]
    pub cluster: Cluster,
}

impl Config {
    pub fn new() -> Self {
        Self {
            verbose: get_default_verbose(),
            listen: get_default_listen(),
            cluster: get_default_cluster(),
        }
    }
}

impl From<&String> for Config {
    fn from(data: &String) -> Self {
        from_str(data).expect("JSON was not well-formatted")
    }
}

impl From<Option<String>> for Config {
    fn from(path_o: Option<String>) -> Self {
        match path_o {
            Some(path) => {
                let data: String = read_to_string(path).expect("Unable to read file");
                Self::from(&data)
            },
            None => Self::new(),
        }
    }
}
