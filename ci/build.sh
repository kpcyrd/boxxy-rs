#!/bin/sh
set -ex

case "$TRAVIS_OS_NAME-$TARGET" in
    linux-x86_64-apple-darwin)
        # cross compiling for osx is worse than windows, don't actually link a binary
        export RUSTFLAGS="-Clinker=true -Car=true"
        export PATH="$PATH:$PWD/ci/dummy"
        cargo build --release --verbose --target="$TARGET"
        cargo build --release --verbose --target="$TARGET" --no-default-features
        ;;
    *)
        cargo build --release --verbose --target="$TARGET"
        cargo build --release --verbose --target="$TARGET" --no-default-features
        cargo build --examples --release --verbose --target="$TARGET"
        ;;
esac
