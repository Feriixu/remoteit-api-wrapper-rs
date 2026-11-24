# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Changed

- Update direct dependencies and refresh the tracked lockfiles, including `dirs` 6 to 7 for the optional credentials loader. The `home_dir` call used by this crate is unaffected by the `dirs` 7 Windows `preference_dir` behavior change.
