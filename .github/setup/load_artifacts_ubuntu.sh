#!/bin/sh
# input https://github.com/BurntSushi/ripgrep/blob/master/ci/ubuntu-install-packages
# input https://docs.github.com/en/actions/using-github-hosted-runners/customizing-github-hosted-runners

if ! command -V sudo; then
  apt-get update
  apt-get install -y --no-install-recommends sudo
fi

sudo apt-get update
sudo apt-get install -y --no-install-recommends \
  wget \
  make \
  python3 \
  python3-pip \
  python3-setuptools \
  python3-wheel \

sudo pip3 install \
  argparse \
  datetime

sudo apt-get remove -y \
  llvm \
  clang \
  clang-tidy

# echo "path is $PATH"
# echo "checking clang-tidy"
# clang-tidy --version
# which clang-tidy
# echo ".done"
sudo rm -f /usr/bin/clang-tidy

ver="14.0.0"
pkg="clang+llvm-$ver-x86_64-linux-gnu-ubuntu-18.04"

wget -O clang-$ver.tgz "https://github.com/llvm/llvm-project/releases/download/llvmorg-$ver/$pkg.tar.xz"
mkdir -p artifacts/clang

# extract all binaries
tar -xf clang-$ver.tgz $pkg/bin

# move the binaries to the artifacts folder
mv $pkg/bin/clang-tidy/* artifacts/clang

rm -rf $pkg
rm clang-$ver.tgz

ls -la artifacts/clang
artifacts/clang/clang-tidy --version

make -C test-files/c-demo/project build-data
ls -la test-files/c-demo/_bld/out