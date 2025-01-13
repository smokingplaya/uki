//! uki
//! created by smokingplaya 2024-2025

pub(crate) mod configuration;
pub(crate) mod argument;
pub(crate) mod preset;

/**
 * execute
 * * Executes configuration file
 */
pub(crate) fn execute(
  arguments: Vec<String>
) -> anyhow::Result<()> {
  let mut cli_arguments = arguments.clone();

  println!("{cli_arguments:?}");

  if !cli_arguments.is_empty() {
    cli_arguments.remove(0);
  }

  configuration::new()?
    .run_preset(
      cli_arguments,
      arguments.first().cloned()
    )
}

fn main() -> anyhow::Result<()> {
  let mut arguments = std::env::args()
    .collect::<Vec<String>>();

  // removing path to program
  arguments.remove(0);

  execute(arguments)
}
