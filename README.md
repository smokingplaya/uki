![shi](misc/logo.png)
# uki

**uki** is a utility for running predefined commands in various shells on your system through a simple YAML configuration. It allows you to customize commands for frequent use and run them through a single call.

## Installing
* Linux
  - Debian-based:
  ```bash
  # updating dependencies
  sudo apt update && sudo apt upgrade
  wget https://github.com/smokingplaya/uki/releases/latest/download/uki-linux_x86 -O uki
  sudo mv uki /usr/local/bin
  ```
* Windows (powershell)\
  *Not tested on windows yet, do it on your own risk*
  ```powershell
  & {Invoke-WebRequest -Uri "https://raw.githubusercontent.com/smokingplaya/uki/refs/heads/master/misc/windows.ps1" -OutFile "$env:USERPROFILE\Downloads\install_uki.ps1"; & "$env:USERPROFILE\Downloads\install_uki.ps1"}
  ```

## Using

```bash
uki <preset> <аргументы>
```

- **preset** is the name of the preset (a set of pre-prepared commands) to be run. If you do not specify a preset, the `default` preset will be used by default.
- **arguments** are the parameters that are passed to the preset commands.

### Examples:

1. Run the `default` preset (no arguments):
    ```bash
    uki
    ```
2. Run the `example` preset with the argument “hello, there”:
    ```bash
    uki example "hello, there"
    ```
3. Run the `default` preset with arguments passed:
    ```bash
    uki default arg1 arg2
    ```

## Configuration

For **uki** to work, there must be a YAML configuration file called `.uki` in the current directory.

### Example of `.uki` file structure:

```yaml
default-enviroment: bash # Default shell for all presets (optional)
default-preset: default # Default preset if not specified (optional)

# Presets section (required)
presets:
  # Preset name
  default:
    enviroment: bash        # Shell override for a specific preset (optional)
    description: "Пресет, который ничего не делает"  # Preset description (optional)
    arguments:
      - name: arg1          # Name of the argument
        default: ""         # Default value for the argument (optional)
    commands:               # Command list (required)
      - echo "${arg1}"      # Example of a command using the argument
```

### Supported shells:

- `bash`
- `zsh`
- `powershell`
- `cmd`

### Description of configuration parameters:

- **default-enviroment** - specifies the default shell for all presets. If not specified, the system-installed shell will be used for the preset.
  
- **default-preset** - specifies the preset that will be run if the first argument (preset name) was not passed when `uki` was called.

- **presets** - mandatory section where all presets are described. Each preset can contain the following parameters:
  - **enviroment** - the shell in which the commands of this preset will be executed.
  - **description** - description of the preset to make it clear what it does.
  - **arguments** - list of arguments that can be passed to the preset.
  - **commands** - list of commands that will be executed when the preset is started. Arguments passed through the CLI can be used in commands using the `${arg}` syntax.

### Example of preset usage:

```yaml
presets:
  example:
    enviroment: zsh
    description: "Example of preset"
    arguments:
      - name: message
        default: "Hello, world!"
    commands:
      - echo "${message}"
```

If you execute the command:

```bash
uki example “What's up”
```

Then **uki** will start the `example` preset by passing the ``“What's up”`` argument to the command, and execute the ``echo “What's up”`` command in the `zsh` shell.

## Building

To build **uki**, follow these steps:

```bash
git clone https://github.com/smokingplaya/uki
cargo build --release
cd target/release
```

## Support and Contributions
We are always open to improvements! If you have ideas, comments, or fixes, feel free to open a pull request.