#!/bin/sh
set -ex

doxygen() {
    version=1.8.14
    wget ftp://ftp.stack.nl/pub/users/dimitri/doxygen-$version.src.tar.gz -O doxygen-$version.src.tar.gz
    tar -zxvf doxygen-$version.src.tar.gz
    cd doxygen-$version
    mkdir build
    cd build
    cmake ..
    make
    sudo make install
}

pmdk() {
    version=1.4
    wget https://github.com/pmem/pmdk/archive/$version.tar.gz -O pmdk-$version.tar.gz
    tar -zxvf pmdk-$version.tar.gz
    cd pmdk-$version
    make
    sudo make install
}

main() {
    doxygen
    pmdk
}

main