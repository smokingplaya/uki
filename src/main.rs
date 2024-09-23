/**
 * uki
 * created by smokingplaya 2024
 *
 * changes:
 * rewrote the code so it's clearer.
 * now we use serde_yml
 * now the yml-config file doesn't need a separate folder, it just uses the .uki file in cwd
 * changed the structure of the yml-config file, now it looks more like the smokingplaya/runny file structure
 * added command history; now you can work with cd/pwd etc.
 */

pub(crate) mod configuration;
pub(crate) mod argument;
pub(crate) mod preset;

/**
 * execute
 * * Executes configuration file
 */
pub(crate) fn execute(
  arguments: &Vec<String>
) -> anyhow::Result<()> {
  let mut cli_arguments = arguments.clone();

  if cli_arguments.len() > 0 {
    cli_arguments.remove(0);
  }

  configuration::new()?
    .run_preset(
      cli_arguments,
      arguments.get(0).cloned()
    )
}

fn main() -> anyhow::Result<()> {
  let mut arguments = std::env::args()
    .collect::<Vec<String>>();

  // removing path to program
  arguments.remove(0);

  execute(&arguments)
}