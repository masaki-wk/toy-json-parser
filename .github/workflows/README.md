# Workflows

This directory contains GitHub Actions workflows for continuous integration and package release management.
The release flow is described in [release-flow.md](./release-flow.md).

## Workflows for continuous integration

### CI

[CI (ci.yml)](./ci.yml) verifies the following:

- Verify formatting
- Lint
- Verify documentation (all doc comments and the generated `README.md`)
- Tests on the following Rust toolchains:
  - Stable
  - Nightly
  - MSRV: The minimum supported Rust version

## Workflows for release

### Create Release Pull Request

[Create Release Pull Request (create-release-pull-request.yml)](./create-release-pull-request.yml) is run manually with the package version as input.

The workflow performs the following actions:

- Checks the following:
  - Verifies that the version follows Semantic Versioning
  - Checks that the version differs from the current package version
  - Checks that the corresponding release branch, tag, and GitHub Release do not already exist
- Creates a release branch and a release pull request
  - Updates the package version in `Cargo.toml`
  - Creates a release branch `release/<version>` and commits the updated `Cargo.toml` to it
  - Creates a release pull request with the title `Release <version>`

### Check Pull Request

[Check Pull Request (check-pull-request.yml)](./check-pull-request.yml) runs for pull requests targeting the `main` branch.
Release-specific checks are performed when the source branch name starts with `release/`.

The workflow checks the following:

- For release pull requests (`release/`)
  - The pull request title is `Release <version>`
  - The release branch name is `release/<version>`
  - The corresponding release tag and GitHub Release do not already exist
  - The package can be published
- For normal pull requests (not `release/`)
  - The pull request title does not start with "Release "

### Release

[Release (release.yml)](./release.yml) runs when a release pull request is merged into `main`.

The workflow performs the following actions:

- Creates a release tag `v<version>`
- Creates a draft GitHub Release `v<version>`
