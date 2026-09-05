# Release flow

The release flow consists of the following steps:

1. Manually run `Create Release Pull Request`.
   The workflow executes the following:
   1. Creates a release branch and updates the package version in `Cargo.toml` on the branch
      - Branch name: `release/<version>`
      - Base branch: `main`
   2. Creates a release pull request
      - Target branch: `main`
      - Title: `Release <version>`
2. Review the generated release pull request and then merge it.
3. The workflow automatically executes the following:
   1. Creates a release tag
      - Target commit: the merge commit
      - Tag name: `v<version>`
      - Tag type: lightweight
   2. Creates a draft GitHub Release
      - Title: `v<version>`
      - Tag: `v<version>`
      - Release notes: automatically generated
      - Status: draft
4. Review the draft GitHub Release, modify it, and convert it from a draft to a published release
5. Manually publish the crate with `cargo publish` on the local environment
