/**
 * uki
 * created by smokingplaya 2024
 *
 * uki 0.2
 * changes:
 *  rewrited code and becomes clearly
 *  now we using serde_yml
 *  now yml-config file doesn't requires separate folder, it just uses .uki file in cwd
 *  yml-config file changed structure, now its more like to smokingplaya/runny file structure
 *  added command history; now you can works with cd/pwd etc.
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