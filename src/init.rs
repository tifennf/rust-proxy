use config::{Config, File, Value};
use core::panic;
use std::{
    collections::{HashMap, HashSet},
    env,
    fs::OpenOptions,
    io::{ErrorKind, Write},
};

pub fn init_config() -> Config {
    let exe_path = env::current_exe().unwrap();
    let directory_path = exe_path.parent().unwrap().to_str();
    let config_path = format!("{}/Config.toml", directory_path.unwrap());

    check_configfile(&config_path);

    let config_file = File::with_name(&config_path);
    let mut config = Config::default();
    if let Err(e) = config.merge(config_file) {
        panic!("You need to make a Config.toml, error: {}", e);
    }

    config
}

pub fn server_config() -> HashMap<String, Value> {
    let config = init_config();
    let server = config
        .get_table("server")
        .unwrap_or_else(|e| panic!("Error: {}", e));

    server
}

pub fn get_caddr(config: &HashMap<String, Value>) -> String {
    let ip = config.get("ip").cloned().unwrap();
    let port = config.get("port").cloned().unwrap();

    format!("{}:{}", ip, port)
}

pub fn get_ip_whitelist(config: &HashMap<String, Value>) -> HashSet<String> {
    let ip_whitelist = config.get("allowed_ip").cloned();

    let ip_whitelist = match ip_whitelist {
        Some(v) => v.into_array().unwrap(),
        None => panic!("Incorrect Config.toml"),
    };

    ip_whitelist
        .iter()
        .map(|ip| ip.to_string())
        .collect::<HashSet<String>>()
}

pub fn check_configfile(config_path: &str) {
    let default_config =
        b"[server]\nip = \"127.0.0.1\"\nport = \"3000\"\nallowed_ip = [\"127.0.0.1\"]\n";

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(config_path);

    match file {
        Ok(mut file) => {
            file.write_all(default_config)
                .expect("could not write default config");
        }
        Err(e) => {
            if let ErrorKind::AlreadyExists = e.kind() {
                //file already exist, so it's fine
                return ();
            } else {
                panic!("Something went wrong...");
            }
        }
    }
}
