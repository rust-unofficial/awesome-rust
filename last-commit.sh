#!/bin/sh

sed -i 's#\s*<img src="https://img.shields.io/github/last-commit/.*/master.svg">\s*##' README.md
sed -i 's#^\s*\*.*https://github.com/\([^)]*\).*$#\0 <img src="https://img.shields.io/github/last-commit/\1/master.svg">#' README.md
