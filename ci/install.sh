#!/bin/sh
set -ex

apt-get -qq update

case "$1" in
    i686-pc-windows-gnu)
        apt-get install mingw-w64

        # setup compiler
        cat >> ~/.cargo/config <<__END__
[target.i686-pc-windows-gnu]
linker = "i686-w64-mingw32-gcc"
ar = "i686-w64-mingw32-gcc-ar"
rustflags = "-C panic=abort"
__END__
        ;;
    i686-unknown-linux-gnu)
        apt-get install -qy gcc-multilib
        ;;
    *)
        # nop
        ;;
esac
