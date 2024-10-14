# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.4.1] - 2024-10-14

### <!-- 2 -->🚜 Refactor
- Version target options and improve version matching by @flaviodelgrosso in [#6](https://github.com/flaviodelgrosso/pushapp/pull/6)

### <!-- 7 -->⚙️ Miscellaneous Tasks
- Update readme by @flaviodelgrosso

## [1.4.0] - 2024-10-14

### <!-- 0 -->🚀 Features
- Add version target and refactor registry client by @flaviodelgrosso in [#5](https://github.com/flaviodelgrosso/pushapp/pull/5)

### <!-- 7 -->⚙️ Miscellaneous Tasks
- Release pushapp-cli version 1.4.0 by @flaviodelgrosso

## [1.3.2] - 2024-10-05

### <!-- 2 -->🚜 Refactor
- Removed --optional flag, -P flag will merge `dependencies` and `optionalDependencies` aswell. by @flaviodelgrosso in [#4](https://github.com/flaviodelgrosso/pushapp/pull/4)
- Update struct and variable names in package_json.rs by @flaviodelgrosso

### <!-- 6 -->🧪 Testing
- Add unit tests for merge function in hashmap.rs by @flaviodelgrosso

### <!-- 7 -->⚙️ Miscellaneous Tasks
- Release pushapp-cli version 1.3.2 by @flaviodelgrosso
- Update release action to call build and depends on it by @flaviodelgrosso
- Add GitHub Actions workflow for building and testing the Rust project by @flaviodelgrosso

## [1.3.1] - 2024-10-05

### <!-- 2 -->🚜 Refactor
- Improve package manager detection with lock files. add bun support by @flaviodelgrosso
- Split package manager `install_deps` fn in smaller functions by @flaviodelgrosso

### <!-- 7 -->⚙️ Miscellaneous Tasks
- Release pushapp-cli version 1.3.1 by @flaviodelgrosso
- Update launch.json for pushapp-cli debugging by @flaviodelgrosso

## [1.3.0] - 2024-10-04

### <!-- 0 -->🚀 Features
- Add a new flag for checking global packages by @flaviodelgrosso
- Add validator for package selection by @flaviodelgrosso

### <!-- 4 -->⚡ Performance
- Rollback to stream processing from `join_all` futures by @flaviodelgrosso
- Use `FuturesUnordered` in `fetch_updates` and `process_update_results` fns by @flaviodelgrosso
- Use atomic ref counter for client in UpdateChecker by @flaviodelgrosso

### <!-- 5 -->🎨 Styling
- Use table to show flag options in README.md by @flaviodelgrosso

### <!-- 6 -->🧪 Testing
- Add tests for fs_utils by @flaviodelgrosso
- Add test for spec fields in package_json.rs by @flaviodelgrosso

### <!-- 7 -->⚙️ Miscellaneous Tasks
- Release pushapp-cli version 1.3.0 by @flaviodelgrosso
- Add CODEOWNERS file for repository ownership by @flaviodelgrosso
- Update commit parsers in cliff.toml by @flaviodelgrosso
- Add some roadmap points in readme.md by @flaviodelgrosso

## [1.2.1] - 2024-10-03

### <!-- 2 -->🚜 Refactor
- Use `join_all` in `process_update_results` fn by @flaviodelgrosso
- Group modules in cli folder by @flaviodelgrosso
- Implement UpdateChecker struct and chunked functions for better testing and readability by @flaviodelgrosso

### <!-- 7 -->⚙️ Miscellaneous Tasks
- Release pushapp-cli version 1.2.1 by @flaviodelgrosso
- Add prompt module by @flaviodelgrosso

## [1.2.0] - 2024-10-03

### <!-- 0 -->🚀 Features
- Feat: add new flag options to check only prod or optional deps by @flaviodelgrosso

### <!-- 2 -->🚜 Refactor
- Rename get_update_info to get_package_info for clarity by @flaviodelgrosso
- Move update check log to check_updates function by @flaviodelgrosso
- Optimize check_updates code readability and performance by @flaviodelgrosso
- Improve version parsing and error handling in PackageInfo display by @flaviodelgrosso

### <!-- 3 -->📚 Documentation
- Readme update by @flaviodelgrosso
- Update README by @flaviodelgrosso

### <!-- 7 -->⚙️ Miscellaneous Tasks
- Update changelog and cliff config by @flaviodelgrosso
- Release pushapp-cli version 1.2.0 by @flaviodelgrosso
- Update CHANGELOG.md and add cliff.toml configuration by @flaviodelgrosso

## [1.1.1] - 2024-10-02

### <!-- 2 -->🚜 Refactor
- Detect package manager from package json field by @flaviodelgrosso

### <!-- 7 -->⚙️ Miscellaneous Tasks
- Release pushapp-cli version 1.1.1 by @flaviodelgrosso
- Readme update by @flaviodelgrosso

## [1.1.0] - 2024-10-02

### <!-- 0 -->🚀 Features
- Add --dev flag to update only dev deps by @flaviodelgrosso
- Add GitHub Actions workflow for release automation by @flaviodelgrosso

### <!-- 1 -->🐛 Bug Fixes
- Update badge labels in README for clarity by @flaviodelgrosso

### <!-- 7 -->⚙️ Miscellaneous Tasks
- Add CHANGELOG.md by @flaviodelgrosso
- Release pushapp-cli version 1.1.0 by @flaviodelgrosso
- Move renovate in .github folder by @flaviodelgrosso
- Update release workflow to include changelog and add pre-release-hook by @flaviodelgrosso

## [1.0.1] - 2024-10-01

### <!-- 0 -->🚀 Features
- Add clap by @flaviodelgrosso

### <!-- 7 -->⚙️ Miscellaneous Tasks
- Release pushapp-cli version 1.0.1 by @flaviodelgrosso

## [1.0.0] - 2024-10-01

[1.4.1]: https://github.com/flaviodelgrosso/pushapp/compare/v1.4.0..1.4.1
[1.4.0]: https://github.com/flaviodelgrosso/pushapp/compare/v1.3.2..v1.4.0
[1.3.2]: https://github.com/flaviodelgrosso/pushapp/compare/v1.3.1..v1.3.2
[1.3.1]: https://github.com/flaviodelgrosso/pushapp/compare/v1.3.0..v1.3.1
[1.3.0]: https://github.com/flaviodelgrosso/pushapp/compare/v1.2.1..v1.3.0
[1.2.1]: https://github.com/flaviodelgrosso/pushapp/compare/v1.2.0..v1.2.1
[1.2.0]: https://github.com/flaviodelgrosso/pushapp/compare/v1.1.1..v1.2.0
[1.1.1]: https://github.com/flaviodelgrosso/pushapp/compare/v1.1.0..v1.1.1
[1.1.0]: https://github.com/flaviodelgrosso/pushapp/compare/v1.0.1..v1.1.0
[1.0.1]: https://github.com/flaviodelgrosso/pushapp/compare/v1.0.0..v1.0.1

<!-- generated by git-cliff -->
