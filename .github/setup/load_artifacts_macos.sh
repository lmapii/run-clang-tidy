#!/bin/sh

brew install gcc wget make python3

pip3 install \
  argparse \
  datetime

ver="14.0.0"
pkg="clang+llvm-$ver-x86_64-apple-darwin"

wget -O clang-$ver.tgz "https://github.com/llvm/llvm-project/releases/download/llvmorg-$ver/$pkg.tar.xz"
mkdir -p artifacts/clang
tar -xf clang-$ver.tgz $pkg/bin/clang-tidy
mv $pkg/bin/clang-tidy artifacts/clang
rm -rf $pkg
rm clang-$ver.tgz

ls -la artifacts/clang
artifacts/clang/clang-tidy --version

gmake -C test-files/c-demo/project build-data
ls -la test-files/c-demo/_bld/out