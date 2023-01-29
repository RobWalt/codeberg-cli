[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[<img alt="crates.io" src="https://img.shields.io/crates/v/codeberg-cli.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/codeberg-cli)

<p align="center">
  <img alt="codeberg-cli logo" src="https://codeberg.org/RobWalt/codeberg-cli/raw/branch/main/logo.png" width="500" height="500">
</p>

# cod-rs

CLI Tool for [Codeberg](https://codeberg.org/) similar to `gh` and `glab`.

<p align="center">
  <img alt="Shell running the issue view command" width="600" src="https://codeberg.org/RobWalt/codeberg-cli/raw/branch/main/dogfood.gif">
</p>

# Installation 


## I. Cargo

The easiest way to install the `cod` command is using cargo. (If you haven't installed rust/cargo yet, take a look [here](https://doc.rust-lang.org/cargo/getting-started/installation.html))

### I.I. Cargo Build 

Clone the repo and install `cod` with 

```sh 
cargo install --path .
```

### I.II. Cargo Crates.io (Not working at the moment)

I can't publish the crate at the moment since all dependencies of this crate must be available on crates.io. However, some of my dependencies are forks that include fixes / new features and this prevents me from triggering the release workflow.

---

Anywhere, just run 

```sh
cargo install codeberg-cli
```

## II. Checking the installation

After that, the `cod` command should be available for you. Check it with `cod -V`

```sh
cod 0.1.0
```

# Usage

Run `cod --help` for a detailed help menu. Each subcommand also has it's own help menu (which might need better documentation)

```sh 
Usage: cod [COMMAND]

Commands:
  auth   Choose authentification subcommands
  user   User information
  issue  Issue commands
  pull   Pull request commands
  label  Label commands
  repo   Repo commands
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

# Development 

- [Codeberg Gitea API Docs](https://codeberg.org/api/swagger) used by [forgejo](https://codeberg.org/forgejo/forgejo/src/branch/forgejo/docs/content/doc/developers/api-usage.en-us.md#api-guide) which is in turn used by codeberg.
