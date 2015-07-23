#!/bin/sh

set -e

cargo -v doc

# Manual doc deployment
echo "<meta http-equiv=refresh content=0;url=mpi/index.html>" >> target/doc/index.html
ghp-import -n target/doc
git push -f origin gh-pages
