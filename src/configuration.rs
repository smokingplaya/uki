use crate::preset::Preset;
use std::{collections::HashMap, env, fs::File, io::BufReader};
use serde::{Deserialize, Serialize};

const CONFIGURATION_FILE: &'static str = ".uki";

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Configuration {
  #[serde(rename = "default-preset")]
  default_preset: Option<String>,
  #[serde(rename = "default-enviroment")]
  default_enviroment: Option<String>,
  presets: HashMap<String, Preset>
}

/**
 * new
 * * Try to find configuration file in current working directory (cwd), and returns it as Rust structure
 */
pub(crate) fn new() -> anyhow::Result<Configuration> {
  let path = env::current_dir().unwrap().as_path().join(CONFIGURATION_FILE);

  let file = File::open(path)?;
  let reader = BufReader::new(file);

  Ok(serde_yml::from_reader(reader)?)
}

impl Configuration {
  /**
   * get_default_preset
   * * Returns default preset name for execution
   */
  pub fn get_default_preset(&self) -> String {
    self.default_preset
      .clone()
      .unwrap_or("default".to_string())
  }

  /**
   * run_preset
   * * Starts execution of the preset
   * @param preset Name of preset
   */
  pub fn run_preset(
    &self,
    cli_arguments: Vec<String>,
    preset: Option<String>
  ) -> anyhow::Result<()> {
    let preset_name = preset
      .unwrap_or(self.get_default_preset());

    self.presets
      .get(&preset_name)
      .ok_or_else(|| anyhow::anyhow!("Preset not found"))?
      .execute(
        &preset_name,
        cli_arguments,
        &self.default_enviroment
      )?;

    Ok(())
  }
}