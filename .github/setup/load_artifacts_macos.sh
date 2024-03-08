#!/bin/sh

brew install wget make python3

pip3 install \
  argparse \
  datetime

ver="14.0.0"
pkg="clang+llvm-$ver-x86_64-apple-darwin"

wget -O clang-$ver.tgz "https://github.com/llvm/llvm-project/releases/download/llvmorg-$ver/$pkg.tar.xz"
mkdir -p artifacts/clang

# extract all binaries
tar -xf clang-$ver.tgz $pkg/bin

# move the binaries to the artifacts folder
mv $pkg/bin/* artifacts/clang

rm -rf $pkg
rm clang-$ver.tgz

ls -la artifacts/clang
artifacts/clang/clang-tidy --version

# generate compile-commands.json
gmake -C test-files/c-demo/project build-data
ls -la test-files/c-demo/_bld/out

# build project (clang-tidy can fail due to compiler errors)
gmake -C test-files/c-demo/project