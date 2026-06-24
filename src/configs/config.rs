use std::{env, fs, io::BufReader, path::PathBuf, str::FromStr};

use anyhow::{Context, Ok, Result, bail};
use serde::{Deserialize, Serialize};

pub enum FileChosen {
    JSON,
    TOML,
}

#[derive(Debug, Clone)]
pub struct Commands {
    pub cmd: String,
    pub args: Vec<String>,
}

#[derive(Serialize, Debug, Deserialize)]
struct Config {
    pub command: String,
    pub watch: Vec<String>,
    pub dir: Option<String>,
    pub verbose: bool,
}

#[derive(Debug, Clone)]
pub struct Configs {
    pub watch: Vec<String>,
    pub cmd: Commands,
    pub dir: PathBuf,
    pub verbose: bool,
}

impl Configs {
    pub fn load_file() -> Result<Self> {
        let config = Self::get_config()?;

        let cmd = Self::parse_user_command(config.command)
            .with_context(|| "You haven't configured the command to run")?
            .unwrap();
        let path = resolve_directory(config.dir);

        Ok(Configs {
            cmd,
            watch: config.watch,
            dir: path,
            verbose: config.verbose,
        })
    }

    fn get_config() -> Result<Config> {
        let filetype: FileChosen = Self::detect_file().with_context(|| "Something went wrong while loading the config file.\nAre you sure you've initialized one?")?;

        match filetype {
            FileChosen::JSON => return Ok(Self::read_json()?),
            FileChosen::TOML => return Ok(Self::read_toml()?),
        }
    }

    fn read_toml() -> Result<Config> {
        let content =
            fs::read_to_string("watch.toml").with_context(|| "Could not find toml config")?;
        let config: Config =
            toml::from_str(&content).with_context(|| "Failed to parse toml config")?;
        return Ok(config);
    }
    fn read_json() -> Result<Config> {
        let file = fs::File::open("watch.json")?;
        let reader = BufReader::new(file);

        let config: Config =
            serde_json::from_reader(reader).with_context(|| "Error while parsing config file")?;
        return Ok(config);
    }
    fn detect_file() -> Result<FileChosen> {
        let exists = check_existence("watch.toml");
        if exists {
            return Ok(FileChosen::TOML);
        };
        let exists = check_existence("watch.json");
        if exists {
            return Ok(FileChosen::JSON);
        };
        bail!("No Supported config type is found")
    }

    fn parse_user_command(input: String) -> Result<Option<Commands>> {
        let p = shell_words::split(&input).with_context(
            || "Error while parsing the your command input, please ensure it's a valid command",
        )?;
        match p.as_slice() {
            [] => Ok(None),
            [n, a @ ..] => Ok(Some(Commands {
                cmd: n.to_string(),
                args: a.to_vec(),
            })),
        }
    }
}

#[inline(always)]
fn resolve_directory(dir: Option<String>) -> PathBuf {
    if let Some(d) = dir
        && d != ".".to_string()
    {
        return PathBuf::from_str(&d)
            .with_context(|| "The provided dir is not valid")
            .unwrap();
    };
    env::current_dir()
        .with_context(|| "Couldn't automatically resolve the CWD, please set it explicitly")
        .unwrap()
}

#[inline(always)]
fn check_existence(filename: &str) -> bool {
    return fs::exists(filename).unwrap_or(false);
}
