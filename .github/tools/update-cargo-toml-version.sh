#! /bin/sh
#
# Usage: $0 VERSION < Cargo.toml > Cargo.toml.new
#
# Update package version in Cargo.toml.

if [ "$#" -ne 1 ]; then
  echo "Usage: $0 VERSION" >&2
  exit 1
fi
version="$1"

sed '
  /^\[package\]/,/^\[/ {
    s/^\([[:space:]]*version[[:space:]]*=[[:space:]]*"\)[^"]*\(".*\)$/\1'"$version"'\2/
  }
'
