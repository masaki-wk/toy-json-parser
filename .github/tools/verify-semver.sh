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

numeric_id='\(0\|[1-9][0-9]*\)'
prerelease='-[-.0-9A-Za-z][-.0-9A-Za-z]*'
buildmetadata='+[-.0-9A-Za-z][-.0-9A-Za-z]*'

echo "$version" | grep -x "${numeric_id}\.${numeric_id}\.${numeric_id}\(\|${prerelease}\)\(\|${buildmetadata}\)" >/dev/null || {
  echo "Error: version string \`$version\` is invalid" >&2
  exit 1
}
