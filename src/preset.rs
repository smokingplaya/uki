use std::{collections::HashMap, process::Command};
use serde::{Deserialize, Serialize};
use crate::argument::Argument;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Preset {
  enviroment: Option<String>,
  description: Option<String>,
  arguments: Option<Vec<Argument>>,
  commands: Vec<String>
}

impl Preset {
  /**
   * execute
   * * Synchronized preset execution
   * @param enviroment Execution shell
   */
  pub fn get_run_argument(
    &self,
    enviroment: &String
  ) -> anyhow::Result<&str> {
    match enviroment.to_lowercase().as_str() {
      "powershell" => Ok("-Command"),
      "bash" => Ok("-c"),
      "zsh" => Ok("-c"),
      "cmd" => Ok("/C"),
      _ => Err(anyhow::anyhow!("Your shell is unsupported."))
    }
  }

  /**
   * collect_arguments
   * * Collects all arguments regarding CLI arguments and preset arguments.
   */
  fn collect_arguments(
    &self,
    cli_arguments: Vec<String>
  ) -> HashMap<String, String> {
    let preset_arguments = self.arguments.as_ref().unwrap();
    let mut result = HashMap::new();

    preset_arguments.iter().enumerate().for_each(|(i, arg)| {
      let value = cli_arguments.get(i)
        .cloned()
        .unwrap_or_else(|| arg.default.clone().unwrap_or_default());

      result.insert(arg.name.clone(), value);
    });

    result
  }

  /**
   * prepare_commands
   * * Returns commands that are ready to be executed
   */
  fn prepare_commands(&self, cli_arguments: Vec<String>) -> Vec<String> {
    let preset_arguments = self.arguments.as_deref().unwrap_or_default();

    if preset_arguments.is_empty() {
      return self.commands.clone();
    }

    let arguments = self.collect_arguments(cli_arguments);

    self.commands
      .iter()
      .cloned()
      .map(|mut command| {
        for (key, value) in &arguments {
          let placeholder = format!("${{{}}}", key);

          command = command.replace(&placeholder, value);
        }
        command
    }).collect()
  }


  /**
   * execute
   * * Executes and handles all commands in shell
   */
  pub fn execute(
    &self,
    name: &str,
    cli_arguments: Vec<String>,
    enviroment: &Option<String>
  ) -> anyhow::Result<()> {
    let env = self.enviroment.clone().or_else(|| enviroment.clone()).ok_or_else(|| {
      anyhow::anyhow!("No environment for preset {} was found!", name)
    })?;

    let prefix = self.get_run_argument(&env)?;

    Command::new(env)
      .arg(prefix)
      .arg(self.prepare_commands(cli_arguments).join(";"))
      .status()?;

    Ok(())
  }
}