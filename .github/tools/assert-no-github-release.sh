#! /bin/sh
#
# Usage: $0 OWNER REPO TAG
#
# Verifies that no GitHub Release exists for the specified tag in the target repository

# Parse command-line arguments
if [ "$#" -ne 3 ]; then
  echo "Usage: $0 OWNER REPO TAG" >&2
  exit 1
fi
owner="$1"
repo="$2"
tag="$3"

# Query the target repository
gh api "repos/${owner}/${repo}" >/dev/null 2>&1 || {
  echo "Error: failed to query repository ${owner}/${repo}" >&2
  exit 2
}

# Query the release associated with the tag
encoded_tag=$(printf '%s' "$tag" | jq -sRr @uri)
response=$(gh api "repos/${owner}/${repo}/releases/tags/${encoded_tag}" --include 2>&1)
http_status=$(printf '%s\n' "$response" | sed -n '1s/.* \([0-9][0-9][0-9]\).*/\1/p')

# Report the result
case "$http_status" in
  200)
    echo "Error: a release for tag \`${tag}\` already exists" >&2
    exit 1
    ;;
  404)
    echo "No release associated with tag \`${tag}\` was found"
    exit 0
    ;;
  *)
    echo "Error: the request for the release associated with tag \`${tag}\` returned HTTP status ${http_status}" >&2
    exit 2
    ;;
esac
