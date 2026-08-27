#! /bin/sh

# - Remove links: e.g. [`FOO`] into `FOO`
# - Convert opening ``` of bare code blocks into ```text
#
sed -e 's/\[\(`[^`]*`\)\]/\1/g' -e '$!{N;s/^\n```$/\n```text/;P;D}'
