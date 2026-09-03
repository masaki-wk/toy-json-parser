#! /bin/sh
#
# Usage: $0 [FILE...]
#
# Print package version in Cargo.toml, read from stdin or FILE.

sed -n -e '
  /^\[package\]/,/^\[/ {
    s/^[[:space:]]*version[[:space:]]*=[[:space:]]*"\([^"]*\)".*$/\1/p
  }
' $*
