#! /bin/sh

# - Convert `` [`FOO`] `` to `` `FOO` ``
#
sed 's,\[\(`[^`]*`\)\],\1,g'
