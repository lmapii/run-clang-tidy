#!/bin/sh
# input https://github.com/BurntSushi/ripgrep/blob/master/ci/ubuntu-install-packages
# input https://docs.github.com/en/actions/using-github-hosted-runners/customizing-github-hosted-runners

if ! command -V sudo; then
  apt-get update
  apt-get install -y --no-install-recommends sudo
fi

sudo apt-get update
sudo apt-get install -y --no-install-recommends \
  wget

ver="14.0.0"
pkg="clang+llvm-$ver-x86_64-linux-gnu-ubuntu-18.04"

wget -O clang-$ver.tgz "https://github.com/llvm/llvm-project/releases/download/llvmorg-$ver/$pkg.tar.xz"
mkdir -p artifacts/clang
tar -xf clang-$ver.tgz $pkg/bin/clang-tidy
mv $pkg/bin/clang-tidy artifacts/clang
rm -rf $pkg
rm clang-$ver.tgz

ls -la artifacts/clang
artifacts/clang/clang-tidy --version
