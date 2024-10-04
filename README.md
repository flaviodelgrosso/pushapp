# 🚀 Pushapp CLI 🏋🏻‍♂️

⚡ Ultra fast dependencies updater for Node.js written in Rust! ⚡

[![Latest Version]][crates.io] ![Crates.io Downloads](https://img.shields.io/crates/d/pushapp-cli) ![GitHub Repo stars](https://img.shields.io/github/stars/flaviodelgrosso/pushapp?style=flat)

[Latest Version]: https://img.shields.io/crates/v/pushapp-cli.svg
[crates.io]: https://crates.io/crates/pushapp-cli

![image](pushapp-cli.gif)

## Features

- [x] Ultra fast
- [x] Support for npm, pnpm, yarn and bun
- [x] Interactive mode
- [x] Autocomplete
- [x] Colored updatable packages based on semver diff
- [x] CLI utility flags

## Roadmap

- [ ] Monorepo support ⚠️
- [ ] Check global packages ⚠️
- [ ] Single packages update with filters ⚠️
- [ ] Non-interactive mode with different display formatting and infos (publish time, semver grouping ) ⚠️

## Installation

```bash
cargo install pushapp-cli
```

## Usage

Run the following command in your Node.js project directory:

```bash
pushapp
```

## Flag options

| Option                              | Description                          |
|-------------------------------------|--------------------------------------|
| `-d`, `--development`               | Check only `devDependencies`         |
| `-p`, `--production`                | Check only `dependencies`            |
| `-o`, `--optional`                  | Check only `optionalDependencies`    |
| `-h`, `--help`                      | Display help information             |
| `-V`, `--version`                   | Display version information          |

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Flavio Delgrosso
