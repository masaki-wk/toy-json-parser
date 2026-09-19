# Settings on GitHub - Environment `production`

- Deployment protection rules
  - [ ] Required reviewers
  - [ ] Wait timer
  - [ ] Enable custom rules with GitHub Apps
- [x] Allow administrators to bypass configured protection rules
- Deployment branches and tags: Selected branches and tags
  - Branch: 'refs/pull/*/merge'
- Environment secrets: (empty)
- Environment variables: (empty)

## Notes on the deployment branches rule

The branch rule for `refs/pull/*/merge` allows workflows triggered by `pull_request` events to deploy to the environment.
See: [Deployment branches and tags](https://docs.github.com/en/actions/reference/workflows-and-actions/deployments-and-environments#deployment-branches-and-tags)
