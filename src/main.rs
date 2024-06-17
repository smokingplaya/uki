use std::{collections::HashMap, env};

mod config;
mod commands;

pub const UKI_FOLDER: &'static str = ".uki";
const DEFAULT_PRESET: &'static str = "default";

/*
    Commands
*/

type Arguments = Vec<String>;
type PreparedArguments = HashMap<String, String>;
type CommandFunc = fn(Arguments);
pub type CommandMap = HashMap<&'static str, CommandFunc>;

/*
    Main function / Entry point
*/

fn main() {
    let mut commands: CommandMap = HashMap::new();
    commands::register(&mut commands); // haha wtf hows it work

    let mut args = env::args().collect::<Arguments>();
    args.remove(0); // removin this

    let default_preset = DEFAULT_PRESET.to_string();
    let command = args.get(0).unwrap_or(&default_preset).clone();

    // yeah babe
    if args.len() > 0 {
        args.remove(0);
    }

    let action = commands.get(command.clone().as_str());

    match action {
        Some(callback) => callback(args),
        None => config::preset(command.to_owned(), args),
    }
}