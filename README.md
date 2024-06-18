<p align="center">
  <img src=".uki/logo.png">
</p>

<p align="center">
<img src="https://img.shields.io/badge/Powered By Rust-e43717?style=for-the-badge&logo=rust&logoColor=white">
</p>

# **uki**
**uki** is a utility for automating the execution of pre-configured commands and scripts, set up through YAML configurations. This tool is useful for unifying and simplifying the execution of frequently used commands across various workspaces.

# Features
* Simple configuration through YAML files.
* Support for different interpreters: cmd, bash, zsh, powershell.
* Flexibility in command configuration with the use of arguments and presets.
* Universal usage across different workspaces.

# Installation
To install uki, use [winget](https://github.com/microsoft/winget-cli):

```sh
winget install uki
```

# Configuration
The configuration file for each workspace is located at **{workspace}/.uki/config.yml**. Example structure of the file:

```yaml
name: example # Configuration name
description: lorem ipsum... # (optional) Configuration description
authors: ["author1", "author2"]
runner: powershell # Possible values: cmd, bash, zsh, powershell

presets:
   default:
      execute:
      - ls
      - echo "uki is the best!"
   test:
      arguments: ["test", "my_name"]
      execute:
      - echo "$test"
      - echo "My name: $my_name"
```

# Configuration Fields
* **name**: Configuration name.
* **description**: (optional) Configuration description.
* **authors**: List of configuration authors.
* **runner**: Command interpreter. Possible values: cmd, bash, zsh, powershell.
* **presets**: List of command presets.

## Presets
Presets allow you to define a set of commands for execution. Each preset can contain:

* **arguments**: (optional) List of arguments used in commands.
* **execute**: List of commands for execution.
# Usage

Run uki from the command line to execute commands:

```sh
uki
```
By default, the **default** preset will be executed.

To run other presets, specify their name and arguments:

```sh
uki test "Hello, world" George
```
In this example:

* **"Hello, world"** is the argument for test.
* **George** is the argument for my_name.

## Default command
uki also has default commands:
* **make**: creates necessary files/folders for uki in current workspace
* **list**: prints config details with preset list

```sh
uki make
uki list
```

# Contributing
We welcome contributors! To contribute, create a pull request in the repository. Please ensure that your code adheres to good coding standards and is tested.

# License
uki is distributed under the GNU GPL v3.0 license. \
Details can be found in the [LICENSE](LICENSE) file.