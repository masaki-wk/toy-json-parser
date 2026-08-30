#! /bin/sh
#
# Usage: $0 TITLE-PREFIX TAG-PREFIX PULL-REQUEST-TITLE
#
# Checker script for release pull request title.
#
# This script checks whether the following expected conditions are satisfied:
#
# - Pull request title matches TITLE-PREFIX + version-string
# - version-string in the pull request title matches the one in Cargo.toml
# - The release tag TAG-PREFIX + version-string does not exist

if [ "$#" -ne 3 ]; then
  echo "Usage: $0 TITLE-PREFIX TAG-PREFIX PULL-REQUEST-TITLE" >&2
  exit 1
fi
title_prefix="$1"
tag_prefix="$2"
title="$3"
failed=

check_target='Pull request title matches the expected pattern'
version="${title#$title_prefix}"
if [ "$title" = "${title_prefix}${version}" ]; then
  echo "${check_target}: PASS"
else
  echo "${check_target}: FAIL"
  failed=yes
fi

check_target='The version string in the pull request title matches the one in Cargo.toml'
version_from_cargo_toml=$(
  sed -n 's/^[[:space:]]*version[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' Cargo.toml
)
if [ "$version" = "$version_from_cargo_toml" ]; then
  echo "${check_target}: PASS"
else
  echo "${check_target}: FAIL"
  failed=yes
fi

check_target='Check whether the release tag already exists'
if git tag --list | grep -Fx "${tag_prefix}${version}" >/dev/null; then
  echo "${check_target}: FAIL"
  failed=yes
else
  echo "${check_target}: PASS"
fi

if [ -z "$failed" ]; then
  echo "All checks PASSED"
else
  echo "Checks FAILED"
  exit 1
fi
