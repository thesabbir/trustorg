use serde_json::Value;
use std::fs;

pub struct ToConfig {
    pub port: u16,
    pub secrete: String,
    pub admin_email: String,
    pub admin_password: String,
    pub proxy: String,
}

#[allow(unused_variables)]
pub fn read_config() -> ToConfig {
    let config_path = "etc/trustorg.json";
    let json_config = {
        // Load the first file into a string.
        let content = fs::read_to_string(&config_path).unwrap();
        // Parse the string into a dynamically-typed JSON structure.
        serde_json::from_str::<Value>(&content).unwrap()
    };
    let port = json_config["port"].as_u64().unwrap() as u16;
    let admin_email = json_config["admin"]["email"].as_str().unwrap().to_string();
    let admin_password = json_config["admin"]["password"]
        .as_str()
        .unwrap()
        .to_string();
    let secrete = json_config["secrete"].as_str().unwrap().to_string();
    let proxy = json_config["proxy"].as_str().unwrap().to_string();

    return ToConfig {
        port,
        secrete,
        admin_email,
        admin_password,
        proxy,
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
