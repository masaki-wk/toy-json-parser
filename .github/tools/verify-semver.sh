#! /bin/sh
#
# Usage: $0 VERSION
#
# Verify whether the version string is valid with Semantic Versioning - MAJOR.MINOR.PATCH[-prerelease][+buildmetadata]

if [ "$#" -ne 1 ]; then
  echo "Usage: $0 VERSION" >&2
  exit 1
fi
version="$1"

# The official SemVer RegEx copied from https://semver.org/#is-there-a-suggested-regular-expression-regex-to-check-a-semver-string
semver_regex='^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$'

echo "$version" | grep -P "$semver_regex" >/dev/null || {
  echo "Error: version string \`$version\` is invalid" >&2
  exit 1
}
