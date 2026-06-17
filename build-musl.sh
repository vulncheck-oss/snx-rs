#!/bin/bash
set -e 

BUILDDIR="build"

mkdir $BUILDDIR -p
docker build --no-cache -t muslbuild .
docker run --rm -v ./build:/output muslbuild
echo "if successful, built bin should be in ./$BUILDDIR/target/"
