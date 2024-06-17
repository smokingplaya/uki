use std::{env, path::MAIN_SEPARATOR};
use colored::{ColoredString, Colorize};
use crate::{config, Arguments};

// fuck \t, all my homies use..
const TABULATION: &str = "    ";

trait PathSplit {
    fn get_file_from_path(&self) -> &str;
}

impl PathSplit for String {
    fn get_file_from_path(&self) -> &str {
        self
        .rsplit(MAIN_SEPARATOR)
        .next()
        .unwrap_or(&self)
    }
}

pub trait PinkColor {
    fn accent_str(&self) -> ColoredString;
    fn accent(&self) -> String;
}

impl PinkColor for String {
    fn accent_str(&self) -> ColoredString {
        self.magenta().bold()
    }

    fn accent(&self) -> String {
        self.accent_str().to_string()
    }
}

impl PinkColor for &str {
    fn accent_str(&self) -> ColoredString {
        self.magenta().bold()
    }

    fn accent(&self) -> String {
        self.magenta().bold().to_string()
    }
}

pub fn execute(_: Arguments) {
    match config::get(true) {
        Some(config) => {
            println!("Current workspace: {}", env::current_dir().unwrap().display().to_string().get_file_from_path().to_string().accent_str());
            println!("Config {} by {}", config.name.accent_str(), config.authors.join(", ").to_string().accent_str());

            match config.description {
                Some(desc) => println!("Description: {}", desc.accent_str()),
                None => {}
            }

            println!();

            //println!("Presets:\n {}", config.presets.keys().cloned().collect::<Vec<_>>().join(", ").accent_str());

            println!("Presets:");
            config.presets.into_iter().for_each(|(k, v)| {
                let mut args: String = String::new();

                if let Some(arg) = v.arguments {
                    args = format!("({})", arg.join(", "));
                }

                println!("{}- {} {}", TABULATION, k.accent_str(), args);
            })
        },
        None => return
    }

}