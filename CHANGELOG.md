# Changelog

## [1.0.0](https://github.com/netlify/elf-cam/compare/v0.1.1...v1.0.0) (2022-06-23)


### ⚠ BREAKING CHANGES

* The result is now non-nullable and it is a struct with the fields `arch`, `platform`, `runtime`. Runtime is nullable as we can only detect it for ELF binaries for now. If `platform` or `arch` cannot be detected it will throw.

### Features

* Return also architecture and platform ([#8](https://github.com/netlify/elf-cam/issues/8)) ([6aa4a95](https://github.com/netlify/elf-cam/commit/6aa4a956a8916b63846040179e28588d9a362a7f))
