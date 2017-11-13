#!/bin/sh
set -ex

case "$TRAVIS_OS_NAME-$TARGET" in
    linux-x86_64-apple-darwin)
        # cross compiling for osx is worse than windows, don't actually link a binary
        export RUSTFLAGS="-Clinker=true -Car=true"
        export PATH="$PATH:$PWD/ci/dummy"
        cargo build --release --verbose --target="$TARGET"
        ;;
    osx-x86_64-apple-darwin|linux-i686-pc-windows-gnu)
        cargo build --release --verbose --target="$TARGET"
        for x in boxxy; do
            cargo build --example "$x" --release --verbose --target="$TARGET"
        done
        ;;
    *)
        cargo build --release --verbose --target="$TARGET"
        cargo build --examples --release --verbose --target="$TARGET"
        ;;
esac
