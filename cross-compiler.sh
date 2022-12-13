#!/bin/bash
set -e


### setup env ###
export TARGET=x86_64-elf
export GCC_VERSION=12.2.0
export BINUTILS_VERSION=2.39
export PREFIX="$HOME/cross-$TARGET"

export PATH="$PREFIX/bin:$PATH"


### required packages ###
sudo apt-get update
sudo apt-get upgrade
sudo apt-get install build-essential bison flex libgmp3-dev libmpc-dev libmpfr-dev texinfo


### Delete previous build ###
rm -rf $HOME/src/build-binutils
rm -rf $HOME/src/build-gcc
rm -rf $HOME/src/binutils-$BINUTILS_VERSION
rm -rf $HOME/src/gcc-$GCC_VERSION


### download source ###
mkdir -p $HOME/src/
cd $HOME/src/
wget -N https://ftp.gnu.org/gnu/binutils/binutils-$BINUTILS_VERSION.tar.gz
wget -N https://ftp.gnu.org/gnu/gcc/gcc-$GCC_VERSION/gcc-$GCC_VERSION.tar.gz
tar -xvf binutils-$BINUTILS_VERSION.tar.gz
tar -xvf gcc-$GCC_VERSION.tar.gz


### compile binutils ###
cd $HOME/src
mkdir build-binutils
cd build-binutils
../binutils-$BINUTILS_VERSION/configure --target=$TARGET --prefix="$PREFIX" --with-sysroot --disable-nls --disable-werror
make
make install


### compile gcc ###
cd $HOME/src
which -- $TARGET-as || echo $TARGET-as is not in the PATH
mkdir build-gcc
cd build-gcc
../gcc-$GCC_VERSION/configure --target=$TARGET --prefix="$PREFIX" --disable-nls --enable-languages=c,c++ --without-headers
make all-gcc
make all-target-libgcc
make install-gcc
make install-target-libgcc
