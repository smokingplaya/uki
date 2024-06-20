use std::{collections::HashMap, fs::File, io::Read, process::{Command, Stdio}};
use serde::{Deserialize, Serialize};

use crate::{commands::list::PinkColor, Arguments, PreparedArguments, UKI_FOLDER};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    pub name: String,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub runner: String,

    pub presets: HashMap<String, Preset>
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Preset {
    pub arguments: Option<Vec<String>>,
    pub runner: Option<String>,
    pub execute: Vec<String>
}

/*
    Default
*/

const CONFIG_FILE: &'static str = "config.yml";

fn parse_error(message: &str, should_print: bool) {
    if !should_print {
        return
    }

    println!("uki: unable to parse config: {}", message);
}

pub fn get_path() -> String {
    format!("{UKI_FOLDER}\\{CONFIG_FILE}")
}

pub fn get(should_print_error: bool) -> Option<Config> {
    match File::open(get_path()) {
        Ok(mut file) => {
            let mut content = String::new();
            let _ = file.read_to_string(&mut content);

            let yml = serde_yaml::from_str::<Config>(&content);

            if let Ok(config) = yml {
                return Some(config)
            }

            parse_error(&yml.err().unwrap().to_string(), should_print_error);
        },
        Err(err) => parse_error(&err.to_string(), should_print_error),
    }

    None
}

fn get_argument(runner: &str) -> &str {
    match runner {
        "cmd" => "/C",
        "powershell" => "-Command",
        "bash" => "-c",
        "zsh" => "-c",
        _ => ""
    }
}

fn format_string(to_format: String, args: &HashMap<String, String>) -> String {
    let mut result = to_format.clone();

    args.into_iter().for_each(|(k, value)| {
        result = result.replace(format!("${}", k).as_str(), value);
    });

    result
}

fn handle_command(runner: String, cmd: String, args: PreparedArguments) {
    let arg = get_argument(&runner);

    let command = format_string(cmd, &args);

    let _ = Command::new(runner.clone())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .args([arg, &command])
        .status();
}

pub fn preset(command: String, args: Arguments) {
    match get(true) {
        Some(config) => {
            match config.presets.get(&command) {
                Some(preset) => {
                    let runner = preset.runner.clone()
                        .unwrap_or(config.runner);

                    let arg = get_argument(&runner);

                    if arg.is_empty() {
                        // fuck \n
                        return println!("It looks like {} doesn't supports the {} runner.
You can create an issue on our GitHub repository, and we add it to next version of uki!
uWu {}", "uki".accent(), runner.accent(), "^-^".accent())
                    }

                    let mut prepared_args: PreparedArguments = HashMap::new();

                    // мда. треш..
                    match &preset.arguments {
                        Some(preset_args) => {
                            let mut i = 0;
                            preset_args.iter().for_each(|arg| {
                                match args.get(i) {
                                    Some(yo) => prepared_args.insert(arg.to_owned(), yo.to_owned()),
                                    None => return,
                                };

                                i += 1;
                            });
                        },
                        None => {}
                    }

                    preset.execute.clone()
                        .into_iter()
                        .for_each(|command| handle_command(runner.clone(), command, prepared_args.clone()));
                },
                None => return println!("Preset {} doesn't exists!", command.accent())
            }

        },
        None => return
    }
}