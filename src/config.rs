use std::env;
use yaml_rust::{ YamlLoader, YamlEmitter };
use std::fs;

pub struct Config {
    video_folder_to_transfer: String,
    image_folder_to_transfer: String,
    audio_folder_to_transfer: String,
}

impl Config {
    fn new() -> Self {
        Self {
            video_folder_to_transfer: String::new(),
            image_folder_to_transfer: String::new(),
            audio_folder_to_transfer: String::new(),
        }
    }

    pub fn get_config_from_yaml() -> Self {
        let config_path = env::current_dir().unwrap().join("config.yaml");
        let yaml = fs::read_to_string(config_path).unwrap();
        let docs = YamlLoader::load_from_str(&yaml).unwrap();
        let doc = &docs[0];
        let video_folder_to_transfer = doc["video_folder_to_transfer"]
            .as_str()
            .unwrap()
            .to_string();
        let image_folder_to_transfer = doc["image_folder_to_transfer"]
            .as_str()
            .unwrap()
            .to_string();
        let audio_folder_to_transfer = doc["audio_folder_to_transfer"]
            .as_str()
            .unwrap()
            .to_string();

        Self {
            video_folder_to_transfer,
            image_folder_to_transfer,
            audio_folder_to_transfer,
        }
    }

    pub fn load_default_yaml_config() {
        let config_path = env::current_dir().unwrap().join("config.yaml");
        let yaml = fs::read_to_string(config_path).unwrap();
        let docs = YamlLoader::load_from_str(&yaml).unwrap();
        let doc = &docs[0];
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            video_folder_to_transfer: "Videos".to_string(),
            image_folder_to_transfer: "Images".to_string(),
            audio_folder_to_transfer: "Audio".to_string(),
        }
    }
}
