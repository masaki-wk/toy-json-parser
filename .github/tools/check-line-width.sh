#! /bin/sh

# Help function
show_help () {
  cat <<__END__
Usage: $0 WIDTH [FILE...]

Check whether files contain lines whose length reaches the specified width.
__END__
}

# Parse command-line arguments
if [ $# -lt 1 ]; then
  show_help
  exit 1
fi
width="$1"
shift

# Check the input
awk -e "length(\$0) >= $width && !/^\[!\[/ { print \"Error: Line \" NR \" contains \" length(\$0) \" chars\"; found=1 } END { exit found }" $*
