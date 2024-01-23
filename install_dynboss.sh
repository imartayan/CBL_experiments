#!/bin/bash
set -euxo pipefail

git submodule update --init --recursive
cd dynboss
sed -i -e 's/python/python3/g' src/Makefile
sed -i -e 's/print /print(/g' src/make_lut.py
sed -i -e 's/))))/)))))/g' src/make_lut.py
cd dsk-1.6906
make dsk canon=0
# make dsk k=64 canon=0
cd ..
cd src
make revcomps=0
