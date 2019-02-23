#!/bin/sh
set -ex

doxygen() {
    version=Release_1_8_15
    wget https://codeload.github.com/doxygen/doxygen/tar.gz/$version -O doxygen-$version.src.tar.gz
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
