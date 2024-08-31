use serde::Deserialize;
use std::fs;

#[derive(Debug,Deserialize)]
pub struct MongoConfig{
    pub uri:String,
    pub database:String,
    pub collection:String,
}

#[derive(Debug,Deserialize)]
pub struct Config{
    pub mongodb:MongoConfig,
}

impl Config{
    pub fn from_file(path:&str)->Self{
        let config_str=fs::read_to_string(path).expect("Failed to read config file");
        serde_yaml::from_str(&config_str).expect("Failed to parse config file")
    }
}