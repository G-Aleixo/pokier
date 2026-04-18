use serde::{
    Serialize,
    Deserialize,
};

#[derive(Serialize, Deserialize, Debug)]
#[derive(crate::encode::Encode)]
pub struct ServerConfig {
    pub ip: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(crate::encode::Encode)]
pub struct GameConfig {
    pub starting_chips: u32,
    pub max_players: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(crate::encode::Encode)]
pub struct Config {
    pub server: ServerConfig,
    pub game: GameConfig,
}

impl Config {
    pub fn load(path: &str) -> Self {
        use std::fs;
        let content = fs::read_to_string(path).expect("Failed to read config file");
        toml::from_str(&content).expect("Failed to parse config")
    }

    pub fn load_from_str(content: &str) -> Self {
        toml::from_str(&content).expect("Failed to parse config")
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::load_from_str(include_str!("assets/default_config.toml"))
    }
}

/// Tries to get a config file from the config/settings.toml file and creates a default one if none exist
pub fn get_config() -> Config {
    let exe_path = std::env::current_exe().expect("Couldn't get current exe path");
    let exe_dir = exe_path.parent().expect("No parent directory");

    let config_dir = exe_dir.join("config");
    std::fs::create_dir_all(&config_dir).expect("Failed to create config directory");

    let config_file_path = config_dir.join("settings.toml");
    if std::fs::exists(&config_file_path).unwrap() {
        return Config::load(&config_file_path.to_str().unwrap());
    } else {
        std::fs::write(&config_file_path, toml::to_string_pretty(&Config::default()).unwrap()).unwrap();
        return Config::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn load_from_file() {
        // setup a temporary directory
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("config.toml");

        // write a sample file
        let mut file = File::create(&file_path).unwrap();
        writeln!(
            file,
            r#"
            [server]
            ip = "127.0.0.1"
            port = 8080

            [game]
            starting_chips = 500
            max_players = 4
            "#
        ).unwrap();

        let config = Config::load(file_path.to_str().unwrap());

        assert_eq!(config.server.ip, "127.0.0.1");
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.game.starting_chips, 500);
        assert_eq!(config.game.max_players, 4);
    }
}