# Changelog

## [1.0.0](https://github.com/netlify/binary-info/compare/v1.0.0...v1.0.0) (2022-06-23)


### ⚠ BREAKING CHANGES

* The result is now non-nullable and it is a struct with the fields `arch`, `platform`, `runtime`. Runtime is nullable as we can only detect it for ELF binaries for now. If `platform` or `arch` cannot be detected it will throw.

### Features

* Rename package to binary-info and fix release ([#15](https://github.com/netlify/binary-info/issues/15)) ([0d59e30](https://github.com/netlify/binary-info/commit/0d59e30e7088620060bc021dc6ba2e8a5650a2ae))
* Return also architecture and platform ([#8](https://github.com/netlify/binary-info/issues/8)) ([6aa4a95](https://github.com/netlify/binary-info/commit/6aa4a956a8916b63846040179e28588d9a362a7f))


### Miscellaneous Chores

* Fix release ([#17](https://github.com/netlify/binary-info/issues/17)) ([9e97df0](https://github.com/netlify/binary-info/commit/9e97df06d34ca1e5d9dff0b03d684392d4ae736f))
* fix release workflow ([#13](https://github.com/netlify/binary-info/issues/13)) ([0aade15](https://github.com/netlify/binary-info/commit/0aade15d09809e3433fc5e7520a682ac03243faf))
