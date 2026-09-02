#! /bin/sh
#
# Usage: $0 VERSION [FILE...]
#
# Update package version in Cargo.toml, which given from stdin or FILE.

if [ "$#" -lt 1 ]; then
  echo "Usage: $0 VERSION" >&2
  exit 1
fi
version="$1"
shift

sed -e '
  /^\[package\]/,/^\[/ {
    s/^\([[:space:]]*version[[:space:]]*=[[:space:]]*"\)[^"]*\(".*\)$/\1'"$version"'\2/
  }
' $*
