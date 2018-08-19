use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::io::{Read, Write};
use std::path;

const CONFIG_REPO: &'static str = "repo_path";
const CONFIG_EDITOR: &'static str = "editor_path";

pub enum ConfigFile {
    Repo,
    Editor,
}

pub struct FileHandler;

pub trait ConfigManagement {
    fn config_dir_create(&self) -> io::Result<()>;
    fn config_dir_exists(&self) -> bool;
    fn read_from_config(&self, file: ConfigFile) -> io::Result<String>;
    fn write_to_config(&self, file: ConfigFile, value: String) -> io::Result<()>;
}

pub trait FileManagement {
    fn path_exists(&self, path: &str) -> bool;
    fn rm_file(&self, file: ConfigFile) -> io::Result<()>;
}

impl ConfigManagement for FileHandler {
    fn config_dir_create(&self) -> io::Result<()> {
        fs::create_dir_all(config_dir_path())
    }

    fn config_dir_exists(&self) -> bool {
        self.path_exists(&config_dir_path())
    }

    fn read_from_config(&self, file: ConfigFile) -> io::Result<String> {
        let file_name = config_name(file);
        let mut file = fs::File::open(&file_name)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect(&format!("Unable to read file at: {}", file_name));
        if contents.ends_with("\n") {
            contents.pop().expect("File is empty");
        }

        Ok(contents)
    }

    fn write_to_config(&self, file: ConfigFile, value: String) -> io::Result<()> {
        let file_name: String = config_name(file);
        let path = path::Path::new(&file_name);

        let mut file = match fs::File::create(&path) {
            Err(e) => panic!("Couldn't create {}: {}", path.display(), e.description()),
            Ok(file) => file,
        };

        match file.write_all(value.as_bytes()) {
            Err(e) => panic!("Couldn't write to {}: {}", path.display(), e.description()),
            Ok(_) => Ok(()),
        }
    }
}

impl FileManagement for FileHandler {
    fn path_exists(&self, path: &str) -> bool {
        fs::metadata(path).is_ok()
    }

    fn rm_file(&self, file: ConfigFile) -> io::Result<()> {
        let path: String = config_file_path(config_name(file));
        if self.path_exists(&path) {
            fs::remove_file(&path)?;
            Ok(())
        } else {
            let invalid_path = io::Error::new(
                ErrorKind::NotFound,
                format!("Path does not exist: {}", path),
            );
            Err(invalid_path)
        }
    }
}

fn config_name(file: ConfigFile) -> String {
    match file {
        ConfigFile::Repo => config_file_path(CONFIG_REPO.to_string()),
        ConfigFile::Editor => config_file_path(CONFIG_EDITOR.to_string()),
    }
}

fn config_file_path(file_name: String) -> String {
    match env::home_dir() {
        Some(location) => format!(
            "{home}/{eureka}/{file_name}",
            home = location.display(),
            eureka = ".eureka",
            file_name = file_name
        ),
        None => panic!("Could not resolve your $HOME directory"),
    }
}

fn config_dir_path() -> String {
    match env::home_dir() {
        Some(home_dir) => format!("{}/{}", home_dir.display(), ".eureka"),
        None => panic!("Could not resolve your $HOME directory"),
    }
}
