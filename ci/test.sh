#!/bin/sh
set -ex

case "$TRAVIS_OS_NAME-$TARGET" in
    linux-x86_64-unknown-linux-gnu|osx-x86_64-apple-darwin)
        cargo test --release --verbose
        ;;
    *)
        # nop
        ;;
esac
