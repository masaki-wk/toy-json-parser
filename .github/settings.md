# Settings on GitHub

## General

- Default branch: `main`
- Releases
  - [ ] Enable release immutability
- Features
  - [ ] Wikis
  - [x] Issues
  - [ ] Sponsorships
  - [x] Preserve this repository
  - [ ] Discussions
  - [x] Projects
  - [x] Pull requests
- Pull Requests
  - [x] Allow merge commits
  - [x] Allow squash merging
  - [x] Allow rebase merging
  - [ ] Always suggest updating pull request branches
  - [ ] Allow auto-merge
  - [ ] Automatically delete head branches
- Commits
  - [ ] Require contributors to sign off on web-based commits
  - [x] Allow comments on individual commits
- Archives
  - [ ] Include Git LFS objects in archives
- Pushes
  - [ ] Limit how many branches and tags can be updated in a single push
- Issues
  - [x] Auto-close issues with merged linked pull requests

## Rulesets

- Branch ruleset `main`
  - Target branches: Default (changed)
  - Bypass list: (empty)
  - Branch protections
    - [ ] Restrict creations
    - [ ] Restrict updates
    - [x] Restrict deletions
    - [ ] Require linear history
    - [ ] Require deployments to succeed
    - [ ] Require signed commits
    - [x] Require a pull request before merging (changed)
      - Required approvals: 0
      - [ ] Dismiss stale pull request approvals when new commits are pushed
      - [ ] Require review from specific teams
      - [ ] Require review from Code Owners
      - [ ] Require approval of the most recent reviewable push
      - [ ] Require conversation resolution before merging
      - [x] Require an additional approval for unattributed Copilot pull requests
      - Allowed merge methods: Merge, Squash, Rebase
    - [x] Require status checks to pass (changed)
      - [x] Require branches to be up to date before merging (changed)
      - [ ] Do not require status checks on creation
      - Status checks that are required (added)
        - (All CI jobs)
    - [x] Block force pushes
    - [ ] Require code scanning results
    - [ ] Require code quality results
    - [ ] Restrict code coverage
    - [ ] Automatically request Copilot code review
