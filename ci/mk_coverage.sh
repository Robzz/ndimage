#!/bin/bash

set -ev

for file in target/debug/ndimage-*[^\.d]; do
    mkdir -p "target/cov/$(basename $file)"
    ../kcov-build/usr/local/bin/kcov --exclude-pattern=/.cargo,/usr/lib --verify "target/cov/$(basename $file)" "$file"
done
bash <(curl -s https://codecov.io/bash)
echo "Uploaded code coverage"
rm -f target/debug/ndimage-*
