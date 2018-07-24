#!/bin/bash

set -ev

# Build and test
cargo build --verbose
cargo test --verbose

# Build kcov
cd ..
wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz
tar xzf master.tar.gz
cd kcov-master
mkdir build
cd build
cmake .. -DCMAKE_C_COMPILER_LAUNCHER="ccache" -DCMAKE_CXX_COMPILER_LAUNCHER="ccache"
make
make install DESTDIR=../../kcov-build
cd ../../ndimage
