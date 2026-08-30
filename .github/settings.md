# Settings on GitHub

## General

- Default branch: `main`

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
    - [ ] Require status checks to pass
    - [x] Block force pushes
    - [ ] Require code scanning results
    - [ ] Require code quality results
    - [ ] Restrict code coverage
    - [ ] Automatically request Copilot code review
