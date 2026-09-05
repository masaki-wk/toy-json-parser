# Workflows

This directory contains GitHub Actions workflows for continuous integration and releasing the package.
The release flow is described in [release-flow.md](./release-flow.md).

## Workflows for continuous integration

### CI

[CI](ci.yml) verifies the following:

- Rust formatting
- Linting
- Rust documentation and the generated `README.md`
- Builds and tests using the stable, nightly, and minimum supported Rust versions

## Workflows for release

### Create Release Pull Request

[Create Release Pull Request](./create-release-pull-request.yml) is run manually with the package version as an input.

The workflow:

- Checks the following conditions
  - Verifies that the version follows Semantic Versioning
  - Checks that the version is differ from the current package version
  - Checks that the corresponding release branch, tag, and GitHub Release do not already exist
- Creates a release branch and a release pull request
  - Updates the package version in `Cargo.toml`
  - Creates a release branch `release/<version>` and commits the updated `Cargo.toml` to it
  - Creates a release pull request with the title `Release <version>`

### Check Pull Request

[Check Pull Request](./check-pull-request.yml) runs for pull requests targeting the `main` branch.
Release-specific checks are performed when the source branch starts with `release/`.

The workflow checks that:

- For release pull requests (`release/`)
  - The pull request title is `Release <version>`
  - The release branch name is `release/<version>`
  - The corresponding release tag and GitHub Release do not already exist
  - The package can be published
- For normal pull requests (not `release/`)
  - The pull request title does not start with "Release "

### Release

[Release](./release.yml) runs when a release pull request is merged into `main`.

The workflow:

- Creates a release tag `v<version>`
- Creates a draft GitHub Release `v<version>`
