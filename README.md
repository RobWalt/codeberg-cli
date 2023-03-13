[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[<img alt="crates.io" src="https://img.shields.io/crates/v/codeberg-cli.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/codeberg-cli)

<p align="center">
  <img alt="codeberg-cli logo" src="https://codeberg.org/RobWalt/codeberg-cli/raw/branch/main/assets/logo.png" width="500">
</p>

# codeberg-cli (berg)

CLI Tool for [Codeberg](https://codeberg.org/) similar to `gh` and `glab`.

<p align="center">
  <img alt="Shell running the issue view command" width="600" src="https://codeberg.org/RobWalt/codeberg-cli/raw/branch/main/assets/userinfo.gif">
  <img alt="Shell running the issue view command" height="800" src="https://codeberg.org/RobWalt/codeberg-cli/raw/branch/main/assets/issueview.gif">
</p>

# Installation 


## I. Cargo

The easiest way to install the `berg` command is using cargo. (If you haven't installed rust/cargo yet, take a look [here](https://doc.rust-lang.org/cargo/getting-started/installation.html))

### I.I. Cargo Build (from source)

Clone the repo and install `berg` with 

```sh 
cargo install --path .
```

### I.II. Cargo Crates.io

Anywhere, just run 

```sh
cargo install codeberg-cli
```

## II. Checking the installation

After that, the `berg` command should be available for you. Check it with `berg -V`

```sh
berg 1.0.0
```

# Usage

Run `berg -h` for the help menu. Each subcommand also has it's own help menu

```sh 
Codeberg CLI app

Usage: berg [COMMAND]

Commands:
  auth        Authentication subcommands
  user        User subcommands
  issue       Issue subcommands
  pull        Pull request subcommands
  label       Label subcommands
  repo        Repository subcommands
  milestone   Milestone subcommands
  completion  Print completion script
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help (see more with '--help')
  -V, --version  Print version

```

# Development 

Please take a look at [CONTRIBUTING.md](https://codeberg.org/RobWalt/codeberg-cli/raw/branch/main/CONTRIBUTING.md) before opening PRs. We can't accept PRs which don't stick to the guidelines.
